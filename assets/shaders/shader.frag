#version 460 core

in vec2 TexCoord;

// uniform vec4 ourColor;
uniform sampler2D diffuse;
uniform sampler2D normal;

out vec4 FragColor;
  
void main()
{
    FragColor = 0.5 * texture(diffuse, TexCoord) + 0.5 * texture(normal, TexCoord);
}
