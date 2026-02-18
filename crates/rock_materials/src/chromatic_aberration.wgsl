#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    amount: f32,
    alpha: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time;
    let uv = mesh.uv;

    // Calculate animated chromatic aberration amount
    // Uses multiple sine waves at different frequencies to create complex pulsing
    var amount = 0.0;

    amount = (1.0 + sin(time * 6.0)) * 0.5;
    amount *= 1.0 + sin(time * 16.0) * 0.5;
    amount *= 1.0 + sin(time * 19.0) * 0.5;
    amount *= 1.0 + sin(time * 27.0) * 0.5;
    amount = pow(amount, 3.0);

    // Scale by the uniform amount parameter
    amount *= settings.amount;

    // Sample each color channel with offset UV coordinates
    // Red channel shifts right, blue channel shifts left, green stays centered
    let sample_r = textureSample(base_texture, base_sampler, vec2<f32>(uv.x + amount, uv.y));
    let sample_g = textureSample(base_texture, base_sampler, uv);
    let sample_b = textureSample(base_texture, base_sampler, vec2<f32>(uv.x - amount, uv.y));

    let r = sample_r.r;
    let g = sample_g.g;
    let b = sample_b.b;

    // Use the center sample's alpha to preserve transparency
    let alpha = sample_g.a;

    // Combine channels and darken slightly based on effect strength
    let col = vec3<f32>(r, g, b) * (1.0 - amount * 0.5);

    // Multiply texture alpha with the uniform alpha setting
    return vec4<f32>(col, alpha * settings.alpha);
}
