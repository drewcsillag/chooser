package com.thecsillags.choice;

import java.util.List;
import java.util.function.Consumer;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;

public class Chooser {
    private final Deque<List<Integer>> executions;
    private final List<Integer> preChosen;
    private final ArrayList<Integer> newChoices;

    private int index;

    public Chooser(final Deque<List<Integer>> executions, final List<Integer> preChosen) {
        this.executions = executions;
        this.preChosen = preChosen;
        this.index = 0;
        this.newChoices = new ArrayList<Integer>();
    }

    public static void run(final Consumer<Chooser> fn) {
        final Deque<List<Integer>> executions = new ArrayDeque<>();
        executions.add(new ArrayList<>());
        while (!executions.isEmpty()) {
            fn.accept(new Chooser(executions, executions.pop()));
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
            executions.push(execution);
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
}
