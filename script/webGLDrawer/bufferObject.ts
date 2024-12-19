export function initVBO(
  gl: WebGL2RenderingContext,
  program: WebGLProgram,
  attributeName: string,
  stride: GLint,
  usage: GLenum,
  values: Float32Array,
): WebGLBuffer {
  const attributeIndex: GLint = gl.getAttribLocation(program, attributeName);
  const vbo: WebGLBuffer | null = gl.createBuffer();
  if (null === vbo) {
    throw new Error("createBuffer failed");
  }
  gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
  gl.bufferData(gl.ARRAY_BUFFER, values, usage);
  gl.enableVertexAttribArray(attributeIndex);
  gl.vertexAttribPointer(attributeIndex, stride, gl.FLOAT, false, 0, 0);
  gl.bindBuffer(gl.ARRAY_BUFFER, null);
  return vbo;
}
