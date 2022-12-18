extern crate chooser;

mod sudoku;

fn count_in_binary(c: &mut chooser::ParChooser) {
    let v = vec![0, 1];
    println!("X: {0} {1} {2} {3}", c.choose(&v), c.choose(&v), c.choose(&v), c.choose(&v));
}

fn magic_square(c: &mut chooser::ParChooser) {
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
    if square[0] + square[3] + square[6] != 15 || square[2] + square[4] + square[6] != 15 {
        return;
    }
    square.push(c.pick(left));
    if square[1] + square[4] + square[7] != 15 {
        return;
    }
    square.push(c.pick(left));
    if square[6] + square[7] + square[8] != 15
        || square[2] + square[5] + square[8] != 15
        || square[0] + square[4] + square[8] != 15
    {
        return;
    }

    println!("{0} {1} {2}", square[0], square[1], square[2]);
    println!("{0} {1} {2}", square[3], square[4], square[5]);
    println!("{0} {1} {2}", square[6], square[7], square[8]);
    println!("");
    // c.stop(); //stop at first solution
}
fn main() {
    chooser::run_par_choices(count_in_binary, 8);
    println!();
    chooser::run_par_choices(magic_square, 1);

    // chooser::run_choices(|c|
    sudoku::solve_faster(
        // c,
        [
            // [0,0,0, 0,0,0, 0,0,0],
            // [0,0,0, 0,0,0, 0,0,0],
            // [0,0,0, 0,0,0, 0,0,0],

            // [0,0,0, 0,0,0, 0,0,0],
            // [0,0,1, 0,0,0, 0,0,0],
            // [0,0,0, 0,0,0, 2,0,0],

            // [0,0,0, 0,0,0, 0,0,0],
            // [0,0,0, 0,0,0, 0,0,0],
            // [0,0,0, 0,0,0, 0,0,0]
        [0, 3, 0, 6, 0, 0, 0, 8, 0],
            [0, 0, 9, 8, 0, 1, 7, 0, 2],
            [0, 0, 0, 5, 0, 0, 0, 0, 6],
            [0, 0, 0, 0, 1, 0, 0, 0, 3],
            [0, 8, 5, 0, 0, 0, 9, 0, 4],
            [0, 7, 0, 0, 2, 0, 0, 0, 0],
            [0, 9, 0, 0, 0, 7, 0, 0, 0],
            [0, 5, 3, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 9, 0, 0, 4, 7],
        ],
    );
}
