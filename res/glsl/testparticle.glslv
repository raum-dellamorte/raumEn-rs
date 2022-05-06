#version 400

in vec2 pos;

uniform mat4 projection;
uniform mat4 modelview;

void main(void){
    
	gl_Position = projection * modelview * vec4(pos, 0.0, 1.0);

}
