#version 430 core

layout(local_size_x = 64) in;  // 64 threads per workgroup

// Define the Particle structure
struct Particle {
    vec2 pos;
    vec2 vel;
    vec2 acc;
};

// Define the SSBO buffer that will hold an array of Particles
layout(std430, binding = 0) buffer ParticleBuffer {
Particle particles[];  // Array of particles
};

uniform vec2 uMousePos;
uniform float uQuadSize;
uniform float uTime;
uniform float uDt;

const float G = 6.67430e-11;
const float softening = 0.001;

void main() {
uint idx = gl_GlobalInvocationID.x;  // Get the global index of the current workgroup

    // If the index is out of range, return early
if(idx >= gl_NumWorkGroups.x * gl_WorkGroupSize.x) {
return;
}

    // Read the particle data
Particle p = particles[idx];

    // Calculate vector from particle to cursor
vec2 dir = uMousePos - p.pos;
float dist = length(dir) + softening;  // Add softening to prevent division by zero

    // Calculate acceleration using Newton's law of gravitation (scaled for 2D)
float forceMagnitude = (G) / (dist * dist);  // Gravitational force magnitude
vec2 forceDirection = normalize(dir);  // Direction towards the cursor
p.acc = forceMagnitude * forceDirection;  // Set the acceleration (force direction * force/mass)

    // Update velocity and position
p.vel += p.acc * uDt;  // Update velocity based on acceleration and time
p.pos += p.vel * uDt;  // Update position based on velocity and time

    // Write the updated particle data back to the buffer
particles[idx] = p;
}
