#version 430 core

layout(local_size_x = 64) in;

struct Particle {
    vec2 pos;
    vec2 vel;
    vec2 acc;
};

layout(std430, binding = 0) buffer ParticleBuffer {
    Particle particles[];
};

uniform vec2 uMousePos;
uniform float uQuadSize;
uniform float uTime;
uniform float uDt;

const float G = 6.67430e-11;
const float softening = 0.001;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    if(idx >= gl_NumWorkGroups.x * gl_WorkGroupSize.x) {
        return;
    }

    Particle p = particles[idx];
    vec2 dir = uMousePos - p.pos;
    float dist = length(dir) + softening;

    float forceMagnitude = (G) / (dist * dist);
    vec2 forceDirection = normalize(dir); 
    p.acc = forceMagnitude * forceDirection;

    p.vel += p.acc * uDt;
    p.pos += p.vel * uDt;

    particles[idx] = p;
}
