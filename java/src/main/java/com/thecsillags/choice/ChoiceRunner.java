package com.thecsillags.choice;

import java.util.List;
import java.util.function.Consumer;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;

public class ChoiceRunner {
    private final Deque<List<Integer>> executions=new ArrayDeque<>();

    public void run(final Consumer<Chooser> fn) {
        fn.accept(new Chooser(executions, new ArrayList<>()));
        while (!executions.isEmpty()) {
            fn.accept(new Chooser(executions, executions.pop()));
        }
    }


    public static class Chooser {
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

    private static void testMagicSquare(final Chooser c, final int[] counterbox) {
        final List<Integer> l = new ArrayList<>();
        for (int i = 1; i <= 9; i++) {
            l.add(i);
        }
        final List<Integer> square = new ArrayList<>();
        counterbox[1]++;

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
        System.out.println(String.format(
            "%d %d %d\n%d %d %d\n%d %d %d\n",
            square.get(0), square.get(1),square.get(2),
            square.get(3), square.get(4),square.get(5),
            square.get(6), square.get(7),square.get(8)
        ));
        counterbox[0]+=1;
    }

    public static void main(final String... ignoredArgs) {
        final ChoiceRunner r=new ChoiceRunner();
        final int[] counterbox = new int[]{0, 0};
        r.run((c)->testMagicSquare(c, counterbox));
        System.out.println("solutions, total executions: " + counterbox[0] + ", " + counterbox[1]);
    }
}
