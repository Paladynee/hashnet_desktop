#version 430 core
layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

uniform vec2 uMousePos;
uniform float uQuadSize;
uniform float uTime;
uniform float uDt;

void main() {
    vec2 center = gl_in[0].gl_Position.xy;
    float size = uQuadSize;

    vec2 top_left = center + vec2(-size, size);
    vec2 top_right = center + vec2(size, size);
    vec2 bottom_left = center + vec2(-size, -size);
    vec2 bottom_right = center + vec2(size, -size);

    gl_Position = vec4(top_left, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(bottom_left, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(top_right, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(bottom_right, 0.0, 1.0);
    EmitVertex();

    EndPrimitive();
}