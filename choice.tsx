export class ChoiceRunner {
  private executions: number[][] = [];

  public run(fn: (c: Chooser) => void): void {
    const firstChooser = new Chooser(this, []);
    fn(firstChooser);
    while (this.executions.length > 0) {
      const execution = this.executions.pop();
      const chooser = new Chooser(this, execution);
      fn(chooser);
    }
  }

  public addExecution(execution: number[]): void {
    this.executions.push(execution);
  }
}

export class Chooser {
  private runner: ChoiceRunner;
  private preChosen: number[];
  private index: number;
  private newChoices: number[];

  constructor(runner: ChoiceRunner, prechosen: number[]) {
    this.runner = runner;
    this.preChosen = prechosen;
    this.index = 0;
    this.newChoices = [];
  }

  public choose_index(numArgs: number): number {
    if (this.index < this.preChosen.length) {
      const retind = this.preChosen[this.index];
      this.index++;
      return retind;
    }

    for (let i = 1; i < numArgs; i++) {
      const execution = [...this.preChosen, ...this.newChoices, i];
      this.runner.addExecution(execution);
    }
    this.newChoices.push(0);
    return 0;
  }

  public choose<T>(l: T[]): T {
    const ind = this.choose_index(l.length);
    return l[ind];
  }

  public pick<T>(l: T[]): T {
    const ind = this.choose_index(l.length);
    const ret = l[ind];
    l.splice(ind, 1);
    return ret;
  }
}

function test_binary_counter(c: Chooser) {
  const l = [c.choose([0, 1]), c.choose([0, 1]), c.choose([0, 1])];
  console.log(l);
}

function test_solve_magic_square(c: Chooser, counterbox: number[]) {
  const left = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  const square = [];
  counterbox[1]++;

  square.push(c.pick(left));
  square.push(c.pick(left));
  square.push(c.pick(left));
  if (square[0] + square[1] + square[2] !== 15) {
    return;
  }

  square.push(c.pick(left));
  square.push(c.pick(left));
  square.push(c.pick(left));
  if (square[3] + square[4] + square[5] !== 15) {
    return;
  }

  square.push(c.pick(left));
  if (square[0] + square[3] + square[6] !== 15 || square[2] + square[4] + square[6] !== 15) {
    return;
  }

  square.push(c.pick(left));
  if (square[1] + square[4] + square[7] !== 15) {
    return;
  }

  square.push(c.pick(left));
  if (
    square[6] + square[7] + square[8] !== 15 ||
    square[2] + square[5] + square[8] !== 15 ||
    square[0] + square[4] + square[8] !== 15
  ) {
    return;
  }

  console.log(square.slice(0, 3));
  console.log(square.slice(3, 6));
  console.log(square.slice(6, 9));
  console.log('');
  counterbox[0] += 1;
}
const runner = new ChoiceRunner();
const counterbox = [0, 0];
runner.run((c: Chooser) => test_solve_magic_square(c, counterbox));
console.log('solutions, total executions:', counterbox);
runner.run(test_binary_counter);
