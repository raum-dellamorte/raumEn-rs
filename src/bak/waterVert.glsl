#version 400

in vec2 position;

out vec4 clipSpace;
out vec2 textureCoords;
out vec3 toCamVec;
out vec3 fromLightVector[4];
out float visibility;

uniform mat4 playerLoc;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;
uniform vec3 camPos;
uniform vec3 lightPos[4];

const float tiling = 4.0;
const float density = 0.007;
const float gradient = 1.5;

void main(void) {
  
  vec4 worldPos = modelMatrix * vec4(position.x, 0.0, position.y, 1.0);
  vec4 posRelToCam = viewMatrix * worldPos;
  clipSpace = projectionMatrix * posRelToCam;
  gl_Position = clipSpace;
  textureCoords = vec2(position.x/2.0 + 0.5, position.y/2.0 + 0.5) * tiling;
  toCamVec = camPos - worldPos.xyz;
  for(int i = 0; i < 4; i++){
    fromLightVector[i] = worldPos.xyz - lightPos[i];
  }
  
  vec4 posRelToPlayer = playerLoc * worldPos; // for fog from player perspective
  float dist = length(posRelToPlayer.xyz) * 0.25;
  visibility = exp(-pow((dist * density), gradient));
  visibility = clamp(visibility,0.0,1.0);
 
}