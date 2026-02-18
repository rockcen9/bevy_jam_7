#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

const TWO_PI: f32 = 6.28318530718;

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom material uniforms
@group(2) @binding(0) var<uniform> settings: FirePortalSettings;

struct FirePortalSettings {
    intensity: f32,
    alpha: f32,
    _pad: vec2<f32>,
}

// Hash-based value noise (replaces iChannel0 texture lookup)
fn hash(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(127.1, 311.7));
    q += dot(q, q.yx + 19.19);
    return fract((q.x + q.y) * q.x);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y,
    );
}

// Fractal brownian motion - matches shadertoy fbm sampling at p*0.05 scale
fn fbm(p: vec2<f32>) -> f32 {
    let s = 20.0; // 1/0.05 - maps to original texture sample scale
    return noise(p * s)
        + 0.5  * noise(p * s * 2.0)
        + 0.25 * noise(p * s * 4.0)
        + 0.1  * noise(p * s * 8.0);
}

// Returns the portal rim brightness curve (peaks at r == radius)
fn portal_rim(r: f32) -> f32 {
    let radius = 0.4;
    let width  = 150.0;
    return 1.0 - pow(r - radius, 2.0) * width;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time;

    // Center UV: [0,1] -> [-0.5, 0.5]
    let uv = mesh.uv - vec2<f32>(0.5);
    let r  = length(uv);

    // Convert to polar coordinates, scroll inward over time
    var st = vec2<f32>(
        atan2(uv.y, uv.x),
        r + t * 0.1,
    );

    // Twist angle by radius and wrap to [0, 2PI]
    st.x += st.y * 1.1;
    st.x  = fract(st.x / TWO_PI) * TWO_PI;

    // FBM noise in polar space
    var n = fbm(st) * 1.5 - 1.0;
    n = max(n, 0.1);

    // Divide by the rim falloff (creates bright glow at portal edge)
    let rim_inv = max(1.0 - portal_rim(r), 0.0);
    let color   = n / max(rim_inv, 0.001);

    // Circular mask: full inside r=0.4, fades to zero at r=0.48
    let mask = smoothstep(0.48, 0.4, r);

    // palette brown_reddish #9c5864 (156,88,100) scaled dark
    let fire = vec3<f32>(0.09, 0.05, 0.06) * color * mask * settings.intensity;

    // Black inner fill that fades into the fire rim
    let black_factor = smoothstep(0.36, 0.28, r) * mask;
    let final_color  = fire * (1.0 - black_factor);

    let brightness  = max(max(fire.r, fire.g), fire.b);
    let fire_alpha  = smoothstep(0.0, 0.1, brightness) * settings.alpha;
    let black_alpha = black_factor * settings.alpha;
    let final_alpha = max(fire_alpha, black_alpha);

    return vec4<f32>(final_color, final_alpha);
}
