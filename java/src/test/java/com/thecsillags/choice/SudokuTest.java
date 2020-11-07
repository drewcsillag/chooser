package com.thecsillags.choice;

import com.google.common.collect.Sets;
import org.junit.Test;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Consumer;
import java.util.stream.IntStream;

import static com.google.common.collect.Lists.newArrayList;
import static java.util.stream.Collectors.toList;
import static org.junit.Assert.assertEquals;

/**
 * A more lengthy test of the parallel chooser.
 * <p>
 * In reality, this could be faster, generate less garbage, etc. The threading assuredly is unnecessary,
 * as it runs faster non-threaded, but when threading is wrong, this fails miserably, which is the point.
 * <p>
 * Also, this is written in a way not so much to be fast (though it finishes sub-second), but to be a decent
 * exposition of the simplicity of chooser usage, and a simplified sudoku solver.
 */
public class SudokuTest {
    private static final BoxIndexCache BOX_INDEX_CACHE = new BoxIndexCache();
    private static final List<Integer> CANDIDATES = newArrayList(1, 2, 3, 4, 5, 6, 7, 8, 9);

    /**
     * Using a Parallel Chooser, solve the sudoku puzzle.
     *
     * @param chooser  the chooser
     * @param puzzle the puzzle to solve
     */
    private static void sudoku(
            final ParallelChooser chooser,
            final Puzzle puzzle,
            final Consumer<Puzzle> solutionConsumer) {
        for (int row = 0; row < 9; row++) {
            for (int col = 0; col < 9; col++) {
                // presupplied cell
                if (puzzle.get(row, col) != 0) {
                    continue;
                }
                final List<Integer> candidates = puzzle.getCandidates(row, col);

                if (candidates.isEmpty()) {
                    return; // this is a dead end
                }
                // place a number and keep going
                puzzle.set(row, col, chooser.choose(candidates));
            }
        }
        solutionConsumer.accept(puzzle);
        chooser.stop();
    }

    private static Puzzle makePuzzle() {
        return new Puzzle(newArrayList(
                newArrayList(9, 2, 0, 0, 0, 5, 8, 0, 0),
                newArrayList(0, 0, 1, 7, 2, 6, 3, 0, 9),
                newArrayList(0, 0, 3, 8, 9, 1, 2, 0, 6),
                newArrayList(0, 8, 0, 0, 0, 0, 1, 0, 2),
                newArrayList(7, 0, 0, 0, 6, 0, 5, 0, 8),
                newArrayList(0, 0, 0, 0, 3, 0, 7, 0, 0),
                newArrayList(5, 0, 8, 0, 1, 3, 0, 0, 7),
                newArrayList(0, 4, 0, 6, 0, 7, 9, 1, 5),
                newArrayList(0, 0, 0, 2, 0, 0, 6, 0, 0)));
    }

    @Test
    public void testSudokuPuzzle() throws InterruptedException {
        final AtomicReference<Puzzle> puzzleBox = new AtomicReference<>();
        ParallelChooser.run(
                c -> sudoku(c, makePuzzle(), puzzleBox::set),
                () -> Executors.newFixedThreadPool(10));

        assertEquals(newArrayList(
                newArrayList(9, 2, 6, 3, 4, 5, 8, 7, 1),
                newArrayList(8, 5, 1, 7, 2, 6, 3, 4, 9),
                newArrayList(4, 7, 3, 8, 9, 1, 2, 5, 6),
                newArrayList(6, 8, 5, 4, 7, 9, 1, 3, 2),
                newArrayList(7, 3, 4, 1, 6, 2, 5, 9, 8),
                newArrayList(2, 1, 9, 5, 3, 8, 7, 6, 4),
                newArrayList(5, 6, 8, 9, 1, 3, 4, 2, 7),
                newArrayList(3, 4, 2, 6, 8, 7, 9, 1, 5),
                newArrayList(1, 9, 7, 2, 5, 4, 6, 8, 3)),
                puzzleBox.get().puzzle);
    }

    /**
     * A quick way to get which box a given (row, col) is in.
     */
    private static class BoxIndexCache {
        private static final List<List<Integer>> boxIndex = newArrayList(
                newArrayList(0, 0, 0, 1, 1, 1, 2, 2, 2),
                newArrayList(0, 0, 0, 1, 1, 1, 2, 2, 2),
                newArrayList(0, 0, 0, 1, 1, 1, 2, 2, 2),
                newArrayList(3, 3, 3, 4, 4, 4, 5, 5, 5),
                newArrayList(3, 3, 3, 4, 4, 4, 5, 5, 5),
                newArrayList(3, 3, 3, 4, 4, 4, 5, 5, 5),
                newArrayList(6, 6, 6, 7, 7, 7, 8, 8, 8),
                newArrayList(6, 6, 6, 7, 7, 7, 8, 8, 8),
                newArrayList(6, 6, 6, 7, 7, 7, 8, 8, 8)
        );

        int getBoxIndex(final int row, final int col) {
            return boxIndex.get(row).get(col);
        }
    }

    /**
     * Indexes to make lookups of the constraints on a given row, column, box of a sudoku puzzle.
     * <p>
     * This is *very* mutable.
     */
    private static class Puzzle {
        private final List<Set<Integer>> rows; // for each of the rows, what numbers live there
        private final List<Set<Integer>> columns; // for each of the columns, what numbers live there
        private final List<Set<Integer>> boxes; // for each of the boxes, what numbers live there
        private final List<List<Integer>> puzzle; // the puzzle

        private Puzzle(final List<List<Integer>> puzzle) {
            this.puzzle = puzzle;
            this.rows = IntStream.range(0, 9).mapToObj(ignored -> Sets.<Integer>newHashSet()).collect(toList());
            this.columns = IntStream.range(0, 9).mapToObj(ignored -> Sets.<Integer>newHashSet()).collect(toList());
            this.boxes = IntStream.range(0, 9).mapToObj(ignored -> Sets.<Integer>newHashSet()).collect(toList());

            // load the puzzle as it is into the indexes
            for (int row = 0; row < 9; row++) {
                for (int col = 0; col < 9; col++) {
                    final Integer number = puzzle.get(row).get(col);
                    if (number > 0) {
                        addToIndex(row, col, number);
                    }
                }
            }
        }

        private int get(final int row, final int col) {
            return puzzle.get(row).get(col);
        }

        private void set(final int row, final int col, final int num) {
            puzzle.get(row).set(col, num);
            addToIndex(row, col, num);
        }

        private void addToIndex(final int row, final int col, final int num) {
            rows.get(row).add(num);
            columns.get(col).add(num);
            boxes.get(BOX_INDEX_CACHE.getBoxIndex(row, col)).add(num);
        }

        /**
         * Get the candidates for the cell denoted by row, col.
         *
         * @param row the row coordinate
         * @param col the column coordinate
         * @return the candidates
         */
        private List<Integer> getCandidates(final int row, final int col) {
            final HashSet<Integer> leftovers = Sets.newHashSet(CANDIDATES);
            leftovers.removeAll(rows.get(row));
            leftovers.removeAll(columns.get(col));
            leftovers.removeAll(boxes.get(BOX_INDEX_CACHE.getBoxIndex(row, col)));
            return newArrayList(leftovers);
        }
    }
}
