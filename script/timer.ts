export class Timer {
  _counter: number;
  _frequency: number;
  _start_at: number;
  _onTimerReset: () => void;

  constructor({
    frequency,
    onTimerReset,
  }: {
    frequency: number;
    onTimerReset: () => void;
  }) {
    this._counter = 0;
    this._frequency = frequency;
    this._start_at = 0;
    this._onTimerReset = onTimerReset;
  }

  public start() {
    this._start_at = performance.now();
  }

  public update() {
    this._counter += 1;
    if (this._frequency < performance.now() - this._start_at) {
      console.log(
        `${this._counter.toString()} animation loops per ${this._frequency.toString()} ms`,
      );
      this._onTimerReset();
      this._counter = 0;
      this._start_at = performance.now();
    }
  }
}
