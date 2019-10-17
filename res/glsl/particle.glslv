#version 400

in vec2 pos;
in mat4 view;
in vec4 tex_offsets;
in float blend_factor;

out vec2 coords_a;
out vec2 coords_b;
out float blend;

uniform mat4 projection;
uniform float row_count;

void main(void){
  
  vec2 tex_coords = pos + vec2(0.5, 0.5);
  tex_coords.y = 1.0 - tex_coords.y;
  tex_coords /= row_count;
  coords_a = tex_coords + tex_offsets.xy;
  coords_b = tex_coords + tex_offsets.zw;
  blend = blend_factor;
    
	gl_Position = projection * view * vec4(pos, 0.0, 1.0);

}
