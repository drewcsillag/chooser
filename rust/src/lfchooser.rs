// a lock-free implementation
use crossbeam::atomic::AtomicCell;
use std::hint;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
pub struct Chooser<'a> {
    new_choices: Vec<usize>,
    pre_chosen: Vec<usize>,
    index: usize,
    newexecutions: Vec<Vec<usize>>,
    timetodie: &'a Arc<AtomicBool>,
}

impl Chooser<'_> {
    pub fn new(
        execution: Vec<usize>,
        timetodie: &Arc<AtomicBool>,
    ) -> Chooser {
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

use std::thread;

pub fn run_choices<'a, F>(f: F, numthreads: usize)
where
    F: FnMut(&mut Chooser) + std::marker::Send + Copy,
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

        for _threadno in 0..numthreads {
            let spin_lock = spin_lock_main.clone();
            let busy = busy_main.clone();
            let executions_cell = executions_cell_main.clone();
            let timetodie = timetodie_main.clone();

            worker_handles.push(s.spawn(move || {
                fast_worker_thread(spin_lock, executions_cell, busy, timetodie, f);
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
    timetodie: Arc<AtomicBool>,
    mut f: F,
) where
    F: FnMut(&mut Chooser) + std::marker::Send + Copy,
{
    loop {
        // if time to drop dead, die
        if timetodie.load(Ordering::Acquire) {
            break;
        }

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
                let mut parc = Chooser::new(execution, &timetodie);
                f(&mut parc);

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
