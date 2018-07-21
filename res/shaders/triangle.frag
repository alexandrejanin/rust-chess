#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} IN;

uniform sampler2D Tex;

out vec4 Color;

void main() {
    //Flip texture vertically
    Color = texture(Tex, vec2(IN.TexCoord.x, -IN.TexCoord.y));
}
