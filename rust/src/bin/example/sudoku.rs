use chooser;
use std::collections::HashSet;

const BOX_INDEXES: [[usize; 9]; 9] = [
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
];

#[derive(Clone)]
struct Indexes {
    rows: Vec<HashSet<u8>>,
    cols: Vec<HashSet<u8>>,
    boxes: Vec<HashSet<u8>>,
    board: [[u8; 9]; 9],
}

fn index_board(board: [[u8; 9]; 9]) -> Indexes {
    let mut result = Indexes {
        rows: (0..9).map(|_| HashSet::new()).collect(),
        cols: (0..9).map(|_| HashSet::new()).collect(),
        boxes: (0..9).map(|_| HashSet::new()).collect(),
        board,
    };
    (0..9).for_each(|row| {
        (0..9).for_each(|col| {
            let cell = board[row][col];
            if cell > 0 {
                result.rows[row].insert(cell);
                result.cols[col].insert(cell);
                result.boxes[BOX_INDEXES[row][col]].insert(cell);
            }
        })
    });
    return result;
}

fn add_cell(indexes: &mut Indexes, row: usize, col: usize, cell: u8) {
    indexes.rows[row].insert(cell);
    indexes.cols[col].insert(cell);
    indexes.boxes[BOX_INDEXES[row][col]].insert(cell);
    indexes.board[row][col] = cell;
}

fn candidates(indexes: &Indexes, row: usize, col: usize) -> Vec<u8> {
    return (1..10)
        .filter(|i| {
            !indexes.rows[row].contains(i)
                && !indexes.cols[col].contains(i)
                && !indexes.boxes[BOX_INDEXES[row][col]].contains(i)
        })
        .collect();
}


pub fn solve_faster(board: [[u8; 9]; 9]) {
    let init_index = &index_board(board);
    chooser::run_choices(
        |c| {
            let indexes: &mut Indexes = &mut init_index.clone();
            for row in 0..9 {
                for col in 0..9 {
                    if indexes.board[row][col] != 0 {
                        continue;
                    }
                    let cand = candidates(&indexes, row, col);
                    if cand.len() == 0 {
                        return;
                    }
                    add_cell(indexes, row, col, *c.choose(&cand));
                }
            }
            // for row in 0..9 {
            //     println!("{:?}", indexes.board[row]);
            // }
            c.stop();
        }
    )
}

pub fn solve_faster_par(board: [[u8; 9]; 9]) {
    let init_index = &index_board(board);
    chooser::parchooser::run_choices(
        |c| {
            let indexes: &mut Indexes = &mut init_index.clone();
            for row in 0..9 {
                for col in 0..9 {
                    if indexes.board[row][col] != 0 {
                        continue;
                    }
                    let cand = candidates(&indexes, row, col);
                    if cand.len() == 0 {
                        return;
                    }
                    add_cell(indexes, row, col, *c.choose(&cand));
                }
            }
            // for row in 0..9 {
            //     println!("{:?}", indexes.board[row]);
            // }
            c.stop();
        },
        10,
    )
}

pub fn solve_faster_bp(board: [[u8; 9]; 9]) {
    let init_index = &index_board(board);
    chooser::bparchooser::run_choices(
        |c| {
            let indexes: &mut Indexes = &mut init_index.clone();
            for row in 0..9 {
                for col in 0..9 {
                    if indexes.board[row][col] != 0 {
                        continue;
                    }
                    let cand = candidates(&indexes, row, col);
                    if cand.len() == 0 {
                        return;
                    }
                    add_cell(indexes, row, col, *c.choose(&cand));
                }
            }
            // for row in 0..9 {
            //     println!("{:?}", indexes.board[row]);
            // }
            c.stop();
        },
        10,
    )
}

pub fn solve_faster_lf(board: [[u8; 9]; 9]) {
    let init_index = &index_board(board);
    chooser::lfchooser::run_choices(
        |c| {
            let indexes: &mut Indexes = &mut init_index.clone();
            for row in 0..9 {
                for col in 0..9 {
                    if indexes.board[row][col] != 0 {
                        continue;
                    }
                    let cand = candidates(&indexes, row, col);
                    if cand.len() == 0 {
                        return;
                    }
                    add_cell(indexes, row, col, *c.choose(&cand));
                }
            }
            // for row in 0..9 {
            //     println!("{:?}", indexes.board[row]);
            // }
            c.stop();
        },
        10,
    )
}
