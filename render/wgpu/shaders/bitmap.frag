#version 450

layout(set = 0, binding = 2) uniform Colors {
    vec4 mult_color;
    vec4 add_color;
};

layout(set = 0, binding = 3) uniform texture2D t_color;
layout(set = 0, binding = 4) uniform sampler s_color;

layout(location=0) in vec2 frag_uv;

layout(location=0) out vec4 out_color;

void main() {

    vec4 color = texture(sampler2D(t_color, s_color), frag_uv);
    // Unmultiply alpha before apply color transform.
    if( color.a > 0 ) {
        color.rgb /= color.a;
        color = mult_color * color + add_color;
        color.rgb *= color.a;
    }

    out_color = color;
}