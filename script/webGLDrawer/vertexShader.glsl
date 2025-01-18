#version 300 es

precision highp float;
precision highp sampler2D;

uniform mat4 u_mvp_matrix;
uniform ivec2 u_scalar_grid;
uniform sampler2D u_scalar_height;

in vec2 a_position;

out float v_height;

void main(void) {
  int i = gl_VertexID % u_scalar_grid[0];
  int j = gl_VertexID / u_scalar_grid[0];
  float z = texelFetch(u_scalar_height, ivec2(i, j), 0).r;
  v_height = z;
  gl_Position = u_mvp_matrix * vec4(a_position, 5. * z, 1.);
}

