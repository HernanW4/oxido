#version 330

in vec3 v_tex_cords;
in vec3 objectColor;


uniform vec3 lightColor;

out vec4 FragColor;

uniform sampler2D tex;

void main(){
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength *lightColor;

    vec3 result = ambient * objectColor;
    FragColor = vec4(result, 1.0);
}
 
