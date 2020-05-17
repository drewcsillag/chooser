import choice3
from typing import List

# +----+----+----+----+
# |     16x | 7+      |
# | (0)  (1)| (2)  (3)|
# +----+-   +-   +----+
# | 2- |    |    |  4 |
# | (4)| (5)| (6)| (7)|
# +   -+----+----+----+
# |    |    | 2/      |
# | (8)| (9)|(10) (11)|
# +----+-   +----+----+
# |     12x | 2/      |
# |(12) (13)|(14) (15)|
# +----+----+----+----+


def sub_check(v1: int, v2: int, diff: int) -> bool:
    if v1 - v2 == diff or v2 - v1 == diff:
        return True
    return False


def div_check(v1: int, v2: int, quot: int) -> bool:
    if v1 / v2 == quot or v2 / v1 == quot:
        return True
    return False


class NoChoices(Exception):
    pass


def all_but(candidates: List[int], *used: int) -> List[int]:
    """Return items in candidates that are not in used."""
    leftovers = set(candidates).difference(used)
    if not leftovers:
        raise NoChoices()
    return [c for c in candidates if c in leftovers]


ONE_TO_FOUR = [1, 2, 3, 4]


def add_choice(row: List[int], c: choice3.Chooser, *used: int) -> None:
    """Choose a item from [1-4] excluding ones that have been used already)
    and append it to row."""
    row.append(c.choose(all_but(ONE_TO_FOUR, *used)))


def kenken(c: choice3.Chooser, box: List[int]) -> None:
    box[0] += 1

    row: List[int] = []
    add_choice(row, c)
    add_choice(row, c, row[0])
    add_choice(row, c, row[0], row[1])
    add_choice(row, c, row[0], row[1], row[2])

    try:
        # 2nd row
        add_choice(row, c, 4, row[0])  # C4
        add_choice(row, c, 4, row[1], row[4])
        if row[0] * row[1] * row[5] != 16:  # 16x
            return
        add_choice(row, c, 4, row[2], row[4], row[5])
        if row[2] + row[3] + row[6] != 7:  # 7+
            return

        row.append(4)  # 4

        # 3rd row
        add_choice(row, c, row[0], row[4])
        if not sub_check(row[8], row[4], 2):  # 2-
            return
        add_choice(row, c, row[1], row[5], row[8])
        add_choice(row, c, row[2], row[6], row[8], row[9])
        add_choice(row, c, row[3], row[7], row[8], row[9], row[10])
        if not div_check(row[10], row[11], 2):  # 2-
            return

        # 4th row
        add_choice(row, c, row[0], row[4], row[8])
        add_choice(row, c, row[1], row[5], row[9], row[12])
        if row[9] * row[12] * row[13] != 12:  # 12x
            return
        add_choice(row, c, row[2], row[6], row[10], row[12], row[13])
        add_choice(row, c, row[3], row[7], row[11], row[12], row[13], row[14])
        if not div_check(row[14], row[15], 2):  # 2/
            return

        print(row)
        for i in range(4):
            print(row[i * 4 : i * 4 + 4])
        print
        c.stop()
    except NoChoices:
        pass


box = [0]
c = choice3.ChoiceRunner()
c.run(lambda c: kenken(c, box))
print(box)

# [2, 4, 1, 3, 1, 2, 3, 4, 3, 1, 4, 2, 4, 3, 2, 1, 1]
# [2, 4, 1, 3]
# [1, 2, 3, 4]
# [3, 1, 4, 2]
# [4, 3, 2, 1]
# [72]
