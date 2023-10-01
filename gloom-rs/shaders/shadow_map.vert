#version 420 core

layout(location = 0) in vec3 xyz_in;
uniform mat4 depth_mvp;

void main(){
    gl_Position =  depth_mvp * vec4(xyz_in, 1);
}
