#version 400

in vec3 position;
in vec2 textureCoordinates;
in vec3 normal;

out vec2 pass_textureCoordinates;
out vec3 surfaceNormal;
out vec3 toLightVector[4];
out vec3 toCameraVector;
out float visibility;

uniform mat4 playerLoc;
uniform mat4 transformationMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform vec3 lightPosition[4];

uniform vec4 plane;
uniform float useClipPlane;

const float density = 0.007;
const float gradient = 1.5;

void main(void) {
  vec4 worldPos = transformationMatrix * vec4(position, 1.0);
  
  if(useClipPlane > 0.5) {
    gl_ClipDistance[0] = dot(worldPos, plane);
  }
  
  vec4 posRelToCam = viewMatrix * worldPos; // for fog from cam perspective
  gl_Position = projectionMatrix * posRelToCam;
  pass_textureCoordinates = textureCoordinates;
  
  surfaceNormal = (transformationMatrix * vec4(normal, 0.0)).xyz;
  for(int i = 0; i < 4; i++){
    toLightVector[i] = lightPosition[i] - worldPos.xyz;
  }
  toCameraVector = (inverse(viewMatrix) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
  
  vec4 posRelToPlayer = playerLoc * worldPos; // for fog from player perspective
  float dist = length(posRelToPlayer.xyz);
  visibility = exp(-pow((dist * density), gradient));
  visibility = clamp(visibility,0.0,1.0);
  
}