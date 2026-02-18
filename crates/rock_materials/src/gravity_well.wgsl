#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Uniforms
@group(2) @binding(0)
var<uniform> time: f32;

@group(2) @binding(1)
var<uniform> center_position: vec2<f32>;

@group(2) @binding(2)
var base_texture: texture_2d<f32>;

@group(2) @binding(3)
var base_sampler: sampler;

const PI: f32 = 3.14159265359;
const TAU: f32 = 6.28318530718;

/**
 * Gravity Well Study
 *
 * Gravity Well Techniques:
 * - Inverse-square gravitational lensing
 * - Multi-well interference warping
 * - Image-based color sampling
 *
 * Visual Features:
 * - Orbiting gravity wells warp the image
 * - Colors sampled from warped texture
 * - Glowing wells with dark cores
 */

/// Warp UV toward a gravity point
fn gravity_warp(uv: vec2<f32>, center: vec2<f32>, mass: f32, softness: f32) -> vec2<f32> {
    let delta = uv - center;
    let dist = length(delta);
    let pull = mass / (dist * dist + softness);
    return uv - normalize(delta) * pull;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let resolution = vec2<f32>(1920.0, 1080.0); // TODO: Make this dynamic
    let aspect = resolution.x / resolution.y;

    // Centered aspect-corrected coords
    var p = (uv - 0.5) * vec2<f32>(aspect, 1.0);

    // time is already a uniform, no need to extract

    // === GRAVITY WELLS ===
    var warped = p;

    // Central pulsing well
    let center = vec2<f32>(
        sin(time * 0.7) * 0.15,
        cos(time * 0.5) * 0.15
    );
    let center_mass = 0.06 + sin(time * 2.0) * 0.02;
    warped = gravity_warp(warped, center, center_mass, 0.01);

    // Orbiting wells
    var well_positions: array<vec2<f32>, 4>;
    for (var i = 0; i < 4; i++) {
        let fi = f32(i);
        let angle = time * (0.4 + fi * 0.15) + fi * TAU / 4.0;
        let radius = 0.35 + sin(time * 0.3 + fi) * 0.1;
        well_positions[i] = vec2<f32>(cos(angle), sin(angle)) * radius;

        let mass = 0.025 * (1.0 + sin(time * 1.5 + fi * 2.0) * 0.5);
        warped = gravity_warp(warped, well_positions[i], mass, 0.015);
    }

    // Convert warped coords back to UV space
    let warped_uv = warped / vec2<f32>(aspect, 1.0) + 0.5;

    // === PROCEDURAL BACKGROUND ===
    // Use procedural gradient instead of black background
    var color = vec3<f32>(
        0.1 + warped_uv.x * 0.3,
        0.1 + warped_uv.y * 0.3,
        0.2 + 0.3 * sin(time * 0.5)
    );

    // === WELL GLOWS (sample color from texture near each well) ===
    var total_glow = 0.0;
    var glow_color = vec3<f32>(0.0);

    // Center well glow
    let center_dist = length(p - center);
    let center_glow = smoothstep(0.25, 0.0, center_dist);
    let center_sample_uv = center / vec2<f32>(aspect, 1.0) + 0.5;
    var center_color = textureSample(base_texture, base_sampler, clamp(center_sample_uv, vec2<f32>(0.0), vec2<f32>(1.0))).rgb;
    // Boost brightness for glow
    center_color = center_color * 1.5 + 0.3;
    glow_color += center_color * center_glow;
    total_glow += center_glow;

    // Orbiting well glows
    for (var i = 0; i < 4; i++) {
        let dist = length(p - well_positions[i]);
        let glow = smoothstep(0.15, 0.0, dist);

        // Sample texture color at well position
        let sample_uv = well_positions[i] / vec2<f32>(aspect, 1.0) + 0.5;
        var well_color = textureSample(base_texture, base_sampler, clamp(sample_uv, vec2<f32>(0.0), vec2<f32>(1.0))).rgb;
        well_color = well_color * 1.5 + 0.3;

        glow_color += well_color * glow;
        total_glow += glow;
    }

    // Apply glow
    color += glow_color * 0.6;

    // === DARK CORES (reduced intensity) ===
    let core = smoothstep(0.06, 0.02, length(p - center));
    color *= 1.0 - core * 0.5; // Reduced from 0.9 to 0.5

    for (var i = 0; i < 4; i++) {
        let core_dist = length(p - well_positions[i]);
        let core_alpha = smoothstep(0.04, 0.015, core_dist);
        color *= 1.0 - core_alpha * 0.5; // Reduced from 0.85 to 0.5
    }

    // === POST ===
    // Slight contrast boost (removed vignette)
    color = pow(color, vec3<f32>(0.95));

    // Calculate alpha based on distance from center for transparency
    let dist_from_center = length(uv - 0.5) * 2.0;
    let alpha = smoothstep(1.0, 0.3, dist_from_center);

    return vec4<f32>(color, alpha);
}
