#version 400

in vec3 a_Pos;
in vec3 a_Norm;
in vec2 a_TexCoord;

out vec2 v_TexCoord;
out vec3 v_SurfaceNorm;
out vec3 v_toLight[4];
out vec3 v_toCam;
out float v_vis;

// layout (std140)
// uniform Locals {
// 	mat4 u_Transform;
// };

uniform mat4 u_Transform;
uniform mat4 u_Projection;
uniform mat4 u_View;

uniform mat4 playerLoc;
uniform vec3 lightPosition[4];
uniform float useFakeLighting;

uniform float numOfRows;
uniform vec2 offset;

uniform vec4 plane;
uniform float useClipPlane;

const float density = 0.007;
const float gradient = 1.5;

void main(void) {
  vec4 worldPos = u_Transform * vec4(a_Pos, 1.0);
  
  if(useClipPlane > 0.5) {
    gl_ClipDistance[0] = dot(worldPos, plane);
  }
  
  vec4 posRelToCam = u_View * worldPos; // for fog from cam perspective
  gl_Position = u_Projection * posRelToCam;
  v_TexCoord = (a_TexCoord / numOfRows) + offset;
  
  vec3 actualNormal = a_Norm;
  if(useFakeLighting > 0.5){
    actualNormal = vec3(0.0,1.0,0.0);
  }
  
  v_SurfaceNorm = (u_Transform * vec4(actualNormal, 0.0)).xyz;
  for(int i = 0; i < 4; i++){
    v_toLight[i] = lightPosition[i] - worldPos.xyz;
  }
  v_toCam = (inverse(u_View) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
  
  vec4 posRelToPlayer = playerLoc * worldPos; // for fog from player perspective
  float dist = length(posRelToPlayer.xyz);
  v_vis = exp(-pow((dist * density), gradient));
  v_vis = clamp(v_vis,0.0,1.0);
  
}
