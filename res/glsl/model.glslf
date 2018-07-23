#version 400

in vec2 v_TexCoord;
in vec3 v_SurfaceNorm;
in vec3 v_toLight[4];
in vec3 v_toCam;
in float v_vis;

out vec4 out_Color;

uniform sampler2D t_Texture;
uniform vec3 lightColour[4];
uniform vec3 attenuation[4];
uniform float shineDamper;
uniform float reflectivity;
uniform vec3 skyColour;

void main(void) {
  vec3 unitNormal = normalize(v_SurfaceNorm);
  vec3 unitCameraVector = normalize(v_toCam);
  
  vec3 totalDiffuse = vec3(0.0);
  vec3 totalSpecular = vec3(0.0);
  
  for(int i = 0; i < 4; i++){
    float distance = length(v_toLight[i]);
    float attFactor = attenuation[i].x + (attenuation[i].y * distance) + (attenuation[i].z * distance * distance);
    vec3 unitLightVector = normalize(v_toLight[i]);
    float nDotl = dot(unitNormal, unitLightVector);
    float brightness = max(nDotl, 0.0);
    vec3 lightDirection = -unitLightVector;
    vec3 reflectedLightDirection = reflect(lightDirection, unitNormal);
    float specularFactor = dot(reflectedLightDirection, unitCameraVector);
    specularFactor = max(specularFactor, 0.0);
    float dampedFactor = pow(specularFactor, shineDamper);
    totalDiffuse = totalDiffuse + (brightness * lightColour[i]) / attFactor;
    totalSpecular = totalSpecular + (dampedFactor * reflectivity * lightColour[i]) / attFactor;
  }
  totalDiffuse = max(totalDiffuse, 0.2);
  
  vec4 textureColour = texture(modelTexture, v_TexCoord);
  if(textureColour.a < 0.5){
    discard;
  }
  
  out_Color = vec4(totalDiffuse, 1.0) * textureColour + vec4(totalSpecular, 1.0);
  out_Color = mix(vec4(skyColour,1.0),out_Color,v_vis);
  
}


void main() {
  vec4 tex = texture(t_Texture, v_TexCoord);
  // float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
  out_Color = tex;
}
