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
        // println!("M: starting scope");
        // The number of threads that are busy doing things + items in the executions
        let busy_main = Arc::new(AtomicUsize::new(0));
        busy_main.fetch_add(1, Ordering::AcqRel);

        // the boolean to make the spinlock with
        let mut spin_locks = Vec::new();
        for _i in 0..numthreads {
            spin_locks.push(AtomicBool::new(false));
        }
        let spin_locks_main = Arc::new(spin_locks);
        // The executions list
        let mut raw_execs = Vec::new();
        for i in 0..numthreads {
            if i == 0 {
                raw_execs.push(vec![vec![]]);
            } else {
                raw_execs.push(Vec::new());
            }
        }
        let mut execution_cells = Vec::new();
        for _i in 0..numthreads {
            execution_cells.push(crossbeam::atomic::AtomicCell::new(raw_execs.pop().unwrap()));
        }
        let execution_cells_main = Arc::new(execution_cells);

        // Bail early!
        let timetodie_main = Arc::new(AtomicBool::new(false));

        let mut worker_handles = Vec::new();

        for threadno in 0..numthreads {
            let spin_locks = spin_locks_main.clone();
            let busy = busy_main.clone();
            let executions_cells = execution_cells_main.clone();
            let timetodie = timetodie_main.clone();

            worker_handles.push(s.spawn(move || {
                // println!("{threadno}: starting");
                fast_worker_thread(threadno, numthreads, spin_locks.into(), executions_cells, busy, timetodie, f);
            }));
        }

        for handle in worker_handles.into_iter() {
            let _x = handle.join();
        }
    })
}

fn fast_worker_thread<F>(
    threadno: usize,
    numthreads: usize,
    spin_locks: Arc<Vec<AtomicBool>>,
    executions_cells: Arc<Vec<AtomicCell<Vec<Vec<usize>>>>>,
    busy: Arc<AtomicUsize>,
    timetodie: Arc<AtomicBool>,
    mut f: F,
) where
    F: FnMut(&mut Chooser) + std::marker::Send + Copy,
{
    let mut next_e = (threadno + 1) % numthreads;
    loop {
        std::hint::spin_loop();

        // if time to drop dead, die
        if timetodie.load(Ordering::Acquire) {
            // println!("STOP!");
            break;
        }
        
        // ----- spinlock to get access to executions
        wait_on_spin_lock(&spin_locks[threadno]);
        let execution = pop_execution(&executions_cells[threadno]);
        spin_locks[threadno].store(false, Ordering::Release);
        // ----- end of spinlock protected area

        match execution {
            Option::Some(execution) => {
                // println!("{threadno}: got an execution");
                // OPTIMIZATION NOTE: could probably make a single mutable one and reset it...
                let mut parc = Chooser::new(execution, &timetodie);
                f(&mut parc);

                busy.fetch_add(parc.newexecutions.len(), Ordering::Acquire);
                // ----- spinlock to get acces to executions
                wait_on_spin_lock(&spin_locks[next_e]);
                extend_executions(&executions_cells[next_e], parc.newexecutions);
                spin_locks[next_e].store(false, Ordering::Release);
                // ----- end of spinlock protected area

                next_e = (next_e + 1) % numthreads; 

                busy.fetch_sub(1, Ordering::Acquire);
            }
            Option::None => {
                let num_busy = busy.load(Ordering::Acquire);

                if num_busy == 0 {
                    break; // all done: no executions and no one busy
                }
            }
        }
    }
}

// fn append_execution(
//     executions_cell: &AtomicCell<Vec<Vec<usize>>>,
//     newexecution: Vec<usize>,
// ) {
//     unsafe { (&mut *(executions_cell.as_ptr())).push(newexecution) }
// }

fn pop_execution(executions_cell: &AtomicCell<Vec<Vec<usize>>>) -> Option<Vec<usize>> {
    unsafe { (&mut *(executions_cell.as_ptr())).pop() }
}

fn extend_executions(
    executions_cell: &AtomicCell<Vec<Vec<usize>>>,
    newexecutions: Vec<Vec<usize>>,
) {
    unsafe { (&mut *(executions_cell.as_ptr())).extend(newexecutions.into_iter()) }
}

fn wait_on_spin_lock(spinlock: &AtomicBool) {
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
