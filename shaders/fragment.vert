#version 410

in vec3 my_color;
out vec4 fragColor;

void main(){
    fragColor = vec4(my_color,1.0);
}

