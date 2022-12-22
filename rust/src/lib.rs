// Single threaded implementationm
pub struct Chooser<'a> {
    new_choices: Vec<usize>,
    pre_chosen: Vec<usize>,
    index: usize,
    executions: &'a mut Vec<Vec<usize>>,
}

impl Chooser<'_> {
    pub fn new(executions: &mut Vec<Vec<usize>>, execution: Vec<usize>) -> Chooser {
        return Chooser {
            new_choices: Vec::new(),
            pre_chosen: execution,
            executions,
            index: 0,
        };
    }

    pub fn choose_index(&mut self, num_items: usize) -> usize {
        if self.index < self.pre_chosen.len() {
            let ret = self.pre_chosen[self.index];
            self.index = self.index + 1;
            return ret;
        }
        for choice in 1..num_items {
            let mut new_exec = self.pre_chosen.to_vec().to_owned();
            new_exec.append(&mut self.new_choices.to_owned());
            new_exec.push(choice);
            self.executions.push(new_exec);
        }
        self.new_choices.push(0);

        return 0;
    }

    pub fn choose<'a, T>(&mut self, choices: &'a Vec<T>) -> &'a T {
        return &choices[self.choose_index(choices.len())];
    }

    pub fn pick<T>(&mut self, choices: &mut Vec<T>) -> T {
        return choices.remove(self.choose_index(choices.len()));
    }

    pub fn stop(&mut self) {
        self.executions.clear();
    }
}

pub fn run_choices<F>(mut f: F)
where
    F: FnMut(&mut Chooser),
{
    let mut executions = vec![vec![]];
    while let Some(execution) = executions.pop() {
        f(&mut Chooser::new(&mut executions, execution));
    }
}

// Parallel implementation
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Copy, Clone)]
enum WorkerCommandType {
    Get,
    Put,
    Stop,
}

#[derive(Copy, Clone)]
enum MainCommandType {
    Go,
    Stop,
}

pub struct WorkerToMain {
    threadno: usize,
    command: WorkerCommandType,
    executions: Option<Vec<Vec<usize>>>,
}

pub struct MainToWorker {
    gostop: MainCommandType,
    execution: Option<Vec<usize>>,
}

pub struct ParChooser<'a> {
    threadno: usize,
    ch: &'a Sender<WorkerToMain>,
    new_choices: Vec<usize>,
    pre_chosen: Vec<usize>,
    index: usize,
    newexecutions: Vec<Vec<usize>>,
}

impl ParChooser<'_> {
    pub fn new(threadno: usize, ch: &Sender<WorkerToMain>, execution: Vec<usize>) -> ParChooser {
        return ParChooser {
            threadno,
            ch,
            new_choices: Vec::new(),
            pre_chosen: execution,
            index: 0,
            newexecutions: Vec::new(),
        };
    }

    pub fn choose_index(&mut self, num_items: usize) -> usize {
        if self.index < self.pre_chosen.len() {
            let ret = self.pre_chosen[self.index];
            self.index = self.index + 1;
            return ret;
        }
        for choice in 1..num_items {
            let mut new_exec = self.pre_chosen.to_vec().to_owned();
            new_exec.append(&mut self.new_choices.to_owned());
            new_exec.push(choice);
            self.newexecutions.push(new_exec);
        }
        self.new_choices.push(0);

        return 0;
    }

    pub fn choose<'a, T>(&mut self, choices: &'a Vec<T>) -> &'a T {
        return &choices[self.choose_index(choices.len())];
    }

    pub fn pick<T>(&mut self, choices: &mut Vec<T>) -> T {
        return choices.remove(self.choose_index(choices.len()));
    }

    pub fn stop(&mut self) {
        self.ch
            .send(WorkerToMain {
                threadno: self.threadno,
                command: WorkerCommandType::Stop,
                executions: Option::None,
            })
            .unwrap();
    }
}

pub fn run_par_choices<'a, F>(f: F, numthreads: usize)
where
    F: FnMut(&mut ParChooser) + std::marker::Send + Copy,
{
    thread::scope(|s| {
        let (maintx, mainrx) = channel();
        let mut threadchans = Vec::new();

        let mut worker_handles = Vec::new();

        // Kick off workers
        for threadno in 0..numthreads {
            let (tx, rx) = channel();
            threadchans.push(tx);
            // gets snarfed into the thread spawn below
            let maintx = maintx.clone();

            worker_handles.push(s.spawn(move || {
                worker_thread(maintx, threadno, rx, f);
            }));
        }

        // Kick off main (btw: moves threadchans)
        let main_handle = thread::spawn(move || {
            main_thread(mainrx, &threadchans, numthreads);
        });

        // print!("Waiting for worker handles");
        loop {
            let h = worker_handles.pop();
            match h {
                Option::Some(h) => {
                    let _r = h.join();
                }
                Option::None => {
                    break;
                }
            }
        }

        // Join main thread
        let _mhj = main_handle.join();
    });
}

