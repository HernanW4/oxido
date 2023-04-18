#version 330

in vec3 objectColor;
in vec3 v_normal;
in vec3 frag_pos;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform mat4 view;

out vec4 FragColor;

uniform sampler2D tex;
const float specularStrength = 0.5;

void main(){
    
    vec3 viewPos = vec3(view[3].xyz);

    vec3 viewDir = normalize(viewPos - frag_pos);

    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength *lightColor;

    vec3 norm = normalize(v_normal);
    vec3 lightDir = normalize(lightPos - frag_pos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
 
