#version 400
in vec3 position;

uniform mat4 transformationMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;

const mat4 flipY = mat4(1.0,  0.0, 0.0, 0.0, 
                        0.0, -1.0, 0.0, 0.0, 
                        0.0,  0.0, 1.0, 0.0, 
                        0.0,  0.0, 0.0, 1.0);

void main(void) {
  gl_Position = projectionMatrix * viewMatrix * transformationMatrix * vec4(position, 1.0) * flipY;
}
