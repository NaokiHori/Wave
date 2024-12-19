#version 300 es

precision mediump float;

uniform sampler2D u_texture;

in vec2 v_texture_coordinates;

out vec4 frag_color;

vec3 value_to_color(float value) {
  value = 2. * value - 1.;
  vec3 color = vec3(0.);
  float l = 0.75;
  if (value < 0.) {
    color[0] = 0. - l * value;
  } else if (value < 0.5) {
    color[0] = 0. + 2. * value;
  } else {
    color[0] = 1.;
  }
  if (value < 0.) {
    color[1] = 0. - l * value;
  } else {
    color[1] = 0. + l * value;
  }
  if (value < -0.5) {
    color[2] = 1.;
  } else if (value < 0.) {
    color[2] = 0. - 2. * value;
  } else {
    color[2] = 0. + l * value;
  }
  return color;
}

void main(void) {
  float value = texture(u_texture, v_texture_coordinates).r;
  frag_color = vec4(value_to_color(value), 1.);
}