fn worker_thread<F>(
    maintx: Sender<WorkerToMain>,
    threadno: usize,
    rx: Receiver<MainToWorker>,
    mut f: F,
) where
    F: FnMut(&mut ParChooser) + std::marker::Send,
{
    loop {
        // CAVEAT ignoring failure
        // Tell main thread to give us something
        let _result = maintx.send(WorkerToMain {
            threadno,
            command: WorkerCommandType::Get,
            executions: Option::None,
        });

        // Get a command from the main thread
        let command: MainToWorker = rx.recv().unwrap();
        match command.gostop {
            // Main thread told us to stop
            MainCommandType::Stop => {
                break;
            }
            // we got a chunk of work to do
            MainCommandType::Go => {
                let mut pc = ParChooser::new(threadno, &maintx, command.execution.unwrap());
                f(&mut pc);
                if !pc.newexecutions.is_empty() {
                    match maintx.send(WorkerToMain {
                        threadno,
                        command: WorkerCommandType::Put,
                        executions: Option::Some(pc.newexecutions),
                    }) {
                        Result::Ok(_whatever) => {}
                        Result::Err(_x) => {
                            // println!("FAILED SENDING TO MAIN {x}");
                        }
                    }
                }
            }
        }
    }
}

fn main_thread(
    mainrx: Receiver<WorkerToMain>,
    threadchans: &Vec<Sender<MainToWorker>>,
    numthreads: usize,
) {
    // The executions list
    let mut executions: Vec<Vec<usize>> = vec![vec![]];
    // threads that are waiting for a chunk
    let mut waiting = Vec::new();

    // threads that are processing a chunk
    let mut busy: Vec<bool> = Vec::new();

    for _i in 0..numthreads {
        busy.push(false);
        waiting.push(false);
    }

    loop {
        // get a message from a worker thread
        let message = mainrx.recv().unwrap();
        // let tno = message.threadno;
        // println!("MAIN: got a message from thread {tno}");
        match message.command {
            // stop the presses! break out of the loop
            WorkerCommandType::Stop => {
                // println!("MAIN: worker said STOP!");
                break;
            }
            // they want something to do
            WorkerCommandType::Get => {
                // println!("MAIN: asked for something to do");
                // try to get an execution
                let execution = executions.pop();
                match execution {
                    // No executions queued, so we're not busy, we'll leave them waiting on a request
                    None => {
                        waiting[message.threadno] = true;
                        busy[message.threadno] = false;
                        // println!("MAIN: queue is empty, they'll have to wait");
                    }
                    // There was an execution, send it to the worker (and it's busy).
                    Some(value) => {
                        busy[message.threadno] = true;
                        // println!("MAIN: giving them something to do");
                        let result = (*threadchans)[message.threadno].send(MainToWorker {
                            gostop: MainCommandType::Go,
                            execution: Option::Some(value),
                        });
                        match result {
                            Result::Ok(_x) => {}
                            Result::Err(_x) => {
                                // println!("MAIN: ERROR got error {x}");
                            }
                        }
                    }
                }
            }
            // They're giving us a new execution
            WorkerCommandType::Put => {
                let mut newexecutions = message.executions.unwrap();
                // let ne = newexecutions.len();
                // let tno = message.threadno;
                // println!("MAIN: thread giving an execution {tno} : {ne}");

                // Are any threads waiting for one? give it to them
                for i in 0..numthreads {
                    if waiting[i] && !newexecutions.is_empty() {
                        // println!("MAIN: giving execution to waiting thread {i}");
                        (*threadchans)[i]
                            .send(MainToWorker {
                                gostop: MainCommandType::Go,
                                execution: Option::Some(newexecutions.pop().unwrap()),
                            })
                            .unwrap();
                        waiting[i] = false;
                        busy[i] = true;
                    }
                }
                // let ie = newexecutions.is_empty();
                // No waiting threads? throw it into the executions vector
                if !newexecutions.is_empty() {
                    // println!("MAIN: no waiting threads, queueing it up");
                    executions.extend(newexecutions.into_iter());
                }
            }
        }

        // Command processed, is it time to be done?

        // If there are more things to execute, clearly not done.
        if !executions.is_empty() {
            continue;
        }

        // Are any threads busy? then we're not done.
        let mut anybusy = false;
        for i in 0..numthreads {
            if busy[i] {
                anybusy = true;
                break;
            }
        }
        if !anybusy {
            // println!("MAIN: we think we're done, exiting");
            break;
        }
    }
    // Tell all the worker threads to shut down
    for i in 0..numthreads {
        threadchans[i]
            .send(MainToWorker {
                gostop: MainCommandType::Stop,
                execution: Option::None,
            })
            .unwrap();
    }
}

use crossbeam::atomic::AtomicCell;
use std::hint;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
pub struct FastParChooser<'a> {
    threadno: usize,
    new_choices: Vec<usize>,
    pre_chosen: Vec<usize>,
    index: usize,
    newexecutions: Vec<Vec<usize>>,
    timetodie: &'a Arc<AtomicBool>,
}

