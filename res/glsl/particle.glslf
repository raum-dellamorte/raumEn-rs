#version 400

in vec2 coords_a;
in vec2 coords_b;
in float blend;

out vec4 outColor;

uniform sampler2D skin;

void main(void){
    
  vec4 colour1 = texture(skin, coords_a);
  vec4 colour2 = texture(skin, coords_b);
	
  colour1.g = 1.0;
  colour2.r = 1.0;
  
  outColor = mix(colour1, colour2, blend);

}
