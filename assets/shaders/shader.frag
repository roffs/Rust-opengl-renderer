#version 460 core

in vec3 FragPos;
in vec2 TexCoord;
in mat3 TBN;

uniform sampler2D diffuseTexture;
uniform sampler2D normalTexture;

uniform vec3 lightPos;
uniform vec3 viewPos;

out vec4 FragColor;
  
void main()
{
    vec3 base_color = vec3(texture(diffuseTexture, TexCoord));

    // Material properties
    float ambient_coef = 0.5;
    vec3 diffuse_coef = vec3(0.7, 0.7, 0.7);
    vec3 specular_coef = vec3(0.5, 0.5, 0.5);
    float shininnes_coef = 32.0;

    // Light properties
    vec3 lightColor = vec3(1.0, 1.0, 1.0);

    // Ambient light
    vec3 ambient = base_color * ambient_coef;

    // Obtain normal from normal map in range [0,1]
    vec3 normal = texture(normalTexture, TexCoord).rgb;
    // Transform normal vector to range [-1,1] 
    normal = (normal * 2.0) - 1.0;
    // Transform normal to TBN space
    normal = normalize(TBN * normal);
    
    vec3 lightDir = normalize(lightPos - FragPos); 

    // Diffuse 
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * base_color;

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);  
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec = pow(max(dot(normal, halfwayDir), 0.0), shininnes_coef);
    vec3 specular = specular_coef * spec;

    vec3 result = ambient + diffuse + specular;

    FragColor = vec4(result * lightColor, 1.0);
}
