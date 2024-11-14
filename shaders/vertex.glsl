#version 410
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec3 aColor;

uniform mat4 transformation;
uniform mat4 view;
uniform mat4 projection;

out vec3 Color;
out vec3 Normal;
out vec3 FragPos;

void main() {
    FragPos = vec3(transformation * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(transformation))) * aNormal;
    Color = aColor;

    gl_Position = projection * view * vec4(FragPos, 1.0);
}
