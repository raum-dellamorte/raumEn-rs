#version 400

in vec2 v_TexCoord;

out vec4 out_Color;

uniform sampler2D depthMap;
uniform sampler2D guiTexture;

void main(void){

  float depth = texture(depthMap,v_TexCoord).r;
  vec4 tex = texture(guiTexture,v_TexCoord);
  float near = 0.1;
  float far = 1000.0;
  float d = 2.0 * near * far / (far + near - (2.0 * depth - 1.0) * (far - near));
  d = d * 0.01;
  d = clamp(d,0.0,1.0);
  float x = v_TexCoord.x;
  if ( x < 0.5 ) {
    out_Color = tex;
  } else {
    out_Color = vec4(d, d, 1.0 - d, 1.0);
  }

}