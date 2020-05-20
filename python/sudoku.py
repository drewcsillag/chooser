from choice import ChoiceRunner, Chooser
from typing import Set, List
def computeBoxIndex(row, column):
    boxrow = row//3
    boxcol = column//3
    return boxrow * 3 + boxcol

boxIndex = []
for row in range(9):
    rbi = []
    boxIndex.append(rbi)
    for column in range(9):
        rbi.append(computeBoxIndex(row, column))

def addTo(row, column, number, sets):
    rows, columns, boxes = sets

    rows[row].add(number)
    columns[column].add(number)

    boxes[boxIndex[row][column]].add(number)

def init_indexes(board) -> (List[Set[int]], List[Set[int]], List[Set[int]]):
    rows = [set() for i in range(9)]
    cols = [set() for i in range(9)]
    boxes = [set() for i in range(9)]
    for row in range(9):
        for col in range(9):
            number = board[row][col]
            if number > 0:
                rows[row].add(number)
                cols[col].add(number)
                boxes[boxIndex[row][col]].add(number)
    return rows, cols, boxes

def canSee(row, column, sets):
    rows, columns, boxes = sets
    res = set(rows[row])
    res.update(columns[column])
    res.update(boxes[boxIndex[row][column]])
    return list(res)

def all_but(candidates: List[int], *used: int) -> List[int]:
    """Return items in candidates that are not in used."""
    leftovers = set(candidates).difference(used)
    return [c for c in candidates if c in leftovers]

CAND = [1,2,3,4,5,6,7,8,9]
counts = [0]
def sudoku(c: Chooser, counts, board):
    counts[0]+=1

    # print("INIT BOARD")
    # printboard(board)
    rows, columns, boxes = init_indexes(board)
    for row in range(9):
        for col in range(9):
            # presupplied cell
            if board[row][col] != 0:
                continue
            # what #'s can go in the cell
            candidates = all_but(CAND, *canSee(row, col, (rows, columns, boxes)))
            if not candidates:
                return
            board[row][col] = num = c.choose(candidates)
            addTo(row, col, num, (rows, columns, boxes))
    printboard(board)
    c.stop()

def printboard(board):
    for row in board:
        print(row)
    print

if __name__ == '__main__':
    cr = ChoiceRunner()
    counts =[0]
    # an easier board
    cr.run(lambda c: sudoku(c, counts, [
        [9,2,0,0,0,5,8,0,0],
        [0,0,1,7,2,6,3,0,9],
        [0,0,3,8,9,1,2,0,6],

        [0,8,0,0,0,0,1,0,2],
        [7,0,0,0,6,0,5,0,8],
        [0,0,0,0,3,0,7,0,0],

        [5,0,8,0,1,3,0,0,7],
        [0,4,0,6,0,7,9,1,5],
        [0,0,0,2,0,0,6,0,0]
    ]))
    print("iterations: ",counts[0])

    counts =[0]

    # a hard puzzle I found somewhere
    cr.run(lambda c: sudoku(c, counts, [
        [0,3,0,6,0,0,0,8,0],
        [0,0,9,8,0,1,7,0,2],
        [0,0,0,5,0,0,0,0,6],

        [0,0,0,0,1,0,0,0,3],
        [0,8,5,0,0,0,9,0,4],
        [0,7,0,0,2,0,0,0,0],

        [0,9,0,0,0,7,0,0,0],
        [0,5,3,0,0,0,0,0,0],
        [0,0,0,0,9,0,0,4,7]
    ]))
    print("iterations: ",counts[0])