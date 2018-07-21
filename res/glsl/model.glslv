#version 150

in vec3 a_Pos;
// in vec3 a_Norm;
// in vec2 a_TexCoord;
out vec3 v_Color;
// out vec2 v_TexCoord;

// layout (std140)
// uniform Locals {
// 	mat4 u_Transform;
// };

// uniform mat4 u_Transform;

void main() {
    // v_TexCoord = a_TexCoord;
    v_Color = vec3(a_Pos.x + 0.5, a_Pos.y + 0.5, a_Pos.z + 0.5); // 
    gl_Position = vec4(a_Pos, 1.0); // u_Transform * 
    //gl_ClipDistance[0] = 1.0;
}
