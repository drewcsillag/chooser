# chooser
Solve constraint problems in a more prology kind of way

I was reading a book on prolog and it reminded me of something I had written years ago, which I realized
effectively did things similar to the way prolog works. That is: you just define what's true and it does
what would otherwise be some recursive search. So I rewrote the idea from what I remembered

There are versions in Python, Java, TypeScript, and JavaScript. Each of the versions has two example cases:
1. print solutions to the 3x3 magic square
2. print the binary numbers from 0 to 7

The python version has a few other examples:
* solve a kenken puzzle
* solve sudoku puzzles
* A translation of one of the tasks from "Learn Prolog Now" page 68.

# How to use it
You write a function that takes a Chooser. Whenever it would have a choice of some set of things, you call
```
res = chooser.choose(things to choose from)
```
It will return one of the items. You then presumably do things based on that choice. And you can call
`choser.choose` as many times as you need to make choices. The ChooserRunner will make sure that your function
is called for all possible combinations of the choices you make. What you pass to `chooser.choose` shoud be the
same whenever the previous choices return the same thing.

# How it works
The outer component is the ChoiceRunner. What it does is runs the function once, which should produce one
or more exeuctions. Then, while there are executions left, runs the function for each of the executions -- which
subsequent runs may produce more executions.

What is an execution? An execution is a predetermined set of choices that the chooser will return. The chooser contains
the execution and whatever choices it makes after the predetermined choices are made. When a choice beyond the
predetermined set is made, it returns the first in the choice list and queues up executions for the other choices that
could have been made at that point.

## Big O
for N = choices available and M = choice points, Execution-wise, it's O(N^M). Space-wise, it's O(N*M)

# TODO
* Clean up typescript package references (I basically just copied it from somewhere else and dropped it in).
* Figure out how to make a pip package for python chooser

