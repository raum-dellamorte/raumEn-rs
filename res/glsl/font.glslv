#version 400

in vec2 a_Pos;
in vec2 a_TexCoord;

out vec2 v_texCoords;
out vec2 v_texCoords2;

uniform vec2 translation;
uniform vec2 offset;

void main(void){
  
  gl_Position = vec4(a_Pos + translation * vec2(2.0, -2.0), 0.0, 1.0);
  
  v_texCoords = a_TexCoord;
  v_texCoords2 = a_TexCoord + offset;
  
}