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

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

enum WorkerGetPut {
    Get,
    Put,
    Stop,
}

enum WorkerGoStop {
    Go,
    Stop,
}
pub struct WorkerToMain {
    threadno: usize,
    putget: WorkerGetPut,
    execution: Option<Vec<usize>>,
}
impl WorkerToMain {
    fn clone(&self) -> Self {
        match self.putget {
            WorkerGetPut::Get => {
                return WorkerToMain {
                    threadno: self.threadno,
                    putget: WorkerGetPut::Get,
                    execution: self.execution.clone(),
                }
            }
            WorkerGetPut::Put => {
                return WorkerToMain {
                    threadno: self.threadno,
                    putget: WorkerGetPut::Put,
                    execution: self.execution.clone(),
                }
            }
            WorkerGetPut::Stop => {
                return WorkerToMain {
                    threadno: self.threadno,
                    putget: WorkerGetPut::Stop,
                    execution: self.execution.clone(),
                }
            }
        }
    }
}

pub struct MainToWorker {
    gostop: WorkerGoStop,
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
                putget: WorkerGetPut::Put,
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
                putget: WorkerGetPut::Stop,
                execution: Option::None,
            })
            .unwrap();
    }
}

pub fn run_par_choices<'a, F: 'static>(mut f: F, numthreads: usize)
where
    F: FnMut(&mut ParChooser) + std::marker::Send + Copy,
{
    let (maintx, mainrx) = channel();
    let mut threadchans = Vec::new();

    let mut worker_handles = Vec::new();
    // WORKERS
    for threadno in 0..numthreads {
        let (tx, rx) = channel();
        threadchans.push(tx);
        // gets snarfed into the thread spawn below
        let maintx = maintx.clone();

        worker_handles.push(thread::spawn(move || {
            worker_thread(maintx, threadno, rx, f);
        }));
    }

    let main_handle = thread::spawn(move || {
        main_thread(mainrx, &threadchans, numthreads);
    });

    // Tell all the threads to shut down
    for i in 0..numthreads {
        threadchans[i]
            .send(MainToWorker {
                gostop: WorkerGoStop::Stop,
                execution: Option::None,
            })
            .unwrap();
    }

    while ! worker_handles.is_empty() {
        let h = worker_handles.pop();
        match h {
            Option::Some(h) => { let _r = h.join(); },
            Option::None => {}
        }
    }

    let _mhj = main_handle.join();
    // could probably do something with join handles here
}

fn worker_thread<F: 'static>(maintx: Sender<WorkerToMain>, threadno: usize, rx: Receiver<MainToWorker>, f: F) 
where F: FnMut(&mut ParChooser) + std::marker::Send + Copy {
    loop {
        // CAVEAT ignoring failure
        // Tell main thread to give us something
        let _result = maintx
            .send(WorkerToMain {
                threadno: threadno,
                putget: WorkerGetPut::Get,
                execution: Option::None,
            });
        
        // Get a command from the main thread
        let command: MainToWorker = rx.recv().unwrap();
        match command.gostop {
            // main thread told us to stop
            WorkerGoStop::Stop => {
                break;
            }
            // we got a chunk of work to do
            WorkerGoStop::Go => {
                f(&mut ParChooser::new(
                    threadno,
                    &maintx,
                    command.execution.unwrap(),
                ));
            }
        }
    }
}

fn main_thread(mainrx: std::sync::mpsc::Receiver<WorkerToMain>, threadchans: &Vec<Sender<MainToWorker>>, numthreads: usize) {
    // Main
    let mut executions: Vec<Vec<usize>> = vec![vec![]];
    // threads that are requesting for a chunk
    let mut request = Vec::new();

    // threads that are processing a chunk
    let mut busy: Vec<bool> = Vec::new();
    for _i in 0..numthreads {
        busy.push(false);
        request.push(false);

    }
    loop {
        // get a message from a worker thread
        let message = mainrx.recv().unwrap().clone();

        match message.putget {
            // stop the presses! break out of the loop
            WorkerGetPut::Stop => {
                break;
            }
            // they want something to do
            WorkerGetPut::Get => {
                // try to get an execution
                let execution = executions.pop();
                match execution {
                    // no execution, we're not busy, and we're waiting on a request
                    None => {
                        request[message.threadno] = true;
                        busy[message.threadno] = false;
                    }
                    // there was an execution, send it to the worker (and it's busy).
                    Some(value) => {
                        busy[message.threadno] = true;
                        (*threadchans)[message.threadno]
                            .send(MainToWorker {
                                gostop: WorkerGoStop::Go,
                                execution: Option::Some(value),
                            })
                            .unwrap();
                    }
                }
            }
            // they're giving us an execution
            WorkerGetPut::Put => {
                let mut sent = false;
                let v = message.execution.unwrap();
                let nopt = Option::Some(v.clone());

                // are any threads waiting for one? give it to them
                for i in 0..numthreads {
                    if request[i] {
                        (*threadchans)[i]
                            .send(MainToWorker {
                                gostop: WorkerGoStop::Go,
                                execution: nopt,
                            })
                            .unwrap();
                        sent = true;
                        request[i] = false;
                        busy[i] = true;
                        break;
                    }
                }
                // no waiting threads? throw it into the executions vector
                if !sent {
                    executions.push(v.to_vec());
                }
            }
        }

        // command processed, is it time to be done

        // if there are more things to execute, clearly not done.
        if !executions.is_empty() {
            continue;
        }

        // Are any threads busy? then we're not done.
        let mut waiting = false;
        for i in 0..numthreads {
            if busy[i] {
                waiting = true;
                break;
            }
        }
        if !waiting {
            break;
        }
    }
}
