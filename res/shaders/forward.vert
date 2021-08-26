#version 450
layout(location = 0) in vec3 apos;
layout(location = 1) in vec3 anrm;

layout(location = 0) out vec3 vpos;
layout(location = 1) out vec3 vnrm;

layout(std140, set = 0, binding = 0)
uniform ViewProj {
    mat4 view;
    mat4 proj;
};

layout(std140, set = 1, binding = 0)
uniform Transform {
    mat4 model;
};

void main()
{
    vpos = (model * vec4(apos, 1.0)).xyz;
    vnrm = normalize((model * vec4(anrm, 0.0)).xyz);
    gl_Position = proj * view * model * vec4(apos, 1.0);
}
