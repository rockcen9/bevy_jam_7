#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var<uniform> settings: DissolverSettings;

struct DissolverSettings {
    resolution: vec2<f32>,
    alpha: f32,
    speed: f32,
    scale: f32,
    pop_width: f32,
    _padding: vec2<f32>,
}

// --- Hash / noise helpers ---

fn hash21(p: vec2<f32>) -> f32 {
    var q = fract(vec3<f32>(p.xyx) * 0.1031);
    q += dot(q, q.yzx + 33.33);
    return fract((q.x + q.y) * q.z);
}

fn noise2(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash21(i + vec2<f32>(0.0, 0.0)), hash21(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash21(i + vec2<f32>(0.0, 1.0)), hash21(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

// 5-octave fBm
fn fbm(pin: vec2<f32>) -> f32 {
    var v  = 0.0;
    var a  = 0.5;
    var p  = pin;
    let rot = mat2x2<f32>(0.8776, 0.4794, -0.4794, 0.8776); // ~29 deg
    let shift = vec2<f32>(100.0, 100.0);
    for (var i = 0; i < 5; i++) {
        v += a * noise2(p);
        p  = rot * p * 2.0 + shift;
        a *= 0.5;
    }
    return v;
}

// --- Turbo colormap (Google, polynomial approx) ---
fn turbo(tin: f32) -> vec3<f32> {
    let t = clamp(tin, 0.0, 1.0);
    let r = 0.1357 + t * ( 4.5974 - t * ( 42.3277 - t * (130.5887 - t * (150.5666 - t * 58.1375))));
    let g = 0.0921 + t * ( 2.7696 + t * ( 14.3562 - t * ( 91.1469 - t * (134.2958 - t * 67.3071))));
    let b = 0.1053 + t * ( 5.5699 - t * (  3.6136 - t * (  8.7466 - t * ( 22.8735 - t * 20.0225))));
    return clamp(vec3<f32>(r, g, b), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let t   = globals.time * settings.speed;
    let uv  = mesh.uv;
    let asp = settings.resolution.x / settings.resolution.y;

    // World-space coords with aspect correction
    let p   = vec2<f32>(uv.x * asp, uv.y) * settings.scale;

    // Three overlapping fBm layers animate independently
    let n1 = fbm(p * 1.0  + vec2<f32>( t * 0.30,  t * 0.10));
    let n2 = fbm(p * 1.70 + vec2<f32>(-t * 0.20,  t * 0.25) + vec2<f32>(3.14, 1.57));
    let n3 = fbm(p * 0.55 + vec2<f32>( t * 0.10, -t * 0.15) + vec2<f32>(7.00, 2.72));

    let noise_val = n1 * 0.50 + n2 * 0.30 + n3 * 0.20;

    // Looping dissolve threshold (0 → 1 → 0 … )
    let dissolve_t = fract(t * 0.12);
    let pw = max(settings.pop_width, 0.001);

    // Region that is "alive"
    let alive = step(dissolve_t, noise_val);

    // Thin glowing edge at the dissolve frontier
    let edge = smoothstep(dissolve_t - pw, dissolve_t, noise_val)
             - smoothstep(dissolve_t,      dissolve_t + pw, noise_val);

    // Turbo color driven by noise + slow hue rotation
    let hue_t   = fract(noise_val * 2.5 - t * 0.18);
    let base_col = turbo(hue_t);

    // Pop: edge flares white
    let col = mix(base_col, vec3<f32>(1.0), clamp(edge * 3.0, 0.0, 1.0));

    let final_alpha = clamp((alive + edge * 3.0) * settings.alpha, 0.0, 1.0);
    return vec4<f32>(col, final_alpha);
}
