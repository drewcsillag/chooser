package com.thecsillags.choice;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedDeque;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class ParallelChooser {
    private final List<Integer> preChosen;
    private final ArrayList<Integer> newChoices;
    private final ChooserState state;

    private int index;

    private static class ChooserState {
        private final AtomicInteger submitted;
        private final AtomicInteger finished;
        private final ExecutorService executor;
        private final Consumer<ParallelChooser> fn;
        private final AtomicBoolean shouldStop;

        private ChooserState(
                final AtomicInteger submitted,
                final AtomicInteger finished,
                final ExecutorService executor,
                final AtomicBoolean shouldStop,
                final Consumer<ParallelChooser> fn) {
            this.submitted = submitted;
            this.finished = finished;
            this.executor = executor;
            this.shouldStop = shouldStop;
            this.fn = fn;
        }

        void submit(final List<Integer> execution) {
            if (!shouldStop.get()) {
                executor.submit(() -> {
                    submitted.incrementAndGet();
                    fn.accept(new ParallelChooser(this, execution));
                    finished.incrementAndGet();
                });
            }
        }

        void stop() {
            shouldStop.set(true);
            executor.shutdown();
        }
    }

    public ParallelChooser(final ChooserState state, final List<Integer> preChosen) {
        this.state = state;
        this.preChosen = preChosen;
        this.index = 0;
        this.newChoices = new ArrayList<>();
    }

    public static void run(
            final Consumer<ParallelChooser> fn,
            final Supplier<ExecutorService> executorSupplier) throws InterruptedException {
        final ConcurrentLinkedDeque<List<Integer>> executions = new ConcurrentLinkedDeque<>();
        final ExecutorService executor = executorSupplier.get();

        final AtomicInteger finished = new AtomicInteger(0);
        final AtomicInteger submitted = new AtomicInteger(0);
        final AtomicBoolean shouldStop = new AtomicBoolean(false);
        final ChooserState state = new ChooserState(submitted, finished, executor, shouldStop, fn);

        // what's the termination condition?
        // a) if the function we're running says so (i.e. they set shouldStop to true)
        // b) submitted is > 0 and there are no current executions running (i.e. submitted == finished)
        state.submit(new ArrayList<>());

        while (submitted.get() != finished.get() && !shouldStop.get() || submitted.get() == 0) {
            Thread.sleep(10);
        }

    }

    public int chooseIndex(final int numArgs) {
        if (index < preChosen.size()) {
            final int retind = preChosen.get(index);
            index++;
            return retind;
        }

        for (int i = 1 ; i < numArgs; i++) {
            final List<Integer> execution = new ArrayList<>();
            execution.addAll(preChosen);
            execution.addAll(newChoices);
            execution.add(i);
            state.submit(execution);
        }
        newChoices.add(0);
        return 0;
    }

    <T> T chooseArg(final T... choices) {
        return choices[chooseIndex(choices.length)];
    }

    <T> T choose(final List<T> choices) {
        return choices.get(chooseIndex(choices.size()));
    }

    <T> T pick(final List<T> choices) {
        final int ind = chooseIndex(choices.size());
        final T ret = choices.get(ind);
        choices.remove(ind);
        return ret;
    }

    void stop() {
        state.stop();
    }
}
