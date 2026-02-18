
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var<uniform> settings: SoulSettings;

struct SoulSettings {

    color: vec4<f32>,

    speed: f32,

    intensity: f32,

    corner_radius: f32,

    seed: f32,

}

// --- Noise primitives ---

fn hash(p: vec2<f32>) -> f32 {

var q = fract(p * vec2<f32>(127.1, 311.7));

    q += dot(q, q.yx + 19.19);

return fract((q.x + q.y) * q.x);

}

// Smooth value noise

fn vnoise(p: vec2<f32>) -> f32 {

let i = floor(p);

let f = fract(p);

let u = f * f * (3.0 - 2.0 * f);

return mix(

mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),

mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),

        u.y,

    );

}

// Fractal brownian motion — 4 octaves

fn fbm(p: vec2<f32>) -> f32 {

var v   = 0.0;

var amp = 0.5;

var pos = p;

for (var i = 0; i < 4; i++) {

        v   += amp * vnoise(pos);

        pos  = pos * 2.03 + vec2<f32>(1.7, 9.2); // offset between octaves

        amp *= 0.5;

    }

return v;

}

// --- Domain-warped fluid noise (Inigo Quilez technique) ---

// Two layers of warping pull the UV like a flowing liquid.

fn fluid_noise(uv: vec2<f32>, t: f32) -> f32 {

    // First warp: drift UV with FBM

let q = vec2<f32>(

fbm(uv                       + t * 0.6),

fbm(uv + vec2<f32>(5.2, 1.3) + t * 0.5),

    );

    // Second warp: fold again using first result

let r = vec2<f32>(

fbm(uv + 3.5 * q + vec2<f32>(1.7, 9.2) + 0.7 * t),

fbm(uv + 3.5 * q + vec2<f32>(8.3, 2.8) + 0.6 * t),

    );

return fbm(uv + 3.5 * r + t * 0.3);

}

// --- Rounded rectangle SDF. uv is centered [-0.5, 0.5] ---

fn rounded_rect_sdf(uv: vec2<f32>, radius: f32) -> f32 {

let half = vec2<f32>(0.5) - vec2<f32>(radius);

let q    = abs(uv) - half;

return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;

}

@fragment

fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

let t   = globals.time * settings.speed;

let uv  = mesh.uv - vec2<f32>(0.5); // centered [-0.5, 0.5]

    // --- Fluid noise ---

    // Seed shifts each tile into a unique region of the noise field so

    // adjacent tiles never show the same animation phase.

let seed_offset = vec2<f32>(settings.seed * 17.3, settings.seed * 31.7);

let noise_val = fluid_noise(uv * 3.0 + seed_offset, t);

    // --- Alpha: driven purely by fluid noise, no edge bias ---

let alpha = clamp(noise_val * 1.2, 0.0, 0.85);

    // --- Color ---

let base      = settings.color.rgb;

let highlight = mix(base, vec3<f32>(1.0), noise_val * 0.15);

let final_col = highlight * settings.intensity;

    // --- Rounded corner mask ---

let sdf         = rounded_rect_sdf(uv, settings.corner_radius);

let corner_mask = smoothstep(0.01, -0.01, sdf);

return vec4<f32>(final_col, alpha * corner_mask);

}
