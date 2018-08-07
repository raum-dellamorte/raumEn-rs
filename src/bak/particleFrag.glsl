#version 400

in vec2 textureCoords1;
in vec2 textureCoords2;
in float blend;

out vec4 outColor;

uniform sampler2D particleTexture;

void main(void){
    
    vec4 colour1 = texture(particleTexture, textureCoords1);
    vec4 colour2 = texture(particleTexture, textureCoords2);
	outColor = mix(colour1, colour2, blend);

}
