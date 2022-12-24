// a lock-free implementation
use std::marker::{Copy, Send};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

pub struct Chooser<'a> {
    new_choices: Vec<usize>,
    pre_chosen: Vec<usize>,
    index: usize,
    newexecutions: Vec<Vec<usize>>,
    timetodie: &'a Arc<AtomicBool>,
}

impl Chooser<'_> {
    pub fn new(execution: Vec<usize>, timetodie: &Arc<AtomicBool>) -> Chooser {
        return Chooser {
            new_choices: Vec::new(),
            pre_chosen: execution,
            index: 0,
            newexecutions: Vec::new(),
            timetodie,
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

pub fn run_choices<'a, F>(f: F, numthreads: usize)
where
    F: FnMut(&mut Chooser) + Send + Copy,
{
    thread::scope(|s| {
        // The number of threads that are busy doing things, and sum of all things in queues
        let busy_main = Arc::new(AtomicUsize::new(1));
        // The ends of the channels
        let mut senders_main = Vec::with_capacity(numthreads);
        let mut receivers_main = Vec::with_capacity(numthreads);

        for i in 0..numthreads {
            let (sender, receiver) = channel();
            if i == 0 {
                match sender.send(Vec::new()) {
                    Result::Err(x) => {
                        println!("FAILED SENDING KICKOFF {x}");
                    }
                    Result::Ok(_x) => {
                        // println!("SUCCEEDED SENDING KICKOFF");
                    }
                }
            }
            senders_main.push(sender);
            receivers_main.push(receiver);
        }
        // Bail early!
        let timetodie_main = Arc::new(AtomicBool::new(false));
        let mut worker_handles = Vec::new();

        for threadno in 0..numthreads {
            let busy = busy_main.clone();
            let timetodie = timetodie_main.clone();
            let receiver = receivers_main.pop().unwrap();
            let senders = senders_main.clone();
            worker_handles.push(s.spawn(move || {
                let mut f = f;
                let timeout = std::time::Duration::from_micros(1);
                let mut next_send_to = (threadno + 1) % numthreads;
                loop {
                    // if time to drop dead, die
                    if timetodie.load(Ordering::Acquire) {
                        break;
                    }
                    let execution_res = receiver.recv_timeout(timeout);
                    match execution_res {
                        Result::Ok(execution) => {
                            let mut chooser = Chooser::new(execution, &timetodie);
                            f(&mut chooser);

                            for ne in chooser.newexecutions.into_iter() {
                                senders[next_send_to].send(ne);
                                busy.fetch_add(1, Ordering::AcqRel);

                                next_send_to = (next_send_to + 1) % numthreads;
                            }
                            busy.fetch_sub(1, Ordering::AcqRel);
                        }
                        Result::Err(_timeout) => {
                            // nothing in our queue, any threads busy?
                            let busy_threads = busy.load(Ordering::Acquire);
                            if busy_threads == 0 {
                                break;
                            }
                        }
                    }
                }
                drop(senders);
            }));
        }
        // seems like this shouldn't be needed, but if not, we have sadness.
        drop(senders_main);
    })
}

