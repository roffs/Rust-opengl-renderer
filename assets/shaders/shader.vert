#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNorm;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBitangent;

uniform mat4 model;
uniform mat4 normalMatrix;

layout (std140, binding = 0) uniform Matrices
{
    mat4 projection;
    mat4 view;
};

out vec3 FragPos;
out vec2 TexCoord;
out mat3 TBN;
  
void main()
{
    vec3 T = normalize(vec3(normalMatrix * vec4(aTangent, 0.0)));
    vec3 B = normalize(vec3(normalMatrix * vec4(aBitangent, 0.0)));
    vec3 N = normalize(vec3(normalMatrix * vec4(aNorm, 0.0)));

    TexCoord = aTexCoord;
    FragPos = vec3(model * vec4(aPos, 1.0));
    TBN = mat3(T, B, N);

    gl_Position = projection * view * model * vec4(aPos, 1.0); 
}