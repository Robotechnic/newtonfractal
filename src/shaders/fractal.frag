#version 300 es

#define cx_mul(a, b) vec2(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x)
#define cx_div(a, b) vec2(((a.x*b.x+a.y*b.y)/(b.x*b.x+b.y*b.y)),((a.y*b.x-a.x*b.y)/(b.x*b.x+b.y*b.y)))

precision highp float;
in vec2 complex;
out vec4 fragColor;

uniform int maxIterations;

uniform vec2 root0;
uniform vec2 root1;
uniform vec2 root2;

uniform vec3 color0;
uniform vec3 color1;
uniform vec3 color2;

uniform vec2 dcoeff0;
uniform vec2 dcoeff1;
uniform vec2 dcoeff2;

vec2 cx_pow(vec2 a, float n) {
    float angle = atan(a.y, a.x);
    float r = length(a);
    float real = pow(r, n) * cos(n*angle);
    float im = pow(r, n) * sin(n*angle);
    return vec2(real, im);
}

void closestRoot(in vec2 z, out vec4 color) {
	float d1 = distance(z, root0);
	float d2 = distance(z, root1);
	float d3 = distance(z, root2);
	if (d1 < d2 && d1 < d3) {
		color = vec4(color0, 1.0);
	} else if (d2 < d1 && d2 < d3) {
		color = vec4(color1, 1.0);
	} else {
		color = vec4(color2, 1.0);
	}
}

vec2 evaluate_polynomial(in vec2 z) {
	return cx_mul(cx_mul((z - root0), (z - root1)), (z - root2));
}

vec2 evaluate_derivative(in vec2 z) {
	return cx_mul(dcoeff0, cx_pow(z, 2.)) + cx_mul(dcoeff1, z) + dcoeff2;
}

void newton(inout vec2 z) {
	z = z - cx_div(evaluate_polynomial(z), evaluate_derivative(z));
}

void iterate(inout vec2 z) {
	for (int i = 0; i < maxIterations; i++) {
		newton(z);
	}
}

void main() {
	vec2 z = complex;
	iterate(z);
	closestRoot(z, fragColor);
}
