#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;
@group(2) @binding(3) var<uniform> fill_color: vec4<f32>;

struct SettingsUniform {
    amount: f32,
    alpha: f32,
    fill: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time;
    let uv = mesh.uv;

    // Animated chromatic aberration amount
    var amount = 0.0;
    amount = (1.0 + sin(time * 6.0)) * 0.5;
    amount *= 1.0 + sin(time * 16.0) * 0.5;
    amount *= 1.0 + sin(time * 19.0) * 0.5;
    amount *= 1.0 + sin(time * 27.0) * 0.5;
    amount = pow(amount, 3.0);
    amount *= settings.amount;

    // Sample each color channel with offset UV (chromatic aberration)
    let sample_r = textureSample(base_texture, base_sampler, vec2<f32>(uv.x + amount, uv.y));
    let sample_g = textureSample(base_texture, base_sampler, uv);
    let sample_b = textureSample(base_texture, base_sampler, vec2<f32>(uv.x - amount, uv.y));

    let r = sample_r.r;
    let g = sample_g.g;
    let b = sample_b.b;
    let tex_alpha = sample_g.a;

    var col = vec3<f32>(r, g, b) * (1.0 - amount * 0.5);

    // Fill bar: filled region grows from bottom → top
    // uv.y = 0 is top, uv.y = 1 is bottom, so threshold is (1.0 - fill)
    let threshold = 1.0 - settings.fill;
    let in_fill = smoothstep(threshold - 0.005, threshold + 0.005, uv.y); // 1.0 in bottom fill region

    // Mix fill_color onto the texture inside the filled region.
    // fill_color.a controls how strongly the color overlays the texture.
    let tinted = mix(col, fill_color.rgb, fill_color.a);
    col = mix(col, tinted, in_fill);

    return vec4<f32>(col, tex_alpha * settings.alpha);
}
