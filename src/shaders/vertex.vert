#version 400

precision highp float;

attribute vec3 position;
attribute vec2 texcoord;
out vec2 complex;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec2 realRange;
uniform vec2 imagRange;

void main() {
	complex = vec2(
		texcoord.x * (realRange.y - realRange.x) + realRange.x,
		texcoord.y * (imagRange.y - imagRange.x) + imagRange.x
	);
	gl_Position = Projection * Model * vec4(position, 1);
}
