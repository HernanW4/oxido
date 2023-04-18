#version 330

in vec3 position;
in vec2 texture;
in vec3 normal;


uniform vec3 lightPos;

out vec3 objectColor;
out vec3 v_normal;
out vec3 frag_pos;


uniform mat4 view;
uniform mat4 perspective;
uniform mat4 model;



void main(){
    gl_Position = perspective * view * model * vec4(position, 1.0);
    objectColor = vec3(1.0, position);
    frag_pos = vec3(model * vec4(position, 1.0));
    v_normal = mat3(transpose(inverse(model))) * normal;
}

