#version 140

in vec3 position;
in vec3 tex_cords;

out vec3 v_tex_cords;
out vec3 objectColor;
out vec3 lightColor;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 model;



void main(){
    v_tex_cords = tex_cords;
    gl_Position = perspective * view * model * vec4(position, 1.0);
    lightColor = vec3(1.0, 1.0, 1.0);
    objectColor = vec3(1.0, position);
}

