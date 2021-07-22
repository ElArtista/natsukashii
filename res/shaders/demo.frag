#version 450
layout(location = 0) in vec3 vpos;
layout(location = 0) out vec4 fcolor;

void main()
{
    vec3 nrm = normalize(cross(dFdx(vpos), dFdy(vpos)));
    vec3 dir = normalize(vec3(0.5,0,1));
    vec3 col = vec3(max(dot(nrm, dir), 0.0));
    fcolor = vec4(col, 1.0);
}
