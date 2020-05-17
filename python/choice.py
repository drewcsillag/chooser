class Chooser(object):
    def __init__(self, runner, prechosen):
        self.runner = runner
        self.prechosen = prechosen
        self.index = 0
        self.newChoices = []

    def choose(self, args):
        if self.index < len(self.prechosen):
            ret = self.prechosen[self.index]
            self.index += 1
            return ret
        else:
            for i in args[1:]:
                execution = self.prechosen + self.newChoices + [i]
                self.runner.executions.append(execution)
            self.newChoices.append(args[0])
            return args[0]

    def stop(self):
        self.runner.stop()


class ChoiceRunner(object):
    def __init__(self):
        self.executions = []

    def run(self, fn):
        chooser = Chooser(self, [])
        fn(chooser)
        while self.executions:
            execution = self.executions[-1]
            chooser = Chooser(self, execution)
            self.executions = self.executions[:-1]
            fn(chooser)

    def stop(self):
        self.executions = []


def test_solve_magic_square(chooser, counterbox):
    left = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    square = []
    counterbox[1] += 1

    for i in range(9):
        c = chooser.choose(left)
        left.remove(c)
        square.append(c)

    if square[0] + square[1] + square[2] != 15:
        return
    elif square[3] + square[4] + square[5] != 15:
        return
    elif (
        square[0] + square[3] + square[6] != 15
        or square[2] + square[4] + square[6] != 15
    ):
        return
    elif square[1] + square[4] + square[7] != 15:
        return

    elif square[6] + square[7] + square[8] != 15:
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


if __name__ == "__main__":
    runner = ChoiceRunner()
    counterbox = [0, 0]
    runner.run(lambda chooser: test_solve_magic_square(chooser, counterbox))
    print("solutions", counterbox)
