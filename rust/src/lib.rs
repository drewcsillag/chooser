#[cfg(test)]

mod tests {
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

        fn choose_index(&mut self, num_items: usize) -> usize {
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

    fn run_choices(f: fn(&mut Chooser)) {
        let mut executions = vec![vec![]];
        while let Some(execution) = executions.pop() {
            f(&mut Chooser::new(&mut executions, execution));
        }
    }

    fn count_in_binary(c: &mut Chooser) {
        let v = vec![0, 1];
        println!("{0} {1} {2}", c.choose(&v), c.choose(&v), c.choose(&v));
    }

    fn magic_square(c: &mut Chooser) {
        let left = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut square = vec![];
        square.push(c.pick(left));
        square.push(c.pick(left));
        square.push(c.pick(left));
        if square[0] + square[1] + square[2] != 15 {
            return;
        }
        square.push(c.pick(left));
        square.push(c.pick(left));
        square.push(c.pick(left));
        if square[3] + square[4] + square[5] != 15 {
            return;
        }

        square.push(c.pick(left));
        if square[0] + square[3] + square[6] != 15
            || square[2] + square[4] + square[6] != 15 {
            return
        }
        square.push(c.pick(left));
        if square[1] + square[4] + square[7] != 15 {
            return
        }
        square.push(c.pick(left));
        if square[6] + square[7] + square[8] != 15
            || square[2] + square[5] + square[8] != 15
            || square[0] + square[4] + square[8] != 15 {
            return;
        }

        println!("{0} {1} {2}", square[0], square[1], square[2]);
        println!("{0} {1} {2}", square[3], square[4], square[5]);
        println!("{0} {1} {2}", square[6], square[7], square[8]);
        println!("");
        c.stop(); //stop at first solution
    }

    #[test]
    fn it_works() {
        run_choices(count_in_binary);
        println!();
        run_choices(magic_square);
        assert_eq!(2 + 2, 3);
    }
}
