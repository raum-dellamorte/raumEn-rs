#version 430 core

in vec2 coordsA;
in vec2 coordsB;
in float blend;

out vec4 outColor;

uniform sampler2D skin;

void main(void){
    
  vec4 colour1 = texture(skin, coordsA);
  vec4 colour2 = texture(skin, coordsB);
	
  colour1.g = 1.0;
  colour2.r = 1.0;
  
  outColor = mix(colour1, colour2, blend);
  outColor = mix(outColor, vec4(1.0,1.0,1.0,1.0), 0.5);
  outColor.a = 1.0;

}
