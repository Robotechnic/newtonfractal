#version 400

precision highp float;

attribute vec3 position;
attribute vec2 texcoord;
out vec2 complex;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
	complex = vec2(
		texcoord.x * 2 - 1,
		texcoord.y * 2 - 1
	);
	gl_Position = Projection * Model * vec4(position, 1);
}
