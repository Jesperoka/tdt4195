#version 420 core

layout(location = 0) in vec3 xyz;
layout(location = 1) in vec4 rgbaIn;

out vec4 rgba;

uniform float time; 

void main()
{
    float a = 1.0;
    float b = 0.0;
    float c = 0.0;
    float d = 0.0;
    float e = 1.0;
    float f = 0.0;

    if (0.0 <= time && time < 5.0) {
        a = a + sin(time);
    } else if (5.0 <= time && time <= 10.0) {
        b = b + sin(time);
    } else if (10.0 <= time && time <= 15.0) {
        c = c + sin(time);
    } else if (15.0 <= time && time <= 20.0) {
        d = d + sin(time);
    } else if (20.0 <= time && time <= 25.0) {
        e = e + sin(time);
    } else if (25.0 <= time && time <= 30.0) {
        f = f + sin(time);
    }
    
    mat4x4 homography = transpose(mat4(
        vec4(a, b , 0.0, c),
        vec4(d, e , 0.0, f),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    ));

    vec4 xyzw = homography*vec4(xyz, 1.0f);
    gl_Position = xyzw; 
    rgba = rgbaIn;
}
