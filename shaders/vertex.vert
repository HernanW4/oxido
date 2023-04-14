#version 140

in vec3 position;
in vec3 tex_cords;

out vec3 v_tex_cords;
out vec3 my_color;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 scale_matrix;
uniform mat4 translation_matrix;


void main(){
    v_tex_cords = tex_cords;
    mat4 matrix = translation_matrix * scale_matrix;
    gl_Position = perspective * view * matrix * vec4(position, 1.0);
    my_color = vec3(1.0, position);
}

