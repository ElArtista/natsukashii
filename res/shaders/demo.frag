#version 450
layout(location = 0) in vec2 vuv;
layout(location = 0) out vec4 fcolor;

void main()
{
    fcolor = vec4(vec3(vuv, 1.0), 1.0);
}
