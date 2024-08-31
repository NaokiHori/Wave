import initWasm, { Entrypoint, InitOutput } from "../pkg/wave";

// simulation domain
const scalarDomainSize = new Float64Array([1.0, 1.0]);
const scalarGridPoints = new Uint32Array([128, 128]);
// simulation parameters
const param_c2 = 1e-1;
const param_nu = 1e-4;

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

function adjustCanvasSize(canvas: HTMLCanvasElement): Corner {
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
      // prepare destination
      const canvas: HTMLCanvasElement = getCanvasElement("my-canvas");
      const ctx: CanvasRenderingContext2D = getContext(canvas);
      // prepare wasm
      const entrypoint = Entrypoint.new(
        scalarDomainSize,
        scalarGridPoints,
        param_c2,
        param_nu,
      );
      const pixels = new Uint8Array(
        wasm.memory.buffer,
        entrypoint.pixels(),
        scalarGridPoints[0] * scalarGridPoints[1] * "rgba".length,
      );
      let corner: Corner = adjustCanvasSize(canvas);
      // enforce scalar field being fit to the canvas
      window.addEventListener("resize", () => {
        corner = adjustCanvasSize(canvas);
      });
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
