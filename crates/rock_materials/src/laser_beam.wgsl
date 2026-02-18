#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    resolution: vec2<f32>,
    alpha: f32,
    _padding: f32,
}

const PI: f32 = 3.14159265358979323846;

// --- Easing Functions ---
// Signature: (start_time, duration, target_amount, current_time)

fn out_expo(start: f32, dur: f32, amount: f32, t: f32) -> f32 {
    let p = clamp((t - start) / dur, 0.0, 1.0);
    if p >= 1.0 { return amount; }
    return amount * (1.0 - pow(2.0, -10.0 * p));
}

fn ease_linear(start: f32, dur: f32, amount: f32, t: f32) -> f32 {
    let p = clamp((t - start) / dur, 0.0, 1.0);
    return amount * p;
}

fn in_out_cubic(start: f32, dur: f32, amount: f32, t: f32) -> f32 {
    var p = clamp((t - start) / dur, 0.0, 1.0);
    if p < 0.5 {
        p = 4.0 * p * p * p;
    } else {
        p = 1.0 - pow(-2.0 * p + 2.0, 3.0) / 2.0;
    }
    return amount * p;
}

fn in_out_expo(start: f32, dur: f32, amount: f32, t: f32) -> f32 {
    var p = clamp((t - start) / dur, 0.0, 1.0);
    if p == 0.0 { return 0.0; }
    if p == 1.0 { return amount; }
    if p < 0.5 {
        p = pow(2.0, 20.0 * p - 10.0) / 2.0;
    } else {
        p = (2.0 - pow(2.0, -20.0 * p + 10.0)) / 2.0;
    }
    return amount * p;
}

fn out_back(start: f32, dur: f32, amount: f32, t: f32, overshoot: f32) -> f32 {
    let p = clamp((t - start) / dur, 0.0, 1.0);
    let c1 = overshoot;
    let c3 = c1 + 1.0;
    return amount * (1.0 + c3 * pow(p - 1.0, 3.0) + c1 * pow(p - 1.0, 2.0));
}

// --- Hash ---

fn hash1(n: f32) -> f32 {
    return fract(sin(n) * 43758.5453);
}

fn hash2(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(
        fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453),
        fract(sin(dot(p, vec2<f32>(269.5, 183.3))) * 43758.5453)
    );
}

// --- Background Tiles ---

fn tile(uv: vec2<f32>, scale: f32) -> f32 {
    let p = fract(uv * scale) - 0.5;
    let b = 0.5 - abs(p.x) - abs(p.y);
    return smoothstep(0.0, 0.02, b) * 0.1;
}

// --- Voronoi ---
// Returns vec2(F1, F2-F1)

fn voronoi(uv: vec2<f32>) -> vec2<f32> {
    let cell = floor(uv);
    let frac_uv = fract(uv);

    var min_dist = 8.0;
    var second_dist = 8.0;

    for (var y: i32 = -1; y <= 1; y++) {
        for (var x: i32 = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let point = hash2(cell + neighbor);
            let diff = neighbor + point - frac_uv;
            let dist = length(diff);
            if dist < min_dist {
                second_dist = min_dist;
                min_dist = dist;
            } else if dist < second_dist {
                second_dist = dist;
            }
        }
    }

    return vec2<f32>(min_dist, second_dist - min_dist);
}

// --- Blend ---

fn layer(base: vec3<f32>, alpha: f32, color: vec3<f32>) -> vec3<f32> {
    return mix(base, color, clamp(alpha, 0.0, 1.0));
}

// --- Fragment ---

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let ti = globals.time;
    let res = settings.resolution;

    let frag_coord = mesh.uv * res;

    var uv = (frag_coord - 0.5 * res) / res.y;

    let t = ti % 6.0;
    var power = 0.0;

    // Dramatic zoom in, then zoom out
    uv = uv * (1.0 - out_expo(0.0, 0.5, 0.05, t));
    uv = uv * (1.0 - ease_linear(0.0, 4.0, 0.15, t));
    uv = uv * (1.0 + in_out_cubic(4.0, 1.0, 0.24, t));

    // Screen shake power envelope
    power = power + out_expo(0.0, 0.5, 0.05, t);
    power = power - in_out_expo(4.0, 1.0, 0.05, t);

    // Screen shake offset
    let shake_x = hash1(floor(t * 30.0));
    let shake_y = hash1(floor(t * 30.0) + 0.2);
    uv = uv + vec2<f32>(shake_x, shake_y) * (0.1 * power) - (0.05 * power);

    var uv1 = uv;
    var uv2 = uv;

    // Whole-screen UV for vignette (aspect-ratio preserved on both axes)
    let over = (frag_coord - 0.5 * res) / res;

    var col = vec3<f32>(0.0);

    // Distort each half of the laser in opposite directions
    uv1.y = uv1.y + 0.1 * sin(uv1.x + t * PI);
    uv2.y = uv2.y + 0.1 * cos(uv2.x + t * PI);

    // Gap opening: starts closed, opens quickly, then closes
    var piece = -1.0;
    piece = piece + out_expo(0.0, 0.5, 1.05, t);
    piece = piece + ease_linear(4.0, 1.0, -1.0, t);

    // Slow down laser movement in the second half of the loop
    var re = 0.0;
    re = re + ease_linear(4.0, 1.0, 1.0, t);
    let t2 = (ti - re) % 6.0;

    // Two voronoi fields scrolling in opposite directions
    var base = voronoi(vec2<f32>(uv1.x * 0.025 - t2 * 0.5, uv1.y) * 20.0).y + piece;
    var base2 = voronoi(vec2<f32>(uv2.x * 0.025 + t2 * 0.4, uv2.y) * 20.0).y + piece;

    // Let the voronoi fields warp the containment ellipse UV
    uv.y = uv.y + base * 0.075;
    uv.y = uv.y - base2 * 0.075;

    // Ellipse that contains the laser beam; squishes open on entry.
    // aspect = res.y/res.x so the ellipse spans the full mesh length, not a fixed pixel width.
    let aspect = res.y / res.x;
    let stretch_y = 10.0 - out_back(0.0, 0.5, 8.5, t, 0.6);
    let circ = length(vec2<f32>(uv.x * aspect, uv.y * stretch_y));

    base = min(base, base2);
    base = clamp(base, 0.0, 1.0) * 2.0;

    // Red glow from the cell borders, masked by the ellipse
    col = layer(col, min(0.02 / smoothstep(0.3, 1.0, circ), smoothstep(0.0, 0.2, base)), vec3<f32>(1.0, 0.0, 0.0));
    // Dark stripes cut through the laser interior
    col = layer(col, min(smoothstep(0.0, 0.5, base), smoothstep(0.4, 0.2, circ) * 2.0), vec3<f32>(0.0, 0.0, 0.0));

    // Vignette and alpha: transparent outside the beam ellipse
    let vignette = smoothstep(1.2, 0.25, length(over));
    col = col * vignette;
    let beam_alpha = smoothstep(0.5, 0.2, circ) * vignette;

    return vec4<f32>(col, settings.alpha * beam_alpha);
}
