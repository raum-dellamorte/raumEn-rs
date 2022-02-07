#version 400

in vec2 v_TexCoord;

out vec4 out_Color;

uniform sampler2D bg_color;
uniform sampler2D fg_color;

void main(void){
  vec4 bg = texture(bg_color,v_TexCoord);
  vec4 fg = texture(fg_color,v_TexCoord);
  // FixMe: Would you just look at this hack.
  float blendFactor;
  if (fg.r > 0.5) {
    blendFactor = 1.0; }
  else {
    blendFactor = 0.0;
  };
  out_Color = mix(bg,fg,blendFactor);
}