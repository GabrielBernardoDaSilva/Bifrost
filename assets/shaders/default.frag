#version 460 core


in vec2 v_tex_coord;
in mat4 debug;

out vec4 frag_color;


uniform vec4 u_color;
uniform sampler2D u_texture;

void main(){
    frag_color = texture(u_texture, v_tex_coord) * u_color;
}