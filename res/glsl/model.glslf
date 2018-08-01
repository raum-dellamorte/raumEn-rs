#version 150

in vec2 v_TexCoord;
in vec3 v_SurfaceNorm;
in vec3 v_toLight;
// in vec3 v_toCam;
in float v_vis;

out vec4 out_Color;

uniform sampler2D t_Texture;
uniform vec3 sky_color;
// uniform float shine_damper;
// uniform float reflectivity;

uniform vec3 light_color;
// uniform vec3 attenuation;

void main(void) {
  vec3 unitNormal = normalize(v_SurfaceNorm);
  // vec3 unitCameraVector = normalize(v_toCam);
  vec3 unitLightVector = normalize(v_toLight);
  
  // float distance = length(v_toLight);
  // float attFactor = attenuation.x + (attenuation.y * distance) + (attenuation.z * distance * distance);
  float nDotl = dot(unitNormal, unitLightVector);
  float brightness = max(nDotl, 0.0);
  vec3 diffuse = brightness * light_color;
  // vec3 lightDirection = -unitLightVector;
  // vec3 reflectedLightDirection = reflect(lightDirection, unitNormal);
  // float specularFactor = dot(reflectedLightDirection, unitCameraVector);
  // specularFactor = max(specularFactor, 0.0);
  // float dampedFactor = pow(specularFactor, shine_damper);
  // vec3 totalDiffuse = max((brightness * light_color) / attFactor, 0.2);
  // vec3 totalSpecular = (dampedFactor * reflectivity * light_color) / attFactor;
  
  vec4 textureColour = vec4(diffuse, 1.0) * texture(t_Texture, v_TexCoord);
  // if(textureColour.a < 0.5){
  //   discard;
  // }
  
  // vec4 diffuseAndSpecular = textureColour; // vec4(totalDiffuse, 1.0) * textureColour + vec4(totalSpecular, 1.0);
  out_Color = mix(vec4(sky_color,1.0),textureColour,v_vis);
  // out_Color = textureColour;
  
}
