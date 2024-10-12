#version 410
layout (location=0) in vec3 aPos;
layout (location=1) in vec3 aNormal;
layout (location=2) in vec3 aColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 my_color;

void main(){
    my_color = aColor;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}

