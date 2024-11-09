import initWasm, { Entrypoint, InitOutput, Metrics } from "../pkg/wave";

class LoopCounter {
  private _counter: number;

  public constructor() {
    this._counter = 0;
  }

  public get(): number {
    return this._counter;
  }

  public update() {
    this._counter += 1;
  }

  public reset() {
    this._counter = 0;
  }
}

interface Corner {
  left: number;
  top: number;
}

function getCanvasElement(id: string): HTMLCanvasElement {
  const canvas: HTMLElement | null = document.getElementById(id);
  if (null === canvas) {
    throw new Error(`failed to find a canvas element: ${id}`);
  }
  return canvas as HTMLCanvasElement;
}

function getContext(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
  const ctx: CanvasRenderingContext2D | null = canvas.getContext("2d", {
    alpha: false,
  });
  if (null === ctx) {
    throw new Error("failed to get context");
  }
  return ctx;
}

function adjustCanvasSize(
  scalarGridPoints: Uint32Array,
  canvas: HTMLCanvasElement,
): Corner {
  const scalarWidth: number = scalarGridPoints[0];
  const scalarHeight: number = scalarGridPoints[1];
  // define aspect ratio as "width / height"
  const scalarAspectRatio: number = scalarWidth / scalarHeight;
  const canvasRect: DOMRect = canvas.getBoundingClientRect();
  const canvasAspectRatio: number = canvasRect.width / canvasRect.height;
  if (canvasAspectRatio < scalarAspectRatio) {
    // fit width (padding / margin vertically)
    const canvasWidth: number = scalarWidth;
    const canvasHeight: number = scalarWidth / canvasAspectRatio;
    const left = 0;
    const top: number = 0.5 * (canvasHeight - scalarHeight);
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;
    return { left, top };
  } else {
    // fit height (padding / margin horizontally)
    const canvasHeight: number = scalarHeight;
    const canvasWidth: number = scalarHeight * canvasAspectRatio;
    const left: number = 0.5 * (canvasWidth - scalarWidth);
    const top = 0;
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;
    return { left, top };
  }
}

window.addEventListener("load", () => {
  initWasm()
    .then((wasm: InitOutput) => {
      // simulation parameters
      const param_c2 = 5e-3;
      const param_nu = 1e-4;
      // prepare destination
      const canvas: HTMLCanvasElement = getCanvasElement("my-canvas");
      const ctx: CanvasRenderingContext2D = getContext(canvas);
      // decide domain aspect ratio based on the initial user screen size
      const canvasRect: DOMRect = canvas.getBoundingClientRect();
      const canvasAspectRatio: number = canvasRect.width / canvasRect.height;
      // simulation domain
      const scalarDomainSize = new Float64Array([
        canvasAspectRatio < 1 ? 1 : 2,
        canvasAspectRatio < 1 ? 2 : 1,
      ]);
      const scalarGridPoints = new Uint32Array([
        canvasAspectRatio < 1 ? 128 : 512,
        canvasAspectRatio < 1 ? 256 : 256,
      ]);
      // random seed
      const randomSeed: bigint = (function (): bigint {
        const min = 0;
        const max: number = Number.MAX_SAFE_INTEGER - 1;
        return BigInt(Math.floor(Math.random() * (max - min) + min));
      })();
      // prepare wasm
      const dt_max = 5e-2;
      const entrypoint = Entrypoint.new(
        randomSeed,
        scalarDomainSize,
        scalarGridPoints,
        param_c2,
        param_nu,
        dt_max,
      );
      const pixels = new Uint8Array(
        wasm.memory.buffer,
        entrypoint.pixels(),
        scalarGridPoints[0] * scalarGridPoints[1] * "rgba".length,
      );
      let corner: Corner = adjustCanvasSize(scalarGridPoints, canvas);
      // enforce scalar field being fit to the canvas
      window.addEventListener("resize", () => {
        corner = adjustCanvasSize(scalarGridPoints, canvas);
      });
      // loop counter
      const maxCounterValue = 100;
      const loopCounter = new LoopCounter();
      // animation kernel
      function updateAndDraw() {
        // integrate in time
        entrypoint.update();
        // construct a new ImageData object from wasm linear memory
        const imageData = new ImageData(
          new Uint8ClampedArray(pixels),
          scalarGridPoints[0],
          scalarGridPoints[1],
        );
        // draw newly-generated ImageData to context
        ctx.putImageData(
          imageData,
          Math.floor(corner.left),
          Math.floor(corner.top),
        );
        loopCounter.update();
        if (maxCounterValue < loopCounter.get()) {
          loopCounter.reset();
          const dt: number = entrypoint.get_dt();
          const metrics: Metrics = entrypoint.get_metrics();
          console.log(`Time-step size: ${dt.toString()}`);
          console.log("Displacement");
          console.log(`  Max: ${metrics.max_displacement.toString()}`);
          console.log(`  Min: ${metrics.min_displacement.toString()}`);
          console.log(`  Ave: ${metrics.mean_displacement.toString()}`);
        }
        // set myself as the callback
        requestAnimationFrame(updateAndDraw);
      }
      // trigger first animation flow
      updateAndDraw();
    })
    .catch(() => {
      throw new Error("failed to initialize wasm wrapper");
    });
});
