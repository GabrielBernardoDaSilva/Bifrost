#version 460 core

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_texCoord;


out vec2 v_tex_coord;
out mat4 debug;

uniform mat4 u_model;
uniform mat4 u_projection;

void main() {
    v_tex_coord = a_texCoord;
    debug = u_projection;
    gl_Position = u_projection * u_model * vec4(a_pos, 1.0);
}