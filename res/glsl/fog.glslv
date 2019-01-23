#version 400

in vec2 a_Pos;

out vec2 v_TexCoord;

// uniform float flip_y;

void main(void){
  
  gl_Position = vec4(a_Pos, 0.0, 1.0);
  
  v_TexCoord = vec2((a_Pos.x+1.0)/2.0, (a_Pos.y+1.0)/2.0);
  // if (flip_y > 0.5) {
  //   v_TexCoord = vec2(v_TexCoord.x, 1.0 - v_TexCoord.y);
  // }
}