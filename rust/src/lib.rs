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
    execution: Option<Vec<usize>>,
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
}

impl ParChooser<'_> {
    pub fn new(threadno: usize, ch: &Sender<WorkerToMain>, execution: Vec<usize>) -> ParChooser {
        return ParChooser {
            threadno: threadno,
            ch: ch,
            new_choices: Vec::new(),
            pre_chosen: execution,
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
            // let tno = self.threadno;
            // println!("Thread {tno} sending new execution");
            // CAUTION: if the send fails here, we currently don't care
            let _send_result = self.ch.send(WorkerToMain {
                threadno: self.threadno,
                command: WorkerCommandType::Put,
                execution: Option::Some(new_exec),
            });
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
                execution: Option::None,
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
            threadno: threadno,
            command: WorkerCommandType::Get,
            execution: Option::None,
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
                f(&mut ParChooser::new(
                    threadno,
                    &maintx,
                    command.execution.unwrap(),
                ));
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
                            Result::Err(x) => {
                                println!("MAIN: ERROR got error {x}");
                            }
                        }
                    }
                }
            }
            // They're giving us a new execution
            WorkerCommandType::Put => {
                // println!("MAIN: thread giving an execution");
                let mut sent = false;
                let newexecution = message.execution.unwrap();

                // Are any threads waiting for one? give it to them
                for i in 0..numthreads {
                    if waiting[i] {
                        // println!("MAIN: giving execution to waiting thread");
                        (*threadchans)[i]
                            .send(MainToWorker {
                                gostop: MainCommandType::Go,
                                execution: Option::Some(newexecution.clone()),
                            })
                            .unwrap();
                        sent = true;
                        waiting[i] = false;
                        busy[i] = true;
                        break;
                    }
                }
                // No waiting threads? throw it into the executions vector
                if !sent {
                    // println!("MAIN: no waiting threads, queueing it up");
                    executions.push(newexecution.to_vec());
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
