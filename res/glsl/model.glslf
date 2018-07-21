#version 150

in vec3 v_Color;
//in vec2 v_TexCoord;
out vec4 out_Color;
//uniform sampler2D t_Texture;

void main() {
  // vec4 tex = texture(t_Texture, v_TexCoord);
  // float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
  out_Color = vec4(v_Color, 1.0);
}
