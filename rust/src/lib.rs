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

use std::sync::mpsc::{channel, Sender};
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

pub fn run_par_choices<F: 'static>(mut f: F, numthreads: usize)
where
    F: FnMut(&mut ParChooser) + std::marker::Send + Copy,
{
    let mut request = Vec::new();
    let mut busy: Vec<bool> = Vec::new();
    for _ in 0..numthreads {
        request.push(false);
        busy.push(false);
    }
    // let l = request.len();
    // println!("request is {l} long");
    let (maintx, mainrx) = channel();

    let mut threadchans = Vec::new();

    // WORKERS
    for threadno in 0..numthreads {
        // println!("thread {threadno} starting");
        let (tx, rx) = channel();
        threadchans.push(tx);
        let maintx = maintx.clone();

        thread::spawn(move || {
            loop {
                // println!("Thread {threadno} getting work");
                maintx
                    .send(WorkerToMain {
                        threadno: threadno,
                        putget: WorkerGetPut::Get,
                        execution: Option::None,
                    })
                    .unwrap();
                let command: MainToWorker = rx.recv().unwrap();
                match command.gostop {
                    WorkerGoStop::Stop => {
                        // println!("Thread {threadno} got stop");
                        break;
                    }
                    WorkerGoStop::Go => {
                        // println!("Thread {threadno} got a Go");
                        f(&mut ParChooser::new(
                            threadno,
                            &maintx,
                            command.execution.unwrap(),
                        ));
                    }
                }
            }
        });
    }

    // println!("starting main bit");
    // Main
    let mut executions: Vec<Vec<usize>> = vec![vec![]];
    loop {
        // println!("Main getting message");
        let message = mainrx.recv().unwrap().clone();

        match message.putget {
            WorkerGetPut::Stop => {
                break;
            }
            WorkerGetPut::Get => {
                // let tno = message.threadno;
                // println!("Main got Get message from thread {tno}");
                let execution = executions.pop();
                match execution {
                    None => {
                        // println!("Main, no message to be given to thread {tno}");
                        request[message.threadno] = true;
                        busy[message.threadno] = false;
                    }
                    Some(value) => {
                        // println!("Main, WE HAVE message to be given to thread {tno}");
                        busy[message.threadno] = true;
                        threadchans[message.threadno]
                            .send(MainToWorker {
                                gostop: WorkerGoStop::Go,
                                execution: Option::Some(value),
                            })
                            .unwrap();
                    }
                }
            }
            WorkerGetPut::Put => {
                // let tno = message.threadno;
                // println!("Main got Put message from thread {tno}");
                let mut sent = false;
                let v = message.execution.unwrap();
                let nopt = Option::Some(v.clone());

                for i in 0..numthreads {
                    if request[i] {
                        threadchans[i]
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
                if !sent {
                    executions.push(v.to_vec());
                }
            }
        }

        let eie = executions.is_empty();
        // println!("Executions empty? {eie}");
        if !eie {
            continue;
        }
        let mut waiting = false;
        for i in 0..numthreads {
            if busy[i] {
                waiting = true;
                break;
            }
        }
        // println!("waiting threads? {waiting}");
        if !waiting {
            break;
        }
    }

    for i in 0..numthreads {
        threadchans[i]
            .send(MainToWorker {
                gostop: WorkerGoStop::Stop,
                execution: Option::None,
            })
            .unwrap();
    }
}
