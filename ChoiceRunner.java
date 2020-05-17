// package com.thecsillags;

import java.util.List;
import java.util.concurrent.Callable;
import java.util.function.Consumer;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;

public class ChoiceRunner {
    private final Deque<List<Integer>> executions=new ArrayDeque<>();

    public void run(Consumer<Chooser> fn) {
        fn.accept(new Chooser(executions, new ArrayList<>()));
        while (!executions.isEmpty()) {
            fn.accept(new Chooser(executions, executions.pop()));
        }
    }


    public static class Chooser {
        private final Deque<List<Integer>> executions;
        private final List<Integer> preChosen;
        private int index;
        private final ArrayList<Integer> newChoices;

        public Chooser(Deque<List<Integer>> executions, List<Integer> preChosen) {
            this.executions = executions;
            this.preChosen = preChosen;
            this.index = 0;
            this.newChoices = new ArrayList<Integer>();
        }

        public int chooseIndex(int numArgs) {
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

        <T> T chooseArg(T... choices) {
            return choices[chooseIndex(choices.length)];
        }

        <T> T choose(List<T> choices) {
            return choices.get(chooseIndex(choices.size()));
        }

        <T> T pick(List<T> choices) {
            final int ind = chooseIndex(choices.size());
            final T ret = choices.get(ind);
            choices.remove(ind);
            return ret;
        }
    }

    private static void testMagicSquare(Chooser c, int[] counterbox) {
        List<Integer> l = new ArrayList<>();
        l.add(1);
        l.add(2);
        l.add(3);
        l.add(4);
        l.add(5);
        l.add(6);
        l.add(7);
        l.add(8);
        l.add(9);
        List<Integer> square = new ArrayList<>();
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

    public static void main(String... args) {
        final ChoiceRunner r=new ChoiceRunner();
        r.run((c) -> {
            System.out.println("" + c.chooseArg(0,1) + "" + + c.chooseArg(0,1) + "" + c.chooseArg(0,1));
        });
        final int[] counterbox = new int[]{0, 0};
        r.run((c)->testMagicSquare(c, counterbox));
        System.out.println("solutions, total executions: " + counterbox[0] + ", " + counterbox[1]);
    }
}