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

    display_board(square);
    // c.stop(); //stop at first solution
}

fn display_board(square: Vec<i32>) {
    println!("{0} {1} {2}", square[0], square[1], square[2]);
    println!("{0} {1} {2}", square[3], square[4], square[5]);
    println!("{0} {1} {2}", square[6], square[7], square[8]);
    println!("");
}
fn main() {
    // println!("BINARY");
    // chooser::bparchooser::run_choices(count_in_binary, 8);
    // println!("\nMAGIC SQUARES");
    // chooser::bparchooser::run_choices(magic_square, 8);
    // println!("\nSUDOKUs");
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
    sudoku::solve_faster_lf(board);
    println!("MAZE");

    let maze = vec![
        //0   1   2   3   4   5   6   7   8  9   10  11
        vec!['X','X','X','X','X','X','X','X','X','X','X','X'], //0
        vec!['X',' ',' ',' ','X',' ',' ',' ',' ',' ',' ','>'], //1
        vec!['X',' ','X','X','X','X','X',' ','X','X','X','X'], //2
        vec!['X',' ','X',' ',' ',' ','X',' ','X',' ',' ','X'], //3
        vec!['X',' ',' ',' ','X',' ',' ',' ','X',' ','X','X'], //4 
        vec!['X','X',' ','X',' ',' ','X',' ',' ',' ','X','X'], //5
        vec!['X',' ','X','X','X','X','X','X','X',' ',' ','X'], //6
        vec!['X',' ','X',' ',' ','X',' ',' ','X','X',' ','X'], //7
        vec!['X',' ','X','X',' ','X','X',' ',' ',' ',' ','X'], //8
        vec!['X',' ','X','X',' ',' ',' ',' ','X',' ',' ','X'], //9
        vec!['X',' ',' ',' ',' ','X',' ','X','X','X',' ','X'], //10
        vec!['X','^','X','X','X','X','X','X','X','X','X','X'], //11

    ];
    chooser::run_choices(move |c| maze_solve(c, &maze));
}

fn find_start(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            if maze[row][col] == '^' {
                return (row, col);
            }
        }
    }
    return (0, 0);
}

fn maze_solve(ch: &mut chooser::Chooser, omaze: &Vec<Vec<char>>)  {
    println!("SOLVE");
    let (mut r, mut c) = find_start(&omaze);

    let mut maze = omaze.to_owned();

    loop {
        println!("current {0}, {1}", r, c);
        let candidates = find_maze_candidates(&maze, r, c);
        if candidates.is_empty() {
            println!("FAIL");
            printboard(&maze);
            return;
        }
        (r, c) = *ch.choose(&candidates);

        if maze[r][c] == '>' {
            printboard(omaze);
            println!("solved");
            printboard(&maze);
            ch.stop();
            return;
        }
        maze[r][c] = '*';
    }
}

fn printboard(m: &Vec<Vec<char>>) {
    for r in 0..m.len() {
        for c in 0..m[r].len() {
            print!("{0} ", m[r][c]);
        }
        println!();
    }
}

fn find_maze_candidates(maze: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut rvec = Vec::new();

    // if we support rows of different lengths, we should check that the respective
    // new r's and c's in the vec are correct.
    if r + 1 < maze.len() && (maze[r+1][c] == ' ' || maze[r+1][c] == '>') {
        rvec.push((r+1, c));
    }
    if r!=0 && (maze[r-1][c] == ' ' || maze[r-1][c] == '>') {
        rvec.push((r-1, c))
    }
    if c+1 < maze[0].len() && (maze[r][c+1] == ' ' || maze[r][c+1] == '>') {
        rvec.push((r,c+1));
    }
    if c != 0 && (maze[r][c-1] == ' ' || maze[r][c-1] == '>') {
        let x = (r, c-1);
        rvec.push(x);
    }
    return rvec;
}
