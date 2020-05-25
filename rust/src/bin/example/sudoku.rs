use std::collections::HashSet;
use chooser;

const BOX_INDEXES: [[usize; 9]; 9] = [
    [0,0,0,1,1,1,2,2,2],
    [0,0,0,1,1,1,2,2,2],
    [0,0,0,1,1,1,2,2,2],
    [3,3,3,4,4,4,5,5,5],
    [3,3,3,4,4,4,5,5,5],
    [3,3,3,4,4,4,5,5,5],
    [6,6,6,7,7,7,8,8,8],
    [6,6,6,7,7,7,8,8,8],
    [6,6,6,7,7,7,8,8,8],
];

struct Indexes {
    rows: Vec<HashSet<u8>>,
    cols: Vec<HashSet<u8>>,
    boxes: Vec<HashSet<u8>>,
    board: [[u8; 9]; 9]
}


fn index_board(board: [[u8; 9]; 9]) -> Indexes {
    let mut result = Indexes {
        rows: (0..9).map(|_| HashSet::new()).collect(),
        cols: (0..9).map(|_| HashSet::new()).collect(),
        boxes: (0..9).map(|_| HashSet::new()).collect(),
        board
    };
    (0..9).for_each(|row| (0..9).for_each(|col| {
        let cell = board[row][col];
        if cell > 0 {
            result.rows[row].insert(cell);
            result.cols[col].insert(cell);
            let box_index = BOX_INDEXES[row][col];
            result.boxes[box_index].insert(cell);
        }
    }));
    return result;
}

fn add_cell(indexes: &mut Indexes, row: usize, col: usize, cell: u8) {
    indexes.rows[row].insert(cell);
    indexes.cols[col].insert(cell);
    indexes.boxes[BOX_INDEXES[row][col]].insert(cell);
    indexes.board[row][col] = cell;
}

fn candidates(indexes: &Indexes, row: usize, col: usize) -> Vec<u8> {
    let cands: HashSet<u8> = (1..10).collect();
    let mut diff: HashSet<u8> = cands
        .difference(&(*indexes).rows[row]).map(|c| *c).collect();
    diff = diff
        .difference(&(*indexes).cols[col]).map(|c| *c).collect();
    diff = diff
        .difference(&(*indexes).boxes[BOX_INDEXES[row][col]]).map(|c| *c).collect();
    let mut result: Vec<u8> = diff.drain().collect();
    result.sort();
    return result;
}

pub fn solve(c: &mut chooser::Chooser, board: [[u8; 9]; 9]) {
    let mut indexes = index_board(board);

    for row in 0..9 {
        for col in 0..9 {
            println!("\nrow {0}, col{1}, cell {2}", row, col, indexes.board[row][col]);

            if indexes.board[row][col] != 0 {
                println!("spot filled in, skipping");
                continue;
            }
            let cand = candidates(&indexes, row, col);
            println!("row {0}, col{1}, result {2:?}", row, col, indexes.board);
            if cand.len() == 0 {
                println!("no candidates, backtracking");
                return;
            }
            println!("candidates -> {:?}", cand);
            let choice = *c.choose(&cand);
            println!("chose {0}", choice);
            add_cell(&mut indexes, row, col, choice);
            println!("row index {:?}", indexes.rows[row]);
            println!("col index {:?}", indexes.cols[col]);
            println!("box index {:?}", indexes.boxes[BOX_INDEXES[row][col]]);
        }
    }
    println!("result {:?}", indexes.board);
    c.stop();
}
