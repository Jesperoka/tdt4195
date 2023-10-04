#version 420 core

layout(location = 0) in vec3 xyz_in;
layout(location = 1) in vec3 nxnynz_in;
layout(location = 2) in vec4 rgba_in;

out vec3 nxnynz;
out vec4 rgba;
out vec3 xyz_light_space;

uniform float time; 
uniform mat4 homography;
uniform mat4 transformation;
uniform mat4 depth_mvp;

void main()
{
    // Perspective transformation
    vec4 xyzw = homography*transformation*vec4(xyz_in, 1.0f);
    gl_Position = xyzw; 

    // Lighting and shadows
    nxnynz = normalize(mat3(transformation)*nxnynz_in);
    rgba = rgba_in;
    xyz_light_space = 0.5*(depth_mvp * transformation * vec4(xyz_in, 1.0f)).xyz + 0.5;
}
