#version 420 core

in vec3 position;
uniform float time;

void main()
{
    if (mod(floor(time), 2) == 0) {
        gl_Position = vec4(position, 1.0f);
    } else { 
        gl_Position = vec4(-position, 1.0f);
    }
}
