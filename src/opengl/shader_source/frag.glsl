#version 430 core
out vec4 FragColor;

uniform vec2 uMousePos;
uniform float uQuadSize;
uniform float uTime;
uniform float uDt;

void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}