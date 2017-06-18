#version 400

in vec4 clipSpace;
in vec2 textureCoords;
in vec3 toCamVec;
in vec3 fromLightVector[4];
in float visibility;

out vec4 out_Color;

uniform sampler2D reflectionTexture;
uniform sampler2D refractionTexture;
uniform sampler2D dudvMap;
uniform sampler2D normalMap;
uniform sampler2D depthMap;

uniform vec3 lightColour[4];
uniform vec3 attenuation[4];
uniform vec3 skyColour;

uniform float moveFactor;

const float shineDamper = 20.0;
const float reflectivity = 0.5;
const float waveStrength = 0.04;
const float refractStrength = 0.7;

const float near = 0.1;
const float far = 1000.0;

void main(void) {

  vec2 refractTxtrCoords = (clipSpace.xy/clipSpace.w)/2.0 + 0.5;
  vec2 reflectTxtrCoords = vec2(refractTxtrCoords.x, -refractTxtrCoords.y);

  float depth = texture(depthMap, refractTxtrCoords).r;
  float floorDist = 2.0 * near * far / (far + near - (2.0 * depth - 1.0) * (far - near));

  depth = gl_FragCoord.z;
  float waterDist = 2.0 * near * far / (far + near - (2.0 * depth - 1.0) * (far - near));

  float waterDepth = floorDist - waterDist;
  float edgeAlpha = clamp(waterDepth / 5.0, 0.0, 1.0);

  vec2 distortedTexCoords = texture(dudvMap, vec2(textureCoords.x + moveFactor, textureCoords.y)).rg*0.1;
  distortedTexCoords = textureCoords + vec2(distortedTexCoords.x, distortedTexCoords.y+moveFactor);
  vec2 distort = (texture(dudvMap, distortedTexCoords).rg * 2.0 - 1.0) * waveStrength * clamp(waterDepth / 20.0, 0.0, 1.0);

  refractTxtrCoords += distort;
  refractTxtrCoords = clamp(refractTxtrCoords, 0.001, 0.999);

  reflectTxtrCoords += distort;
  reflectTxtrCoords.x = clamp(reflectTxtrCoords.x, 0.001, 0.999);
  reflectTxtrCoords.y = clamp(reflectTxtrCoords.y, -0.999, -0.001);

  vec4 reflectColour = texture(reflectionTexture, reflectTxtrCoords);
  vec4 refractColour = texture(refractionTexture, refractTxtrCoords);

  vec4 normalMapColour = texture(normalMap, distortedTexCoords);
  vec3 normal = vec3(normalMapColour.r * 2.0 - 1.0, normalMapColour.b * 1.5, normalMapColour.g * 2.0 - 1.0);
  normal = normalize(normal);

  vec3 viewVec = normalize(toCamVec);
  float refractFactor = clamp(pow(dot(viewVec, normal), refractStrength), 0.0, 1.0);
  
  vec3 specularHighlights = vec3(0.0);

  for(int i = 0; i < 4; i++){
    float distance = length(-fromLightVector[i]);
    float attFactor = attenuation[i].x + (attenuation[i].y * distance) + (attenuation[i].z * distance * distance);
    vec3 reflectedLight = reflect(normalize(fromLightVector[i]), normal);
    float specular = max(dot(reflectedLight, viewVec), 0.0);
    specular = pow(specular, shineDamper);
    specularHighlights = specularHighlights + (lightColour[i] * specular * reflectivity * edgeAlpha) / attFactor;
  }
  
  out_Color = mix(reflectColour, refractColour, refractFactor);
  //out_Color = mix(out_Color, vec4(0.2, 0.2, 0.3, 1.0), 0.14);
  //out_Color = mix(out_Color, (vec4(skyColour,1.0) + vec4(specularHighlights, 0.0)) * edgeAlpha, visibility);
  out_Color = mix(vec4(skyColour,1.0),out_Color,visibility);
  out_Color.a = edgeAlpha;
}