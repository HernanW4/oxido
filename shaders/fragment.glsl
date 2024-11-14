#version 410

in vec3 Color;
in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

void main() {
    float relative_height = FragPos.y / 16.0;
    vec3 surface_color = vec3(0.8, 0.4, 0.2);
    vec3 deep_color = vec3(0.2, 0.1, 0.05);

    float t = smoothstep(0.0, 0.3, relative_height);

    vec3 final_color = mix(deep_color, surface_color, t);
    FragColor = vec4(final_color, 1.0);
}
