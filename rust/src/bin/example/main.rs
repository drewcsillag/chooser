extern crate chooser;

mod sudoku;

fn count_in_binary(c: &mut chooser::bparchooser::Chooser) {
    let v = vec![0, 1];
    println!(
        "X: {0} {1} {2} {3} {4} {5}",
        c.choose(&v),
        c.choose(&v),
        c.choose(&v),
        c.choose(&v),
        c.choose(&v),
        c.choose(&v)
    );
}

fn magic_square(c: &mut chooser::bparchooser::Chooser) {
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
    // println!("BINARY");
    // chooser::bparchooser::run_choices(count_in_binary, 8);
    // println!("\nMAGIC SQUARES");
    // chooser::bparchooser::run_choices(magic_square, 8);
    println!("\nSUDOKU");
    let n = 400;

    let board = [
        [0, 3, 0, 6, 0, 0, 0, 8, 0],
        [0, 0, 9, 8, 0, 1, 7, 0, 2],
        [0, 0, 0, 5, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 1, 0, 0, 0, 3],
        [0, 8, 5, 0, 0, 0, 9, 0, 4],
        [0, 7, 0, 0, 2, 0, 0, 0, 0],
        [0, 9, 0, 0, 0, 7, 0, 0, 0],
        [0, 5, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 9, 0, 0, 4, 7],
    ];

    let st = std::time::Instant::now();
    for _i in 1..n {
        sudoku::solve_faster_lf(board);
    }
    let el = st.elapsed().as_nanos() / n;
    println!("LF elapsed {el} nanos");

    // let st = std::time::Instant::now();
    // for _i in 1..N {
    //     sudoku::solve_faster_bp(board);
    // }
    // let el = st.elapsed().as_nanos() / N;
    // println!("BP elapsed {el} nanos");

    // let st = std::time::Instant::now();
    // for _i in 1..N {
    //     sudoku::solve_faster_par(board);
    // }
    // let el = st.elapsed().as_nanos() / N;
    // println!("PA elapsed {el} nanos");

    let st = std::time::Instant::now();
    for _i in 1..n {
        sudoku::solve_faster(board);
    }
    let el = st.elapsed().as_nanos() / n;
    println!("ST elapsed {el} nanos");
}
