#version 400

precision highp float;

attribute vec3 position;
out vec2 complex;


void main() {
	gl_Position = vec4(position.xyz, 1);
	complex = position.xy;
}
