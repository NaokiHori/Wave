import wbgInit, { Simulator, InitOutput, Metrics } from "../pkg/wave";
import { getCanvasElement, syncCanvasSize } from "./dom";
import { WebGLDrawer } from "./webGLDrawer";
import { Matrix44 } from "./webGLDrawer/matrix44";
import { Vector3 } from "./webGLDrawer/vector3";
import { Timer } from "./timer";
import { Counter } from "./counter";

function logging(simulator: Simulator) {
  const dt: number = simulator.get_dt();
  const metrics: Metrics = simulator.get_metrics();
  console.log(`Time-step size: ${dt.toString()}`);
  console.log("Displacement");
  console.log(`  Max: ${metrics.max_displacement.toExponential(3)}`);
  console.log(`  Min: ${metrics.min_displacement.toExponential(3)}`);
  console.log(`  Ave: ${metrics.mean_displacement.toExponential(3)}`);
}

window.addEventListener("load", () => {
  wbgInit()
    .then((wbgModule: InitOutput) => {
      // simulation parameters
      const param_c2 = 5e-3;
      const param_nu = 1e-4;
      // prepare destination
      const canvas: HTMLCanvasElement = getCanvasElement("my-canvas");
      // decide domain aspect ratio based on the initial user screen size
      syncCanvasSize(canvas);
      // prepare simulator
      const randomSeed: bigint = (function (): bigint {
        const min = 0;
        const max: number = Number.MAX_SAFE_INTEGER - 1;
        return BigInt(Math.floor(Math.random() * (max - min) + min));
      })();
      const scalarDomainSize = new Float32Array([1, 1]);
      const scalarGridPoints = new Uint32Array([256, 256]);
      const dt_max = 5e-2;
      const simulator = new Simulator(
        randomSeed,
        scalarDomainSize,
        scalarGridPoints,
        param_c2,
        param_nu,
        dt_max,
      );
      // initialize drawer
      const drawer = new WebGLDrawer(
        canvas,
        scalarDomainSize,
        scalarGridPoints,
      );
      window.addEventListener("resize", () => {
        syncCanvasSize(canvas);
        drawer.handleWindowResize();
      });
      // rendering loop
      const timer = new Timer({
        frequency: 1000,
        onTimerReset: () => {
          logging(simulator);
        },
      });
      const counter = new Counter();
      function updateAndDraw() {
        simulator.integrate();
        const xRotationMatrix = new Matrix44({
          type: "rotate",
          angle: 0.4 * Math.PI,
          vector: new Vector3({ x: -1, y: 0, z: 0 }),
        });
        const zRotationMatrix = new Matrix44({
          type: "rotate",
          angle: 0.005 * counter.get(),
          vector: new Vector3({ x: 0, y: 0, z: -1 }),
        });
        const rotationMatrix = xRotationMatrix.matmul(zRotationMatrix);
        drawer.draw(
          wbgModule.memory.buffer,
          simulator.get_pos(),
          rotationMatrix,
        );
        counter.update();
        timer.update();
        requestAnimationFrame(updateAndDraw);
      }
      // trigger first rendering
      timer.start();
      drawer.handleWindowResize();
      updateAndDraw();
    })
    .catch((error: unknown) => {
      if (error instanceof Error) {
        console.error(error);
      }
    });
});
