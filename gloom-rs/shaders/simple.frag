#version 420 core

out vec4 color;
uniform float time;

void main()
{
    float time = (time >= 1.0) ? mod(time, 100.0) : time;
    float mul = gl_FragCoord.x/gl_FragCoord.y;
    float v1 = 0.1f*sin(0.4*time*mul) + 1;
    float v2 = 0.2f*sin(0.3*time*mul) + 1;
    float v3 = 0.3f*sin(0.2*time*mul) + 1;
    float v4 = 0.4f*sin(0.1*time*mul) + 1;
    color = vec4(v1, v2, v3, v4);
}
