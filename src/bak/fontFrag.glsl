#version 400

in vec2 pass_textureCoords;
in vec2 pass_textureCoords2;

out vec4 out_Color;

uniform vec3 colour;
uniform sampler2D fontAtlas;

const float width = 0.55;
const float edge = 0.1;
const float widthBorder = 0.5;
const float edgeBorder = 0.6;

const vec3 colourBorder = vec3(1.0, 0.0, 0.5);

void main(void){
  
  float distance = 1.0 - texture(fontAtlas, pass_textureCoords).a;
  float alpha = 1.0 - smoothstep(width, width + edge, distance);
  float distance2 = 1.0 - texture(fontAtlas, pass_textureCoords2).a;
  float alpha2 = 1.0 - smoothstep(width, widthBorder + edgeBorder, distance2);
  float alphaBorder = 1.0 - smoothstep(widthBorder, widthBorder + edgeBorder, distance2);
  
  float alphaOut = alpha + (1.0 - alpha) * alphaBorder;
  vec3 colourOut = mix(colourBorder, colour, (alpha) / (alphaOut));
  
  out_Color = vec4(colourOut, alpha2);
  
}