import choice3

from typing import List

# C0  C1  C2  C3
# C4  C5  C6  C7
# C8  C9  C10 C11
# C12 C13 C14 C15


def subcheck(v1, v2, diff):
    if v1 - v2 == diff or v2 - v1 == diff:
        return True
    return False


def divcheck(v1, v2, quot):
    if v1 / v2 == quot or v2 / v1 == quot:
        return True
    return False


class NoChoices(Exception):
    pass


def all_but(candidates, *used):
    leftovers = set(candidates).difference(used)
    if not leftovers:
        raise NoChoices()
    return [c for c in candidates if c in leftovers]


ONE_TO_FOUR = [1, 2, 3, 4]


def addChoice(row: List[int], c: choice3.Chooser, *used: int):
    row.append(c.choose(all_but(ONE_TO_FOUR, *used)))


def kenken(c: choice3.Chooser, box):
    box[0] += 1

    oneToFour = [1, 2, 3, 4]  # 1st row
    row = [c.pick(oneToFour) for i in range(4)]  # C0 - C3

    try:
        # 2nd row
        addChoice(row, c, 4, row[0])  # C4
        addChoice(row, c, 4, row[1], row[4])  # C5
        if row[0] * row[1] * row[5] != 16:
            return
        addChoice(row, c, 4, row[2], row[4], row[5])  # C6
        if row[2] + row[3] + row[6] != 7:
            return

        row.append(4)  # C7

        # 3rd row
        addChoice(row, c, row[0], row[4])  # C8
        if not subcheck(row[8], row[4], 2):
            return
        addChoice(row, c, row[1], row[5], row[8])  # C9
        addChoice(row, c, row[2], row[6], row[8], row[9])  # C10
        addChoice(row, c, row[3], row[7], row[8], row[9], row[10])  # C11
        if not divcheck(row[10], row[11], 2):
            return

        # 4th row
        addChoice(row, c, row[0], row[4], row[8])  # C12
        addChoice(row, c, row[1], row[5], row[9], row[12])  # C13
        if row[9] * row[12] * row[13] != 12:
            return
        addChoice(row, c, row[2], row[6], row[10], row[12], row[13])  # C14
        addChoice(row, c, row[3], row[7], row[11], row[12], row[13], row[14])  # C15
        if not divcheck(row[14], row[15], 2):
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