#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    strength: f32,
    frequency: f32,
    alpha: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time;
    let uv = mesh.uv;

    // Calculate the oscillating offset based on time
    // This creates a "snake" effect by having the RGB channels
    // oscillate in and out with sine waves
    var temp = settings.strength;
    var temp2 = settings.strength;

    temp *= sin(time * settings.frequency);
    temp2 *= sin(time * settings.frequency);

    // Sample each color channel with different UV offsets
    // Red channel: offset by +temp in X, -temp in Y
    let sample_r = textureSample(base_texture, base_sampler, vec2<f32>(uv.x + temp, uv.y - temp));

    // Green channel: offset by -temp2 in X, +temp2 in Y
    let sample_g = textureSample(base_texture, base_sampler, vec2<f32>(uv.x - temp2, uv.y + temp2));

    // Blue channel: offset by -temp in X, +temp in Y
    let sample_b = textureSample(base_texture, base_sampler, vec2<f32>(uv.x - temp, uv.y + temp));

    // Combine the separated RGB channels
    let r = sample_r.r;
    let g = sample_g.g;
    let b = sample_b.b;

    // Use the green sample's alpha to preserve transparency
    let alpha = sample_g.a;

    // Return the combined color with the uniform alpha
    return vec4<f32>(r, g, b, alpha * settings.alpha);
}
