#version 150 core

in vec3 a_Pos;
// in vec3 a_Norm;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

layout (std150)
uniform Locals {
	mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * vec4(a_Pos, 1.0);
    gl_ClipDistance[0] = 1.0;
}
