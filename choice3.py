from typing import Callable, List, TypeVar

T = TypeVar("T")


class Chooser(object):
    def __init__(self, runner: 'ChoiceRunner', prechosen: List[int]):
        self.runner = runner
        self.prechosen = prechosen
        self.index = 0
        self.newChoices: List[int] = []

    def choose_index(self, numArgs: int) -> int:
        if self.index < len(self.prechosen):
            retind = self.prechosen[self.index]
            self.index += 1
            return retind

        for i in range(1, numArgs):
            execution = self.prechosen + self.newChoices + [i]
            self.runner.executions.append(execution)
        self.newChoices.append(0)
        return 0

    def choose(self, args: List[T]) -> T:
        try:
            index = self.choose_index(len(args))
            return args[index]
        except IndexError:
            print("trying index %d of %r" % (index, args))
            raise

    def pick(self, l: List[T]) -> T:
        c = self.choose_index(len(l))
        ret = l[c]
        del l[c]
        return ret

    def stop(self) -> None:
        self.runner.executions[:] = []


class ChoiceRunner(object):
    def __init__(self) -> None:
        self.executions: List[List[int]] = []

    def run(self, fn: Callable[[Chooser], None]) -> None:
        chooser = Chooser(self, [])
        fn(chooser)
        while self.executions:
            execution = self.executions[-1]
            chooser = Chooser(self, execution)
            self.executions = self.executions[:-1]
            fn(chooser)


def test_solve_magic_square(c: Chooser, counterbox: List[int]) -> None:
    left = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    square = []
    counterbox[1] += 1

    square.append(c.pick(left))  # 0
    square.append(c.pick(left))  # 1
    square.append(c.pick(left))  # 2
    if square[0] + square[1] + square[2] != 15:
        return

    square.append(c.pick(left))  # 3
    square.append(c.pick(left))  # 4
    square.append(c.pick(left))  # 5
    if square[3] + square[4] + square[5] != 15:
        return

    square.append(c.pick(left))  # 6
    if (
        square[0] + square[3] + square[6] != 15
        or square[2] + square[4] + square[6] != 15
    ):
        return

    square.append(c.pick(left))  # 7
    if square[1] + square[4] + square[7] != 15:
        return

    square.append(c.pick(left))  # 8
    if square[6] + square[7] + square[8] != 15:
        return
    elif square[2] + square[5] + square[8] != 15:
        return
    elif square[0] + square[4] + square[8] != 15:
        return

    print(square[0:3])
    print(square[3:6])
    print(square[6:9])
    print()
    counterbox[0] += 1
    # chooser.stop() # to stop at first solution


def test_binary_counter(c: Chooser) -> None:
    print([c.choose([0, 1]) for i in range(3)])


if __name__ == "__main__":
    runner = ChoiceRunner()
    counterbox = [0, 0]
    runner.run(lambda chooser: test_solve_magic_square(chooser, counterbox))
    print("solutions, total executions:", counterbox)
    runner.run(test_binary_counter)
