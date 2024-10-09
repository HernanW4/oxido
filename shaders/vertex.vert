#version 410
out vec2 my_color;

const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);


void main(){
    my_color = verts[gl_VertexID];
    gl_Position = vec4(my_color - 0.5, 0.0, 1.0);
}

