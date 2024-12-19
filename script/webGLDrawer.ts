import { Drawer } from "../pkg/wave";
import { initVertices } from "./webGLDrawer/vertices";
import { initVBO } from "./webGLDrawer/bufferObject";
import { initProgram } from "./webGLDrawer/program";
import vertexShaderSource from "./webGLDrawer/vertexShader.glsl?raw";
import fragmentShaderSource from "./webGLDrawer/fragmentShader.glsl?raw";

function getContext(canvas: HTMLCanvasElement): WebGL2RenderingContext {
  const gl: WebGL2RenderingContext | null = canvas.getContext("webgl2");
  if (null === gl) {
    throw new Error("failed to get context");
  }
  return gl;
}

export function isWebGLDrawerAvailable(canvas: HTMLCanvasElement): boolean {
  const gl: WebGL2RenderingContext | null = canvas.getContext("webgl2");
  return gl !== null;
}

export class WebGLDrawer {
  private _canvas: HTMLCanvasElement;
  private _gl: WebGL2RenderingContext;
  private _program: WebGLProgram;
  private _drawer: Drawer;
  private _scalarWidth: number;
  private _scalarHeight: number;
  private _scalarTexture: WebGLTexture;
  private _numberOfVertices: number;

  public constructor(canvas: HTMLCanvasElement, scalarGridPoints: Uint32Array) {
    const scalarWidth: number = scalarGridPoints[0];
    const scalarHeight: number = scalarGridPoints[1];
    const gl: WebGL2RenderingContext = getContext(canvas);
    const program: WebGLProgram = initProgram({
      gl,
      vertexShaderSource,
      fragmentShaderSource,
      transformFeedbackVaryings: [],
    });
    const positions: number[][] = initVertices(scalarWidth / scalarHeight);
    const numberOfVertices: number = positions.length;
    initVBO(
      gl,
      program,
      "a_position",
      "xy".length,
      gl.STATIC_DRAW,
      new Float32Array(positions.flat()),
    );
    // create and upload the scalar field as a texture
    const scalarTexture: WebGLTexture | null = gl.createTexture();
    if (null === scalarTexture) {
      throw new Error("failed to create a texture");
    }
    gl.bindTexture(gl.TEXTURE_2D, scalarTexture);
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.R8,
      scalarWidth,
      scalarHeight,
      0,
      gl.RED,
      gl.UNSIGNED_BYTE,
      new Uint8Array(scalarWidth * scalarHeight),
    );
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    initVBO(
      gl,
      program,
      "a_texture_coordinates",
      "xy".length,
      gl.STATIC_DRAW,
      new Float32Array([0, 1, 1, 1, 0, 0, 1, 0]),
    );
    gl.uniform1i(gl.getUniformLocation(program, "u_scalar_field"), 0);
    const drawer = new Drawer(scalarGridPoints);
    this._canvas = canvas;
    this._gl = gl;
    this._program = program;
    this._drawer = drawer;
    this._scalarWidth = scalarWidth;
    this._scalarHeight = scalarHeight;
    this._scalarTexture = scalarTexture;
    this._numberOfVertices = numberOfVertices;
  }

  public handleWindowResize() {
    const canvas: HTMLCanvasElement = this._canvas;
    const gl: WebGL2RenderingContext = this._gl;
    const program: WebGLProgram = this._program;
    const scalarAspectRatio: number = this._scalarWidth / this._scalarHeight;
    const w: number = canvas.width;
    const h: number = canvas.height;
    const canvasAspectRatio: number = w / h;
    const scale = (function computeScale() {
      return canvasAspectRatio < scalarAspectRatio
        ? [1 / scalarAspectRatio, canvasAspectRatio / scalarAspectRatio]
        : [1 / canvasAspectRatio, 1];
    })();
    gl.viewport(0, 0, w, h);
    gl.uniform2f(gl.getUniformLocation(program, "u_scale"), scale[0], scale[1]);
  }

  public draw(wasmMemory: ArrayBufferLike, ptrToPositions: number) {
    const gl: WebGL2RenderingContext = this._gl;
    const drawer: Drawer = this._drawer;
    const numberOfVertices: number = this._numberOfVertices;
    const scalarTexture: WebGLTexture = this._scalarTexture;
    const scalarWidth: number = this._scalarWidth;
    const scalarHeight: number = this._scalarHeight;
    const offsetOfPixels: number = drawer.pixelize_webgl(ptrToPositions);
    const pixels = new Uint8Array(
      wasmMemory,
      offsetOfPixels,
      scalarWidth * scalarHeight,
    );
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, scalarTexture);
    gl.texSubImage2D(
      gl.TEXTURE_2D,
      0,
      0,
      0,
      scalarWidth,
      scalarHeight,
      gl.RED,
      gl.UNSIGNED_BYTE,
      pixels,
    );
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, numberOfVertices);
  }
}
