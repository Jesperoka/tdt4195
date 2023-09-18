#version 420 core

layout(location = 0) in vec3 xyz;
layout(location = 1) in vec4 rgbaIn;

out vec4 rgba;

uniform float time;

void main()
{
    gl_Position = vec4(xyz, 1.0f);
    rgba = rgbaIn;
}
