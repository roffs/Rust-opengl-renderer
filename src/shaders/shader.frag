#version 460 core

in vec2 TexCoord;

uniform vec4 ourColor;
uniform sampler2D ourTexture;

out vec4 FragColor;
  
void main()
{
    // FragColor = 0.5*ourColor + 0.5*texture(ourTexture, TexCoord);
    FragColor = ourColor;
} 