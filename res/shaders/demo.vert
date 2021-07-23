#version 450
layout(location = 0) in vec3 apos;
layout(location = 0) out vec3 vpos;

layout(std140, binding = 0)
uniform ViewProj {
    mat4 view;
    mat4 proj;
};

void main()
{
    vpos = apos;
    gl_Position = proj * view * vec4(apos, 1.0);
}
