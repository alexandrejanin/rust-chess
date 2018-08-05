#version 330 core

const int BATCH_SIZE = 150;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UV;

uniform mat4 transforms[BATCH_SIZE];
uniform vec2 SourcePositions[BATCH_SIZE];
uniform vec2 SourceSizes[BATCH_SIZE];


out vec2 UVCoord;
out vec2 SourcePosition;
out vec2 SourceSize;

void main() {
    gl_Position = transforms[gl_InstanceID] * vec4(Position, 1.0);

    UVCoord = UV;
    SourcePosition = SourcePositions[gl_InstanceID];
    SourceSize = SourceSizes[gl_InstanceID];
}
