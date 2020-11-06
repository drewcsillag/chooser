package com.thecsillags.choice;

import com.google.common.collect.Lists;
import org.junit.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Executors;

import static org.junit.Assert.assertEquals;

public class ParallelChooserTest {
    @Test public void testPlain() throws InterruptedException {
        final List<String> results = new CopyOnWriteArrayList<>();
        ParallelChooser.run((c) -> results.add(
                c.chooseArg("0", "1") + c.chooseArg("0", "1") + c.chooseArg("0", "1")),
                () -> Executors.newFixedThreadPool(10));
        results.sort(Comparator.naturalOrder());
        final ArrayList<String> expected = Lists.newArrayList(
                "000", "001", "010", "011",
                "100", "101", "110", "111");
        assertEquals(expected, results);
    }

    private void magicSquare(final ParallelChooser c, final List<String> results) {
        final List<Integer> l = Lists.newArrayList(1, 2, 3, 4, 5, 6, 7, 8, 9);
        final List<Integer> square = new ArrayList<>();

        square.add(c.pick(l));
        square.add(c.pick(l));
        square.add(c.pick(l));
        if (square.get(0) + square.get(1) + square.get(2) != 15) {
            return;
        }

        square.add(c.pick(l));
        square.add(c.pick(l));
        square.add(c.pick(l));
        if (square.get(3) + square.get(4) + square.get(5) != 15) {
            return;
        }

        square.add(c.pick(l));
        if (square.get(0) + square.get(3) + square.get(6) != 15
                || square.get(2) + square.get(4) + square.get(6) != 15) {
            return;
        }

        square.add(c.pick(l));
        if (square.get(1) + square.get(4) + square.get(7) != 15) {
            return;
        }

        square.add(c.pick(l));
        if (
                square.get(6) + square.get(7) + square.get(8) != 15 ||
                        square.get(2) + square.get(5) + square.get(8) != 15 ||
                        square.get(0) + square.get(4) + square.get(8) != 15
        ) {
            return;
        }

        results.add(String.format(
                "%d %d %d %d %d %d %d %d %d",
                square.get(0), square.get(1),square.get(2),
                square.get(3), square.get(4),square.get(5),
                square.get(6), square.get(7),square.get(8)
        ));
    }

    @Test
    public void testMagicSquares() throws InterruptedException {
        final List<String> expected = Arrays.asList(
                "8 1 6 " +
                        "3 5 7 " +
                        "4 9 2",

                "8 3 4 " +
                        "1 5 9 " +
                        "6 7 2",

                "6 1 8 " +
                        "7 5 3 " +
                        "2 9 4",

                "6 7 2 " +
                        "1 5 9 " +
                        "8 3 4",

                "4 9 2 " +
                        "3 5 7 " +
                        "8 1 6",

                "4 3 8 " +
                        "9 5 1 " +
                        "2 7 6",

                "2 9 4 " +
                        "7 5 3 " +
                        "6 1 8",

                "2 7 6 " +
                        "9 5 1 " +
                        "4 3 8"
        );
        final List<String> results = new CopyOnWriteArrayList<>();
        ParallelChooser.run((c)-> magicSquare(c, results), () -> Executors.newFixedThreadPool(10));
        results.sort(Comparator.naturalOrder());
        expected.sort(Comparator.naturalOrder());
        assertEquals(expected, results);
    }
}
