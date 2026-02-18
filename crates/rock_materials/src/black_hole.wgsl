#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    distortion_strength: f32,
    rotation_speed: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time;

    // --- Core distortion logic starts ---

    // 1. Center UV coordinates to (0, 0)
    let uv_centered = mesh.uv - 0.5;

    // 2. Convert to polar coordinates (distance and angle)
    let dist = length(uv_centered);
    var angle = atan2(uv_centered.y, uv_centered.x);

    // 3. Calculate radial distortion (pulling toward center)
    // The closer to center, the more UV coordinates are stretched.
    // Using pow(dist, value < 1) creates this non-linear stretching effect.
    // Higher distortion_strength = smaller exponent = stronger pull toward center.
    let strength_factor = 1.0 - clamp(settings.distortion_strength, 0.0, 0.9);
    let distorted_dist = pow(dist, strength_factor);

    // 4. Calculate swirl distortion (rotation)
    // The closer to center, the more rotation is applied.
    // (dist + 0.2) prevents division by zero at the exact center.
    let angle_offset = time * settings.rotation_speed / (dist + 0.2);
    angle -= angle_offset; // Negative = clockwise pull, positive = counterclockwise

    // 5. Convert distorted distance and angle back to UV coordinates
    let new_uv_centered = vec2<f32>(cos(angle), sin(angle)) * distorted_dist;
    let final_uv = new_uv_centered + 0.5;

    // --- Core distortion logic ends ---

    // 6. Sample the texture using the distorted UV
    var color = textureSample(base_texture, base_sampler, final_uv);

    // 7. (Optional) Create black hole at center
    // Use smoothstep to create a smooth black hole in the center
    let hole = smoothstep(0.02, 0.15, dist);

    // 8. Create circular mask with transparent background
    // Make areas outside 0.5 radius transparent with soft edge
    let edge_softness = 0.05;
    let circular_mask = 1.0 - smoothstep(0.5 - edge_softness, 0.5, dist);

    // Apply hole darkening and circular mask
    color = vec4<f32>(color.rgb * hole, color.a * hole * circular_mask);

    return color;
}
