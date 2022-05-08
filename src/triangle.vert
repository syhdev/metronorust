#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) mat4 proj_matrix;

void main()
{
    gl_Position = proj_matrix * vec4(position, 1.0);
}