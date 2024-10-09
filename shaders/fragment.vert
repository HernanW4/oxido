#version 410

in vec2 my_color;
out vec4 fragColor;

void main(){
    fragColor = vec4(my_color, 0.5, 1.0);
}

