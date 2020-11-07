package com.thecsillags.choice;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Consumer;
import java.util.function.Supplier;

/**
 * Like the Chooser but can use an executor to do stuff in parallel. Like the single threaded Chooser, does not handle
 * exceptions -- it could, it wouldn't be especially hard, but I didn't bother for now.
 */
public class ParallelChooser {
    private final List<Integer> preChosen; // The choices we have to make before... FREEDOM!
    private final List<Integer> newChoices; // Choices we made after the pre-chosen ones
    private final ChooserState state; // The bit that really handles all the concurrency bits

    // When we have pre-chosen choices to make, which step along that way we are
    private int index;

    private ParallelChooser(final ChooserState state, final List<Integer> preChosen) {
        this.state = state;
        this.preChosen = preChosen;
        this.index = 0;
        this.newChoices = new ArrayList<>();
    }

    /**
     * Run a chooser session.
     *
     * @param fn The function to execute all possible choices of
     * @param executorSupplier supplies the ExecutorService instance to execute fn on -- NOTE: the chooser will call
     *                         shutdown() on it when it's done, so if you were planning to have it return one you
     *                         already have, you will have a sad.
     * @throws InterruptedException
     */
    public static void run(
            final Consumer<ParallelChooser> fn,
            final Supplier<ExecutorService> executorSupplier) throws InterruptedException {
        final ChooserState state = new ChooserState(executorSupplier.get(), fn);
        // kick off the first execution
        state.submit(new ArrayList<>());

        state.waitForDone();
    }

    /**
     * Pick a number between 0 and numargs and return it.
     */
    public int chooseIndex(final int numArgs) {
        // if we have pre-decided choices that we're forking from a previous execution, return the prechosen indexes
        if (index < preChosen.size()) {
            final int retind = preChosen.get(index);
            index++;
            return retind;
        }

        // queue up choosing all the other choices except for the first
        for (int i = 1; i < numArgs; i++) {
            final List<Integer> execution = new ArrayList<>();
            execution.addAll(preChosen);
            execution.addAll(newChoices);
            execution.add(i);
            state.submit(execution);
        }
        // return the first choice
        newChoices.add(0);
        return 0;
    }

    /**
     * Return one of the items from choices.
     */
    public <T> T chooseArg(final T... choices) {
        return choices[chooseIndex(choices.length)];
    }

    /**
     * Return one of the items from choices.
     */
    public <T> T choose(final List<T> choices) {
        return choices.get(chooseIndex(choices.size()));
    }

    /**
     * Pick an item from choices, remove it from choices, and return it. Obviously, choices must be mutable.
     */
    public <T> T pick(final List<T> choices) {
        final int ind = chooseIndex(choices.size());
        final T ret = choices.get(ind);
        choices.remove(ind);
        return ret;
    }

    /** Stop the chooser engine from starting more executions. */
    public void stop() {
        state.stop();
    }

    private static class ChooserState {
        // If we ever had more than 2^32 executions pending (signedness of the int not withstanding), this would be a
        // problem, but that's not something I'm worried about. If you have more than 2^31 executions ever, the
        // underlying int will wrap around, but that's ok as submitted will not == finished until all the stuff is done.
        private final AtomicInteger submitted; // the number of executions submitted
        private final AtomicInteger finished; // the number of completed executions
        private final ExecutorService executor;
        private final Consumer<ParallelChooser> fn;
        private final AtomicBoolean shouldStop;

        private ChooserState(
                final ExecutorService executor,
                final Consumer<ParallelChooser> fn) {
            this.finished = new AtomicInteger(0);
            this.submitted = new AtomicInteger(0);
            this.shouldStop = new AtomicBoolean(false);
            this.executor = executor;
            this.fn = fn;
        }

        boolean keepGoing() {
            return submitted.get() != finished.get() && !shouldStop.get();
        }

        void waitForDone() throws InterruptedException {
            // what's the termination condition?
            // a) if the function we're running says so (i.e. they set shouldStop to true)
            // b) if submitted == finished, it means all the work to be done is done.
            while (keepGoing()) {
                // In an ideal world, there would be a concurrency primitive that we could block on to do this, and
                // maybe there is and I'm just blanking-- if CountDownLatch could count up too, that'd be ideal, but
                // :shrug: -- the price of lock-free concurrency is that sometimes, you spin a bit in a thread.

                // Another alternative would be to keep a queue of Futures from executor.submit and just consume the
                // queue calling future.get as you go, but that seems more clunky and plumbing-y than this.
                Thread.sleep(1);
                // Also, like many other concurrency wait functions, we *could* have a max wait time as an argument and
                // bail on the wait after that time, but at least for now, I don't need it.
            }
        }

        void submit(final List<Integer> execution) {
            if (!shouldStop.get()) {
                // Important to do this outside the executor submit bit as the body of that will run....whenever,
                // which can cause finished to catch up and the chooser to "finish" prematurely.
                submitted.incrementAndGet();

                executor.submit(() -> {
                    // I should eventually add some exception handling for this. In the single threaded impl, an exc
                    // would just stop the world, but in a parallel universe, not so. An exception thrown by fn would
                    // cause the chooser to wait forever.
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
}
