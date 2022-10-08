#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

uniform mat4 rotation;

out vec3 Color;

void main()
{
    Color = aColor;
    gl_Position = rotation * vec4(aPos, 1.0f);
}