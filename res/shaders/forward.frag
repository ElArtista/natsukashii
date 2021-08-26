#version 450
layout(location = 0) in vec3 vpos;
layout(location = 1) in vec3 vnrm;

layout(location = 0) out vec4 fcolor;

layout(std140, set = 2, binding = 0)
uniform Material {
    vec3 alb;
};

void main()
{
    vec3 nrm = normalize(vnrm);
    vec3 dir = normalize(vec3(0.5,0,-1));
    vec3 col = alb * vec3(max(dot(nrm, dir), 0.0));
    fcolor = vec4(col, 1.0);
}
