#version 420 core

layout(location = 0) in vec3 xyz_in;
layout(location = 1) in vec3 nxnynz_in;
layout(location = 2) in vec4 rgba_in;

out vec3 nxnynz;
out vec4 rgba;

uniform float time; 
uniform mat4 homography;

void main()
{
    vec4 xyzw = homography*vec4(xyz_in, 1.0f);
    gl_Position = xyzw; 
    nxnynz = nxnynz_in;
    rgba = rgba_in;
}
