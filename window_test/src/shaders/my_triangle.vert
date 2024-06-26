#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

// Can freely add additional params to this struct. Why?
out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    // World matrix * position?
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}