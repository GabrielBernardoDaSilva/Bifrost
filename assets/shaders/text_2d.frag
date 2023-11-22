#version 460 core
in vec2 v_tex_coord;
out vec4 frag_color;

uniform sampler2D u_text;
uniform vec4 u_text_color;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(u_text, v_tex_coord).r);
    frag_color = sampled * u_text_color;
}