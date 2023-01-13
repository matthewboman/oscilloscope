#version 140

const int SIZE = 1024;
const float SIZE_F = 1023.;

uniform vec2 windowSize;
uniform vec2 mouse;
out vec4 color;

layout(std140) uniform Buffer {
  vec4 array[SIZE/4];
};

float get(int ind) {
  return array[ind >> 2][ind & 3];
}

float dist_segment(vec2 line, vec2 point) {
  float t = dot(line, point) / dot(line, line);
  if (t > 1.) t = 1.;
  if (t < 0.) t = 0.;
  return distance(point, line*t);
}

const vec3 bgcolor = vec3(0.0022, 0.0074, 0.0050);
const vec3 trcolor = vec3(0.262, 1.000, 0.643);

void main() {
  int idx = int(gl_FragCoord.x/windowSize.x*SIZE_F);
  float d = 1e10;
  for (int i = max(0, idx-2); i <= min(SIZE-2, idx+2); ++i) {
    float val_0 = get(i);
    float val_1 = get(i+1);
    vec2 A = vec2(float(i)*windowSize.x/SIZE_F, (val_0+1.)*windowSize.y/2.);
    vec2 B = vec2(float(i+1)*windowSize.x/SIZE_F, (val_1+1.)*windowSize.y/2.);
    d = min(d, dist_segment(B-A, gl_FragCoord.xy-A));
  }
  vec3 gcolor = trcolor*exp(-d*d/5.) + bgcolor*(1-exp(-d*d/5.));
  color = vec4(gcolor, 1.);
}

//vim: sw=2 ts=2 et:
