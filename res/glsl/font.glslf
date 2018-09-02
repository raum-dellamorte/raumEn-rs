#version 400

in vec2 v_texCoords;
in vec2 v_texCoords2;

out vec4 out_Color;

uniform vec3 colour;
uniform sampler2D fontAtlas;

const float width = 0.50;
const float edge = 0.05;
const float widthBorder = 0.4;
const float edgeBorder = 0.45;

const vec3 colourBorder = vec3(1.0, 0.0, 0.5);

void main(void){
  
  float dist = 1.0 - texture(fontAtlas, v_texCoords).a;
  float alpha = 1.0 - smoothstep(width, width + edge, dist);
  float dist2 = 1.0 - texture(fontAtlas, v_texCoords2).a;
  float alpha2 = 1.0 - smoothstep(width, widthBorder + edgeBorder, dist2);
  float alphaBorder = 1.0 - smoothstep(widthBorder, widthBorder + edgeBorder, dist2);
  
  float alphaOut = alpha + (1.0 - alpha) * alphaBorder;
  vec3 colourOut = mix(colourBorder, colour, (alpha) / (alphaOut));
  
  out_Color = vec4(colourOut, alpha2);
  
}