impl FastParChooser<'_> {
    pub fn new(
        threadno: usize,
        execution: Vec<usize>,
        timetodie: &Arc<AtomicBool>,
    ) -> FastParChooser {
        return FastParChooser {
            threadno,
            new_choices: Vec::new(),
            pre_chosen: execution,
            index: 0,
            newexecutions: Vec::new(),
            timetodie: timetodie,
        };
    }

    pub fn choose_index(&mut self, num_items: usize) -> usize {
        if self.index < self.pre_chosen.len() {
            let ret = self.pre_chosen[self.index];
            self.index = self.index + 1;
            return ret;
        }
        for choice in 1..num_items {
            let mut new_exec = self.pre_chosen.to_vec().to_owned();
            new_exec.append(&mut self.new_choices.to_owned());
            new_exec.push(choice);
            self.newexecutions.push(new_exec);
        }
        self.new_choices.push(0);

        return 0;
    }

    pub fn choose<'a, T>(&mut self, choices: &'a Vec<T>) -> &'a T {
        return &choices[self.choose_index(choices.len())];
    }

    pub fn pick<T>(&mut self, choices: &mut Vec<T>) -> T {
        return choices.remove(self.choose_index(choices.len()));
    }

    pub fn stop(&mut self) {
        self.timetodie.store(true, Ordering::SeqCst)
    }
}

pub fn run_fast_par_choices<'a, F>(mut f: F, numthreads: usize)
where
    F: FnMut(&mut FastParChooser) + std::marker::Send + Copy,
{
    thread::scope(|s| {
        // The number of threads that are busy doing things
        let busy_main = Arc::new(AtomicUsize::new(0));

        // the boolean to make the spinlock with
        let spin_lock_main = Arc::new(AtomicBool::new(false));
        // The executions list
        let raw_exec = vec![vec![]];
        let executions_cell_main = Arc::new(crossbeam::atomic::AtomicCell::new(raw_exec));

        // Bail early!
        let timetodie_main = Arc::new(AtomicBool::new(false));

        let mut worker_handles = Vec::new();

        for threadno in 0..numthreads {
            let spin_lock = spin_lock_main.clone();
            let busy = busy_main.clone();
            let executions_cell = executions_cell_main.clone();
            let timetodie = timetodie_main.clone();

            worker_handles.push(s.spawn(move || {
                fast_worker_thread(spin_lock, executions_cell, busy, threadno, timetodie, f);
            }));
        }

        for handle in worker_handles.into_iter() {
            let _x = handle.join();
        }
    })
}

fn fast_worker_thread<F>(
    spin_lock: Arc<AtomicBool>,
    executions_cell: Arc<AtomicCell<Vec<Vec<usize>>>>,
    busy: Arc<AtomicUsize>,
    threadno: usize,
    timetodie: Arc<AtomicBool>,
    mut f: F,
) where
    F: FnMut(&mut FastParChooser) + std::marker::Send + Copy,
{
    loop {
        // ----- spinlock to get access to executions
        wait_on_spin_lock(&spin_lock);

        let execution = pop_execution(&executions_cell);

        match execution {
            Option::Some(execution) => {
                // we're busy - have to do this inside, because better to say we're busier
                // than we might be (compared to where we decrement it ouside the spinlock)
                // lest we bail out prematurely
                busy.fetch_add(1, Ordering::Acquire);
                spin_lock.store(false, Ordering::Release);
                // ----- end of spinlock protected area

                // OPTIMIZATION NOTE: could probably make a single mutable one and reset it...
                let mut parc = FastParChooser::new(threadno, execution, &timetodie);
                f(&mut parc);

                // if time to drop dead, die
                if timetodie.load(Ordering::Acquire) {
                    break;
                }

                // ----- spinlock to get acces to executions
                wait_on_spin_lock(&spin_lock);
                extend_executions(&executions_cell, parc.newexecutions);
                spin_lock.store(false, Ordering::Release);
                // ----- end of spinlock protected area

                // we're not busy any more
                busy.fetch_sub(1, Ordering::Acquire);
            }
            Option::None => {
                spin_lock.store(false, Ordering::Release);
                // ----- end of spinlock protected area

                let num_busy_threads = busy.load(Ordering::Acquire);

                if num_busy_threads == 0 {
                    break; // all done: no executions and no one busy
                }
            }
        }
    }
}

fn extend_executions(
    executions_cell: &Arc<AtomicCell<Vec<Vec<usize>>>>,
    newexecutions: Vec<Vec<usize>>,
) {
    unsafe { (&mut *(executions_cell.as_ptr())).extend(newexecutions.into_iter()) }
}

fn pop_execution(executions_cell: &Arc<AtomicCell<Vec<Vec<usize>>>>) -> Option<Vec<usize>> {
    unsafe { (&mut *(executions_cell.as_ptr())).pop() }
}

fn wait_on_spin_lock(spinlock: &Arc<AtomicBool>) {
    loop {
        let res = spinlock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
        match res {
            Result::Ok(_) => {
                break;
            }
            Result::Err(_) => {
                hint::spin_loop();
            }
        }
    }
}
