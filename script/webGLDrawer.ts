import { getWebGL2RenderingContext } from "./webGLDrawer/context";
import { Program } from "./webGLDrawer/program";
import { Texture } from "./webGLDrawer/texture";
import { IndexBufferObject } from "./webGLDrawer/indexBufferObject";
import { setupStaticallyDrawnData } from "./webGLDrawer/setupStaticallyDrawnData";
import { setUniform, setUniformMatrix } from "./webGLDrawer/uniform";
import { Matrix44 } from "./webGLDrawer/matrix44";
import { Vector3 } from "./webGLDrawer/vector3";
import vertexShaderSource from "./webGLDrawer/vertexShader.glsl?raw";
import fragmentShaderSource from "./webGLDrawer/fragmentShader.glsl?raw";

export class WebGLDrawer {
  private _canvas: HTMLCanvasElement;
  private _gl: WebGL2RenderingContext;
  private _program: Program;
  // xy positions
  private _indexBufferObject: IndexBufferObject;
  private _scalarHeightTexture: Texture;
  private _scalarGridPoints: [number, number];
  private _aspectRatio: number;

  public constructor(
    canvas: HTMLCanvasElement,
    scalarDomainSize: Float32Array,
    scalarGridPoints: Uint32Array,
  ) {
    const gl: WebGL2RenderingContext = getWebGL2RenderingContext({
      canvas,
      contextAttributes: {
        preserveDrawingBuffer: false,
      },
    });
    const coordinates = new Float32Array(
      "xy".length * scalarGridPoints[0] * scalarGridPoints[1],
    );
    for (let j = 0; j < scalarGridPoints[1]; j++) {
      for (let i = 0; i < scalarGridPoints[0]; i++) {
        coordinates["xy".length * (j * scalarGridPoints[0] + i) + 0] =
          (0.5 * (2 * i + 1) * scalarDomainSize[0]) / scalarGridPoints[0] -
          0.5 * scalarDomainSize[0];
        coordinates["xy".length * (j * scalarGridPoints[0] + i) + 1] =
          (0.5 * (2 * j + 1) * scalarDomainSize[1]) / scalarGridPoints[1] -
          0.5 * scalarDomainSize[1];
      }
    }
    const program = new Program({
      gl,
      vertexShaderSource,
      fragmentShaderSource,
      transformFeedbackVaryings: [],
    });
    program.use({
      gl,
      callback: (webGLProgram: WebGLProgram) => {
        setUniform({
          gl,
          program: webGLProgram,
          dataType: "INT32",
          uniformName: "u_scalar_grid",
          data: [scalarGridPoints[0], scalarGridPoints[1]],
        });
      },
    });
    const scalarHeightTexture: Texture = program.use({
      gl,
      callback: (webGLProgram: WebGLProgram) => {
        const scalarHeightTexture = new Texture({
          gl,
          program: webGLProgram,
          textureTarget: gl.TEXTURE_2D,
          textureUnit: 0,
          textureName: "u_scalar_height",
        });
        scalarHeightTexture.bindAndExecute({
          gl,
          callback: (boundTexture: Texture) => {
            gl.texStorage2D(
              boundTexture.textureTarget,
              1,
              gl.R32F,
              scalarGridPoints[0],
              scalarGridPoints[1],
            );
            gl.texParameteri(
              boundTexture.textureTarget,
              gl.TEXTURE_MIN_FILTER,
              gl.NEAREST,
            );
            gl.texParameteri(
              boundTexture.textureTarget,
              gl.TEXTURE_MAG_FILTER,
              gl.NEAREST,
            );
          },
        });
        return scalarHeightTexture;
      },
    });
    program.use({
      gl,
      callback: (webGLProgram: WebGLProgram) => {
        setupStaticallyDrawnData({
          gl,
          program: webGLProgram,
          attributeName: "a_position",
          numberOfVertices: scalarGridPoints[0] * scalarGridPoints[1],
          numberOfItemsForEachVertex: "xy".length,
          data: coordinates,
        });
      },
    });
    const indexBufferObject: IndexBufferObject = program.use({
      gl,
      callback: () => {
        const indices = new Int16Array(
          (scalarGridPoints[0] - 1) * (scalarGridPoints[1] - 1) * 6,
        );
        for (let j = 0; j < scalarGridPoints[1] - 1; j++) {
          for (let i = 0; i < scalarGridPoints[0] - 1; i++) {
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 0] =
              (j + 0) * scalarGridPoints[0] + (i + 0);
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 1] =
              (j + 0) * scalarGridPoints[0] + (i + 1);
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 2] =
              (j + 1) * scalarGridPoints[0] + (i + 0);
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 3] =
              (j + 0) * scalarGridPoints[0] + (i + 1);
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 4] =
              (j + 1) * scalarGridPoints[0] + (i + 1);
            indices[6 * (j * (scalarGridPoints[0] - 1) + i) + 5] =
              (j + 1) * scalarGridPoints[0] + (i + 0);
          }
        }
        const indexBufferObject = new IndexBufferObject({
          gl,
          size: (scalarGridPoints[0] - 1) * (scalarGridPoints[1] - 1) * 6,
          usage: gl.STATIC_DRAW,
        });
        indexBufferObject.bindAndExecute({
          gl,
          callback: (boundBuffer: IndexBufferObject) => {
            boundBuffer.updateData({ gl, data: indices });
          },
        });
        return indexBufferObject;
      },
    });
    this._canvas = canvas;
    this._gl = gl;
    this._program = program;
    this._scalarHeightTexture = scalarHeightTexture;
    this._scalarGridPoints = [scalarGridPoints[0], scalarGridPoints[1]];
    this._indexBufferObject = indexBufferObject;
    this._aspectRatio = canvas.width / canvas.height;
  }

  public handleWindowResize() {
    const canvas: HTMLCanvasElement = this._canvas;
    const gl: WebGL2RenderingContext = this._gl;
    const program: Program = this._program;
    const canvasWidth: number = canvas.width;
    const canvasHeight: number = canvas.height;
    program.use({
      gl,
      callback: () => {
        gl.viewport(0, 0, canvasWidth, canvasHeight);
      },
    });
    this._aspectRatio = canvasWidth / canvasHeight;
  }

  public draw(
    wasmMemory: ArrayBufferLike,
    scalarHeight: number,
    rotationMatrix: Matrix44,
  ) {
    const gl: WebGL2RenderingContext = this._gl;
    const program: Program = this._program;
    const indexBufferObject: IndexBufferObject = this._indexBufferObject;
    const scalarHeightTexture: Texture = this._scalarHeightTexture;
    const scalarGridPoints: [number, number] = this._scalarGridPoints;
    const modelMatrix: Matrix44 = rotationMatrix;
    const cameraPosition = new Vector3({
      x: 0,
      y: 0,
      z: 3,
    });
    const viewMatrix: Matrix44 = new Matrix44({
      type: "translate",
      offset: cameraPosition.multiply(-1),
    });
    const perspectiveMatrix = new Matrix44({
      type: "perspective",
      fieldOfView: (1 / 192) * Math.PI,
      aspectRatio: this._aspectRatio,
      near: cameraPosition.norm(),
      far: cameraPosition.norm() * 2,
    });
    program.use({
      gl,
      callback: (webGLProgram: WebGLProgram) => {
        setUniformMatrix({
          gl,
          program: webGLProgram,
          uniformName: "u_mvp_matrix",
          data: perspectiveMatrix
            .matmul(viewMatrix)
            .matmul(modelMatrix)
            .transpose()
            .flat(),
        });
        gl.clearColor(0, 0, 0, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.enable(gl.DEPTH_TEST);
        scalarHeightTexture.bindAndExecute({
          gl,
          callback: (boundScalarHeightTexture: Texture) => {
            gl.texSubImage2D(
              boundScalarHeightTexture.textureTarget,
              0,
              0,
              0,
              scalarGridPoints[0],
              scalarGridPoints[1],
              gl.RED,
              gl.FLOAT,
              new Float32Array(
                wasmMemory,
                scalarHeight,
                scalarGridPoints[0] * scalarGridPoints[1],
              ),
            );
            indexBufferObject.bindAndExecute({
              gl,
              callback: (boundBuffer: IndexBufferObject) => {
                boundBuffer.draw({ gl, mode: gl.TRIANGLES });
              },
            });
          },
        });
      },
    });
  }
}
