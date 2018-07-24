#version 330 core

in vec2 TexCoord;

uniform vec2 SourcePosition;
uniform vec2 SourceSize;
uniform sampler2D Tex;

out vec4 Color;

void main() {
    //Flip texture vertically
    Color = texture(Tex, SourcePosition + SourceSize * TexCoord);
}
