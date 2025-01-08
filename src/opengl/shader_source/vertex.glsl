#version 430 core
layout(location = 0) in vec2 aPos;

uniform vec2 uMousePos;
uniform float uQuadSize;
uniform float uTime;
uniform float uDt;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
}
