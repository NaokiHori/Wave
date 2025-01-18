#version 300 es

precision highp float;

in float v_height;

out vec4 frag_color;

void main(void) {
  float color = 0.5;
  color += 100. * v_height;
  frag_color = vec4(vec3(color), 1.);
}

