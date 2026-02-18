#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    glitch_amount: f32,
    alpha: f32,
}

// Helper functions
fn sat(t: f32) -> f32 {
    return clamp(t, 0.0, 1.0);
}

fn sat_vec2(t: vec2<f32>) -> vec2<f32> {
    return clamp(t, vec2<f32>(0.0), vec2<f32>(1.0));
}

// Remaps interval [a;b] to [0;1]
fn remap(t: f32, a: f32, b: f32) -> f32 {
    return sat((t - a) / (b - a));
}

// Linear interpolation: t=[0;0.5;1], y=[0;1;0]
fn linterp(t: f32) -> f32 {
    return sat(1.0 - abs(2.0 * t - 1.0));
}

// Spectrum offset for chromatic aberration
fn spectrum_offset(t: f32) -> vec3<f32> {
    var ret: vec3<f32>;
    let lo = select(0.0, 1.0, t <= 0.5);
    let hi = 1.0 - lo;
    let w = linterp(remap(t, 1.0 / 6.0, 5.0 / 6.0));
    let neg_w = 1.0 - w;
    ret = vec3<f32>(lo, 1.0, hi) * vec3<f32>(neg_w, w, neg_w);
    return pow(ret, vec3<f32>(1.0 / 2.2));
}

// Random function [0;1]
fn rand(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Random function [-1;1]
fn srand(n: vec2<f32>) -> f32 {
    return rand(n) * 2.0 - 1.0;
}

// Truncate value to discrete levels
fn mytrunc_f32(x: f32, num_levels: f32) -> f32 {
    return floor(x * num_levels) / num_levels;
}

fn mytrunc_vec2(x: vec2<f32>, num_levels: f32) -> vec2<f32> {
    return floor(x * num_levels) / num_levels;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var uv = mesh.uv;

    // Get time and normalize it
    let time = fract(globals.time * 100.0 / 32.0) / 110.0;

    // Get glitch amount from uniform
    let GLITCH = settings.glitch_amount;

    // Calculate random values for glitch effects
    let gnm = sat(GLITCH);
    let rnd0 = rand(mytrunc_vec2(vec2<f32>(time, time), 6.0));
    let r0 = sat((1.0 - gnm) * 0.7 + rnd0);
    let rnd1 = rand(vec2<f32>(mytrunc_f32(uv.x, 10.0 * r0), time)); // horizontal
    let r1 = 1.0 - max(0.0, select(0.9999999, 0.5 - 0.5 * gnm + rnd1, (0.5 - 0.5 * gnm + rnd1) < 1.0));
    let rnd2 = rand(vec2<f32>(mytrunc_f32(uv.y, 40.0 * r1), time)); // vertical
    let r2 = sat(rnd2);

    let rnd3 = rand(vec2<f32>(mytrunc_f32(uv.y, 10.0 * r0), time));
    let r3 = (1.0 - sat(rnd3 + 0.8)) - 0.1;

    let pxrnd = rand(uv + time);

    var ofs = 0.05 * r2 * GLITCH * select(-1.0, 1.0, rnd0 > 0.5);
    ofs += 0.5 * pxrnd * ofs;

    uv.y += 0.1 * r3 * GLITCH;

    // Multi-sample chromatic aberration
    const NUM_SAMPLES: i32 = 20;
    const RCP_NUM_SAMPLES_F: f32 = 1.0 / 20.0;

    var sum = vec4<f32>(0.0);
    var wsum = vec3<f32>(0.0);

    for (var i: i32 = 0; i < NUM_SAMPLES; i++) {
        let t = f32(i) * RCP_NUM_SAMPLES_F;
        var sample_uv = uv;
        sample_uv.x = sat(sample_uv.x + ofs * t);

        let samplecol = textureSample(base_texture, base_sampler, sample_uv);
        let s = spectrum_offset(t);
        sum += vec4<f32>(samplecol.rgb * s, samplecol.a);
        wsum += s;
    }

    sum = vec4<f32>(sum.rgb / wsum, sum.a * RCP_NUM_SAMPLES_F);

    // Apply alpha from uniform
    return vec4<f32>(sum.rgb, sum.a * settings.alpha);
}
