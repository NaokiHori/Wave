import wbgInit, { Simulator, InitOutput, Metrics } from "../pkg/wave";
import { getCanvasElement } from "./dom";
import { CanvasDrawer } from "./canvasDrawer";
import { Timer } from "./timer";

function logging(simulator: Simulator) {
  const dt: number = simulator.get_dt();
  const metrics: Metrics = simulator.get_metrics();
  console.log(`Time-step size: ${dt.toString()}`);
  console.log("Displacement");
  console.log(`  Max: ${metrics.max_displacement.toString()}`);
  console.log(`  Min: ${metrics.min_displacement.toString()}`);
  console.log(`  Ave: ${metrics.mean_displacement.toString()}`);
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
      const canvasRect: DOMRect = canvas.getBoundingClientRect();
      const canvasAspectRatio: number = canvasRect.width / canvasRect.height;
      // prepare simulator
      const randomSeed: bigint = (function (): bigint {
        const min = 0;
        const max: number = Number.MAX_SAFE_INTEGER - 1;
        return BigInt(Math.floor(Math.random() * (max - min) + min));
      })();
      const scalarDomainSize = new Float64Array([
        canvasAspectRatio < 1 ? 1 : 2,
        canvasAspectRatio < 1 ? 2 : 1,
      ]);
      const scalarGridPoints = new Uint32Array([
        canvasAspectRatio < 1 ? 128 : 256,
        canvasAspectRatio < 1 ? 256 : 128,
      ]);
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
      const canvasDrawer = new CanvasDrawer(canvas, scalarGridPoints);
      // register on-resize event
      window.addEventListener("resize", () => {
        // adjust canvas size and paddings inside canvas accordingly
        canvasDrawer.handleWindowResize();
      });
      // rendering loop
      const timer = new Timer({
        frequency: 1000,
        onTimerReset: () => {
          logging(simulator);
        },
      });
      function updateAndDraw() {
        simulator.integrate();
        canvasDrawer.draw(wbgModule.memory.buffer, simulator.get_pos());
        timer.update();
        requestAnimationFrame(updateAndDraw);
      }
      // trigger first rendering
      timer.start();
      updateAndDraw();
    })
    .catch((error: unknown) => {
      if (error instanceof Error) {
        console.error(error);
      }
    });
});
