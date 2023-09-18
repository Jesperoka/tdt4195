#version 420 core

in vec4 rgba;

out vec4 color;

uniform float time;

void main()
{
    color = vec4(rgba);
}
