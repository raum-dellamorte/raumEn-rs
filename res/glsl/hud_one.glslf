#version 400

in vec2 v_TexCoord;

out vec4 out_Color;

uniform sampler2D tex;

void main(void){
  out_Color = texture(tex,v_TexCoord);
}