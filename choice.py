
class Chooser(object):
    def __init__(self, runner, prechosen):
        self.runner = runner
        self.prechosen = prechosen
        self.index = 0
        self.newChoices = []
        # print("CI prechose %r" % (prechosen,))

    def choose(self, *args):
        if self.index < len(self.prechosen):
            ret = self.prechosen[self.index]
            self.index += 1
            return ret
        else:
            for i in args[1:]:
                execution = self.prechosen + self.newChoices + [i]
                self.runner.executions.append(
                    execution
                )
                # print("New exec %r" % (execution,))
            self.newChoices.append(args[0])
            return args[0]

class ChoiceRunner(object):
    def __init__(self):
        self.executions = []
        self.maxlen = 0

    def run(self, fn):
        chooser = Chooser(self, [])
        fn(chooser)
        while self.executions:
            self.maxlen = max(self.maxlen, len(self.executions))

            execution = self.executions[-1]
            chooser = Chooser(self, execution)
            self.executions = self.executions[:-1]
            fn(chooser)


def f(c):

    c1 = c.choose(0, 1)
    c2 = c.choose(0,1 )
    c3 = c.choose(0,1)
    print("%r %r %r" % (c1, c2, c3))

runner=ChoiceRunner()
runner.run(f)
print("maxlen %r" %(runner.maxlen,))
