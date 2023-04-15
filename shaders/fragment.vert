#version 140

in vec3 v_tex_cords;
in vec3 lightColor;
in vec3 objectColor;

out vec4 color;

uniform sampler2D tex;
void main(){
    color = vec4(lightColor * objectColor, 1.0);
}
 
