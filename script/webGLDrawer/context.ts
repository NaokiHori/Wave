export function getWebGL2RenderingContext({
  canvas,
  contextAttributes,
}: {
  canvas: HTMLCanvasElement;
  contextAttributes: { preserveDrawingBuffer: boolean };
}): WebGL2RenderingContext {
  const gl: WebGL2RenderingContext | null = canvas.getContext("webgl2", {
    ...contextAttributes,
  });
  if (null === gl) {
    const message = "Failed to fetch WebGL2 context";
    alert(message);
    throw new Error(message);
  }
  if (!gl.getExtension("EXT_color_buffer_float")) {
    const message = "FLOAT color buffer is not supported";
    alert(message);
    throw new Error(message);
  }
  return gl;
}
