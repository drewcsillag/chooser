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
