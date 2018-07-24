#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UV;

uniform mat4 transform;

out vec2 TexCoord;

void main() {
    gl_Position = transform * vec4(Position, 1.0);
    TexCoord = UV;
}
