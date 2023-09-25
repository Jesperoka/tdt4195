#version 420 core

in vec4 rgba;
in vec3 nxnynz;

out vec4 color;

void main()
{
    vec3 light_direction = normalize(vec3(0.8, -0.5, 0.6));
    color = vec4(rgba.rgb * max(0, dot(nxnynz, -light_direction)), rgba.a);
}
