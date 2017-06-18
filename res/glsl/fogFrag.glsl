#version 400

in vec2 textureCoords;

out vec4 out_Color;

//layout (location = 0) 
uniform sampler2D colourTexture;
//layout (location = 1) 
uniform sampler2D depthTexture;
//uniform sampler2D omitSkyTexture;

const float density = 0.07;
const float gradient = 1.5;
const float near = 0.1;
const float far = 1000.0;
const float LOG2 = 1.442695;

float LinearizeDepth(vec2 uv) {
  float z = texture2D(depthTexture, uv).r;
  return (2.0 * near) / (far + near - z * (far - near));	
}

void main() {
//  float z = gl_FragCoord.z / gl_FragCoord.w;
//  float fogFactor = exp2( -density * density * z * z * LOG2 );
//  fogFactor = clamp(fogFactor, 0.0, 1.0);
  
//  vec4 zColor = vec4(gl_FragCoord.z, gl_FragCoord.z, gl_FragCoord.z, 1.0);
//  vec4 wColor = vec4(gl_FragCoord.w, gl_FragCoord.w, gl_FragCoord.w, 1.0);
  
//  out_Color = vec4(z,z,z,1.0); // mix(vec4(0.5,0.5,0.5,1.0), texture(colourTexture, textureCoords), fogFactor );
  
//  float depth = texture2D(depthTexture, textureCoords).r;
//  depth = (2.0 * near * far / (far + near - (2.0 * depth - 1.0) * (far - near))) + 0.2;
//  depth = clamp(depth, 0.0, 1.0);
//  float visibility = exp(-pow((depth * density), gradient));
//  visibility = clamp(visibility,0.0,1.0);
  out_Color = texture(colourTexture, textureCoords);
//  out_Color = texture(depthTexture, textureCoords);
//  out_Color = texture(omitSkyTexture, textureCoords);
//  out_Color = mix(out_Color, vec4(0.01,0.01,0.01,0.5), fogFactor);
//  out_Color = mix(vec4(0.01,0.01,0.01,0.5), out_Color, fogFactor);
//  vec2 uv = textureCoords; //gl_TexCoord[0].xy;
  //vec4 sceneTexel = texture2D(sceneSampler, uv);
//  float d;
//  if (uv.x < 0.5) // left part
//    d = LinearizeDepth(uv);
//  else // right part
//    d = texture2D(depthTexture, uv).x;
//  out_Color = vec4(d, d, d, 1.0);
  
}