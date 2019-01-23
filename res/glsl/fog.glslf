#version 400

in vec2 v_TexCoord;

out vec4 out_Color;

uniform sampler2D color_texture;
uniform sampler2D depth_map;

void main(void){

  vec4 tex = texture(color_texture,v_TexCoord);
  float depth = texture(depth_map,v_TexCoord).r;
  float near = 0.1;
  float far = 1000.0;
  float d = 2.0 * near * far / (far + near - (2.0 * depth - 1.0) * (far - near));
  d = d * 0.01;
  d = clamp(d,0.0,1.0);
  float x = v_TexCoord.x;
  out_Color = mix(tex, vec4(0.0,0.0,0.0,1.0), d);

}