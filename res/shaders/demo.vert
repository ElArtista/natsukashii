#version 450
layout(location = 0) in vec3 apos;
layout(location = 0) out vec3 vpos;

void main()
{
    vpos = apos;
    gl_Position = vec4(apos, 1.0);
}
