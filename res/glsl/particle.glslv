#version 430 core

layout (location = 0) in vec2 pos;
layout (location = 1) in float blendFactor;
layout (location = 2) in vec4 texOffsets;
layout (location = 3) in mat4 view;

out vec2 coordsA; 
out vec2 coordsB;
out float blend;

uniform mat4 projection;
uniform float rowCount;

void main(void){
  
  vec2 texCoords = pos + vec2(0.5, 0.5);
  texCoords.y = 1.0 - texCoords.y;
  texCoords /= rowCount;
  coordsA = texCoords + texOffsets.xy;
  coordsB = texCoords + texOffsets.zw;
  blend = max(blendFactor, 0.5);
    
	gl_Position = projection * view * vec4(pos, 0.0, 1.0);

}
