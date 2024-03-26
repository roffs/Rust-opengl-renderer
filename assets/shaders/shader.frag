#version 460 core

in vec3 FragPos;
in vec2 TexCoord;
in mat3 TBN;

uniform sampler2D diffuseTexture;
uniform sampler2D normalTexture;

uniform vec3 lightPos;

out vec4 FragColor;
  
void main()
{
    vec3 base_color = vec3(texture(diffuseTexture, TexCoord));
    vec3 lightColor = vec3(1.0, 1.0, 1.0);

    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // obtain normal from normal map in range [0,1]
    vec3 normal = texture(normalTexture, TexCoord).rgb;
    // transform normal vector to range [-1,1] and transform it to TBN space
    normal = normalize(TBN * (normal * 2.0 - 1.0));

    
    vec3 lightDir = normalize(lightPos - FragPos); 
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;


    FragColor = vec4((ambient + diffuse)*base_color, 1.0);
}
