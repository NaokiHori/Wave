import { Drawer } from "../pkg/wave";

interface Corner {
  left: number;
  top: number;
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
  canvas: HTMLCanvasElement,
  scalarGridPoints: Uint32Array,
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

export class CanvasDrawer {
  private _canvas: HTMLCanvasElement;
  private _ctx: CanvasRenderingContext2D;
  private _drawer: Drawer;
  private _scalarGridPoints: Uint32Array;
  private _corner: Corner;

  public constructor(canvas: HTMLCanvasElement, scalarGridPoints: Uint32Array) {
    const ctx: CanvasRenderingContext2D = getContext(canvas);
    const drawer = new Drawer(scalarGridPoints);
    this._canvas = canvas;
    this._ctx = ctx;
    this._drawer = drawer;
    this._scalarGridPoints = scalarGridPoints;
    this._corner = adjustCanvasSize(canvas, scalarGridPoints);
  }

  public handleWindowResize() {
    this._corner = adjustCanvasSize(this._canvas, this._scalarGridPoints);
  }

  public draw(wasmMemory: ArrayBufferLike, ptrToPositions: number) {
    const ctx: CanvasRenderingContext2D = this._ctx;
    const drawer: Drawer = this._drawer;
    const scalarGridPoints: Uint32Array = this._scalarGridPoints;
    const corner: Corner = this._corner;
    const offsetOfPixels: number = drawer.pixelize(ptrToPositions);
    // create a new view to the pixels
    const pixels = new Uint8Array(
      wasmMemory,
      offsetOfPixels,
      scalarGridPoints[0] * scalarGridPoints[1] * "rgba".length,
    );
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
  }
}
