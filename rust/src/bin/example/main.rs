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

    let maze = [
        //0   1   2   3   4   5   6   7   8  9   10  11
        ['X','X','X','X','X','X','X','X','X','X','X','X'], //0
        ['X',' ',' ',' ','X',' ',' ',' ',' ',' ',' ','>'], //1
        ['X',' ','X','X','X','X','X',' ','X','X','X','X'], //2
        ['X',' ','X',' ',' ',' ','X',' ','X',' ',' ','X'], //3
        ['X',' ',' ',' ','X',' ',' ',' ','X',' ','X','X'], //4 
        ['X','X',' ','X',' ',' ','X',' ',' ',' ','X','X'], //5
        ['X',' ','X','X','X','X','X','X','X',' ',' ','X'], //6
        ['X',' ','X',' ',' ','X',' ',' ','X','X',' ','X'], //7
        ['X',' ','X','X',' ','X','X',' ',' ',' ',' ','X'], //8
        ['X',' ','X','X',' ',' ',' ',' ','X',' ',' ','X'], //9
        ['X',' ',' ',' ',' ','X',' ','X','X','X',' ','X'], //10
        ['X','^','X','X','X','X','X','X','X','X','X','X'], //11

    ];
    chooser::run_choices(|c| maze_solve(c, maze));
}

fn find_start(maze: [[char; 12]; 12]) -> (usize, usize) {
    for row in 0..12 {
        for col in 0..12 {
            if maze[row][col] == '^' {
                return (row, col);
            }
        }
    }
    return (0, 0);
}

fn maze_solve(ch: &mut chooser::Chooser, omaze: [[char; 12]; 12])  {
    println!("SOLVE");
    let (mut r, mut c) = find_start(omaze);

    let path = vec![(r, c)];
    let mut maze = omaze.clone();

    loop {
        println!("current {0}, {1}", r, c);
        let candidates = find_maze_candidates(maze, r, c);
        if candidates.is_empty() {
            println!("FAIL");
            printboard(maze);
            return;
        }
        (r, c) = *ch.choose(&candidates);

        if maze[r][c] == '>' {
            println!("Found path!");
            for v in &path {
                println!(" {0} {1}", v.0, v.1);
            }

            printboard(omaze);
            println!("solved");
            printboard(maze);
            ch.stop();
            return;
        }
        maze[r][c] = '*';
    }
}



fn printboard(m: [[char;12];12]) {
    for r in 0..12 {
        println!("{0} {1} {2} {3} {4} {5} {6} {7} {8} {9} {10} {11}",
            m[r][0], m[r][1],  m[r][2],
            m[r][3], m[r][4],  m[r][5],
            m[r][6], m[r][7],  m[r][8],
            m[r][9], m[r][10], m[r][11]);
    }
}

fn find_maze_candidates(maze: [[char; 12]; 12], r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut rvec = Vec::new();

    if r + 1 < 12 && (maze[r+1][c] == ' ' || maze[r+1][c] == '>') {
        rvec.push((r+1, c));
    }
    if r!=0 && (maze[r-1][c] == ' ' || maze[r-1][c] == '>') {
        rvec.push((r-1, c))
    }
    if c+1 < 12 && (maze[r][c+1] == ' ' || maze[r][c+1] == '>') {
        rvec.push((r,c+1));
    }
    if c != 0 && (maze[r][c-1] == ' ' || maze[r][c-1] == '>') {
        let x = (r, c-1);
        rvec.push(x);
    }
    return rvec;
}
