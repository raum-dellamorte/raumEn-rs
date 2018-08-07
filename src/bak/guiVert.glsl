#version 400

in vec2 position;

out vec2 textureCoords;

uniform mat4 transformationMatrix;

uniform float numOfRows;
uniform vec2 offset;
uniform float flipYAxis;

const mat4 flipY = mat4(1.0,  0.0, 0.0, 0.0, 
                        0.0, -1.0, 0.0, 0.0, 
                        0.0,  0.0, 1.0, 0.0, 
                        0.0,  0.0, 0.0, 1.0);

void main(void){

  gl_Position = transformationMatrix * vec4(position, 0.0, 1.0);
  
  if (flipYAxis > 0.5) {
    gl_Position = gl_Position * flipY;
  }
  
  textureCoords = vec2((position.x+1.0)/2.0, 1 - (position.y+1.0)/2.0);
  textureCoords = (textureCoords / numOfRows) + offset;
}