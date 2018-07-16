#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;
uniform sampler2D t_Texture;

void main() {
  vec4 tex = texture(t_Texture, v_TexCoord);
  float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
  Target0 = tex;
}
