#version 400

in vec3 a_Pos;
in vec2 a_TexCoord;
in vec3 a_Norm;

out vec2 v_TexCoord;
out vec3 v_SurfaceNorm;
out vec3 v_toLight;
out vec3 v_toCam;
out float v_vis;

uniform mat4 u_Transform;
uniform mat4 u_Projection;
uniform mat4 u_View;

uniform vec3 light_pos;
// uniform float use_fake_lighting;

uniform float row_count;
uniform vec2 offset;

const float density = 0.007;
const float gradient = 1.5;

void main(void) {
  vec4 worldPos = u_Transform * vec4(a_Pos, 1.0);
  
  vec4 posRelToCam = u_View * worldPos; // for fog from cam perspective
  gl_Position = u_Projection * posRelToCam;
  v_TexCoord = (a_TexCoord / row_count) + offset;
  
  // vec3 actualNormal = a_Norm;
  // if(use_fake_lighting > 0.5){
  //   actualNormal = vec3(0.0,1.0,0.0);
  // }
  
  v_SurfaceNorm = (u_Transform * vec4(a_Norm, 0.0)).xyz;
  v_toLight = light_pos - worldPos.xyz;
  
  v_toCam = (inverse(u_View) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
  
  // vec4 posRelToPlayer = player_loc * worldPos; // for fog from player perspective
  float dist = length(posRelToCam.xyz);
  v_vis = clamp(exp(-pow((dist * density), gradient)),0.5,1.0);
  
}
