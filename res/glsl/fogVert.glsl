#version 400

in vec2 position;

out vec2 textureCoords;

uniform vec3 playerPos;

void main(void) {
  gl_Position = vec4(position, 0.0, 1.0);
  textureCoords = position * 0.5 + 0.5;
}