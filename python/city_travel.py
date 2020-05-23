from choice import run_choices, Chooser

travel = {}


def byMode(src, dest, mode):
    travel.setdefault(src, []).append((dest, mode))


def byCar(src, dest):
    byMode(src, dest, "car")


def byTrain(src, dest):
    byMode(src, dest, "train")


def byPlane(src, dest):
    byMode(src, dest, "plane")


byCar("auckland", "hamilton")
byCar("hamilton", "raglan")
byCar("valmont", "saarbrueken")
byCar("valmont", "metz")

byTrain("metz", "frankfurt")
byTrain("saarbrueken", "frankfurt")
byTrain("metz", "paris")
byTrain("saarbrueken", "paris")

byPlane("frankfurt", "bangkok")
byPlane("frankfurt", "singapore")
byPlane("paris", "losAngeles")
byPlane("bangkok", "auckland")
byPlane("singapore", "auckland")
byPlane("losAngeles", "auckland")


def findPath(src, dest, chooser: Chooser):
    path = [(src, "start")]
    current = src

    while current != dest:
        candidates = travel.get(current)
        if candidates is None:
            return
        next = chooser.choose(candidates)
        path.append(next)
        current = next[0]
    print(path)


run_choices(lambda c: findPath("valmont", "auckland", c))
