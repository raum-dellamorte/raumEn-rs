#version 430 core

layout (location = 0) in vec2 pos;

uniform mat4 projection;

void main(void){
    
	gl_Position = projection * vec4(pos, 0.0, 1.0);

}
