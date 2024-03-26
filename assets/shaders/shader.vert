#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec2 aNorm;
layout (location = 3) in vec2 aTangent;
layout (location = 4) in vec2 aBitagent;

uniform mat4 model;

layout (std140) uniform Matrices
{
    mat4 projection;
    mat4 view;
};

out vec2 TexCoord;
  
void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0); 
    TexCoord = aTexCoord;
}