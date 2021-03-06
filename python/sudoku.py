from choice import run_choices, Chooser
from typing import Set, List, Tuple, TypeVar

Sets = Tuple[List[Set[int]], List[Set[int]], List[Set[int]]]


def computeBoxIndex(row: int, column: int) -> int:
    """The sudoku board has 9 "Boxes" where numbers need to be unique,
    this, given a row and column computes which box"""

    boxrow = row // 3
    boxcol = column // 3
    return boxrow * 3 + boxcol


# Because why not, let's just precompute all the box info
boxIndex: List[List[int]] = []
for row in range(9):
    rbi: List[int] = []
    boxIndex.append(rbi)
    for column in range(9):
        rbi.append(computeBoxIndex(row, column))


def addTo(row: int, column: int, number: int, sets: Sets) -> None:
    """Add number at row and column to sets"""
    rows, columns, boxes = sets

    rows[row].add(number)
    columns[column].add(number)

    boxes[boxIndex[row][column]].add(number)


def init_indexes(board: List[List[int]]) -> Sets:
    """Make the initial Sets from the board to solve"""
    rows: List[Set[int]] = [set() for i in range(9)]
    cols: List[Set[int]] = [set() for i in range(9)]
    boxes: List[Set[int]] = [set() for i in range(9)]
    for row in range(9):
        for col in range(9):
            number = board[row][col]
            if number > 0:
                rows[row].add(number)
                cols[col].add(number)
                boxes[boxIndex[row][col]].add(number)
    return rows, cols, boxes


def canSee(row: int, column: int, sets: Sets) -> List[int]:
    """What numbers are in view from row/column -- i.e. in the same row, column, or box"""
    rows, columns, boxes = sets
    res = set(rows[row])
    res.update(columns[column])
    res.update(boxes[boxIndex[row][column]])
    return list(res)


def all_but(candidates: List[int], *used: int) -> List[int]:
    """Return items in candidates that are not in used."""
    leftovers = set(candidates).difference(used)
    return [c for c in candidates if c in leftovers]


CANDIDATES = [1, 2, 3, 4, 5, 6, 7, 8, 9]


def sudoku(c: Chooser, counts: List[int], board: List[List[int]]) -> None:
    """Solve the board using the chooser. Track iteration count in counts"""
    counts[0] += 1

    rows, columns, boxes = init_indexes(board)
    for row in range(9):
        for col in range(9):
            # presupplied cell
            if board[row][col] != 0:
                continue
            # what #'s can go in the cell
            candidates = all_but(CANDIDATES, *canSee(row, col, (rows, columns, boxes)))
            if not candidates:
                return
            board[row][col] = num = c.choose(candidates)
            addTo(row, col, num, (rows, columns, boxes))
    printboard(board)
    c.stop()


def printboard(board: List[List[int]]) -> None:
    for row in board:
        print(row)
    print


if __name__ == "__main__":
    counts = [0]
    # an easier board
    run_choices(
        lambda c: sudoku(
            c,
            counts,
            [
                [9, 2, 0, 0, 0, 5, 8, 0, 0],
                [0, 0, 1, 7, 2, 6, 3, 0, 9],
                [0, 0, 3, 8, 9, 1, 2, 0, 6],
                [0, 8, 0, 0, 0, 0, 1, 0, 2],
                [7, 0, 0, 0, 6, 0, 5, 0, 8],
                [0, 0, 0, 0, 3, 0, 7, 0, 0],
                [5, 0, 8, 0, 1, 3, 0, 0, 7],
                [0, 4, 0, 6, 0, 7, 9, 1, 5],
                [0, 0, 0, 2, 0, 0, 6, 0, 0],
            ],
        )
    )
    print("iterations: ", counts[0])

    counts = [0]
    # a hard puzzle
    run_choices(
        lambda c: sudoku(
            c,
            counts,
            [
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
        )
    )
    print("iterations: ", counts[0])
