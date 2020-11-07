package com.thecsillags.choice;

import com.google.common.collect.Lists;
import org.junit.Test;

import java.util.*;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Executors;

import static org.junit.Assert.assertEquals;

public class ParallelChooserTest {
    /**
     * Chooses all possible combinations of picking 0 or 1 three times. Sorted will give you the binary numbers
     * between 0 and 7.
     */
    @Test public void testPlain() throws InterruptedException {
        final List<String> results = new CopyOnWriteArrayList<>();
        ParallelChooser.run(chooser -> results.add(
                chooser.chooseArg("0", "1") + chooser.chooseArg("0", "1") + chooser.chooseArg("0", "1")),
                () -> Executors.newFixedThreadPool(10));
        results.sort(Comparator.naturalOrder());
        final List<String> expected = Lists.newArrayList(
                "000", "001", "010", "011",
                "100", "101", "110", "111");
        assertEquals(expected, results);
    }

    /**
     * Solve a 3 x 3 magic square -- adds up to 15 across, down, and diagonals. There are 8 solutions. They happen
     * to be isomorphic, but that's not relevant for what I care about here.
     */
    private void magicSquare(final ParallelChooser chooser, final List<String> results) {
        final List<Integer> l = Lists.newArrayList(1, 2, 3, 4, 5, 6, 7, 8, 9);
        final List<Integer> square = new ArrayList<>();

        // pick the first row -- using .pick()  as it will remove the item from l that it returns, so it saves
        // us some bookkeeping
        square.add(chooser.pick(l));
        square.add(chooser.pick(l));
        square.add(chooser.pick(l));

        // does it not sum to 15, we outta here!
        if (square.get(0) + square.get(1) + square.get(2) != 15) {
            return;
        }

        // pick the second row
        square.add(chooser.pick(l));
        square.add(chooser.pick(l));
        square.add(chooser.pick(l));

        // does the 2nd row add up to 15?
        if (square.get(3) + square.get(4) + square.get(5) != 15) {
            return;
        }

        // pick the first item of the third row
        square.add(chooser.pick(l));
        // check the first column and the diagonal
        if (square.get(0) + square.get(3) + square.get(6) != 15
                || square.get(2) + square.get(4) + square.get(6) != 15) {
            return;
        }

        // pick the second item of the third row
        square.add(chooser.pick(l));
        // check the second column
        if (square.get(1) + square.get(4) + square.get(7) != 15) {
            return;
        }

        // pick the last item
        square.add(chooser.pick(l));
        // check the third row, the third column, and the diagonal
        if (
                square.get(6) + square.get(7) + square.get(8) != 15 ||
                        square.get(2) + square.get(5) + square.get(8) != 15 ||
                        square.get(0) + square.get(4) + square.get(8) != 15
        ) {
            return;
        }

        // if we're still here, we win!
        results.add(String.format(
                "%d %d %d %d %d %d %d %d %d",
                square.get(0), square.get(1), square.get(2),
                square.get(3), square.get(4), square.get(5),
                square.get(6), square.get(7), square.get(8)
        ));
    }

    @Test
    public void testMagicSquares() throws InterruptedException {
        final List<String> expected = Arrays.asList(
                "2 7 6 9 5 1 4 3 8",
                "2 9 4 7 5 3 6 1 8",
                "4 3 8 9 5 1 2 7 6",
                "4 9 2 3 5 7 8 1 6",
                "6 1 8 7 5 3 2 9 4",
                "6 7 2 1 5 9 8 3 4",
                "8 1 6 3 5 7 4 9 2",
                "8 3 4 1 5 9 6 7 2"
        );
        final List<String> results = Collections.synchronizedList(Lists.newArrayList());
        ParallelChooser.run((c)-> magicSquare(c, results), () -> Executors.newFixedThreadPool(10));
        results.sort(Comparator.naturalOrder());
        assertEquals(expected, results);
    }
}
