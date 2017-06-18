#version 400

in vec2 position;
in vec2 textureCoords;

out vec2 pass_textureCoords;
out vec2 pass_textureCoords2;

uniform vec2 translation;
uniform vec2 offset;

void main(void){
  
  gl_Position = vec4(position + translation * vec2(2.0, -2.0), 0.0, 1.0);
  
  pass_textureCoords = textureCoords;
  pass_textureCoords2 = textureCoords + offset;
  
}