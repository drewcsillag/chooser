// a lock-free implementation
use crossbeam::atomic::AtomicCell;
use std::hint;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
    F: FnMut(&mut Chooser) + std::marker::Send + Copy,
{
    thread::scope(|s| {
        // The number of threads that are busy doing things
        let busy_main = Arc::new(AtomicUsize::new(1));

        // the boolean to make the spinlock with
        let spin_lock_main = Arc::new(AtomicBool::new(false));
        // The executions list
        let executions_cell_main = Arc::new(AtomicCell::new(vec![vec![]]));

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
    let mut parc = Chooser::new(vec![], &timetodie);

    loop {
        // if time to drop dead, die
        if timetodie.load(Ordering::Acquire) {
            break;
        }

        match pop_execution(&executions_cell, &spin_lock) {
            Option::Some(execution) => {
                parc.new_choices.clear();
                parc.pre_chosen = execution;
                parc.index = 0;
                f(&mut parc);

                let num_execs = parc.newexecutions.len();
                if num_execs == 0 {
                    // decrement the one we just finished processing
                    busy.fetch_sub(1, Ordering::Acquire);
                    continue;
                }
                extend_executions(&executions_cell, &mut parc.newexecutions, &spin_lock);

                busy.fetch_add(num_execs - 1, Ordering::Acquire);
            }
            Option::None => {
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
    newexecutions: &mut Vec<Vec<usize>>,
    spin_lock: &Arc<AtomicBool>,
) {
    wait_on_spin_lock(&spin_lock);

    unsafe {
        (&mut *(executions_cell.as_ptr())).append(newexecutions);
    }
    spin_lock.store(false, Ordering::Release);
}

fn pop_execution(
    executions_cell: &Arc<AtomicCell<Vec<Vec<usize>>>>,
    spin_lock: &Arc<AtomicBool>,
) -> Option<Vec<usize>> {
    wait_on_spin_lock(&spin_lock);

    let ret = unsafe { (&mut *(executions_cell.as_ptr())).pop() };
    spin_lock.store(false, Ordering::Release);
    ret
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
