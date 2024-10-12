#version 410
layout (location=0) in vec3 aPos;
layout (location=1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec2 my_color;

void main(){
    my_color = vec2(aPos.x - 0.5, aPos.y);
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}

