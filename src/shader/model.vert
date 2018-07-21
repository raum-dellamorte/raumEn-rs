#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(constant_id = 0) const float scale = 1.2f;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coords;
layout(location = 0) out vec3 v_uv;
layout(location = 1) out vec2 v_tex_coords;

uniform mat4 transform;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    v_uv = transform * vec4(position, 1.0);
    v_tex_coords = tex_coords;
    gl_Position = vec4(scale * position, 1.0);
}
