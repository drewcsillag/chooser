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
}

function test_binary_counter(c: Chooser) {
  const l = [c.choose([0, 1]), c.choose([0, 1]), c.choose([0, 1])];
  console.log(l);
}

const runner = new ChoiceRunner();
runner.run(test_binary_counter);
