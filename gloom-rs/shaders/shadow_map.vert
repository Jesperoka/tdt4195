#version 420 core

layout(location = 0) in vec3 xyz_in;
layout(location = 1) in vec3 nxnynz_in;
layout(location = 2) in vec4 rgba_in;

uniform mat4 depth_mvp;
uniform mat4 transformation;

void main(){
    gl_Position =  depth_mvp * transformation * vec4(xyz_in, 1.0f);
}
