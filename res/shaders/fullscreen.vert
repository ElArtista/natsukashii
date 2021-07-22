#version 450
layout(location = 0) out vec2 vuv;

void main()
{
    vuv = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
    gl_Position = vec4(vuv * 2.0 + -1.0, 0.0, 1.0);
}
