#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var<uniform> material: LiquidMaterial;

struct LiquidMaterial {
    color: vec4<f32>,
    time_scale: f32,
};

fn random2d(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

fn noise(st: vec2<f32>) -> f32 {
    let i = floor(st);
    let f = fract(st);

    let a = random2d(i);
    let b = random2d(i + vec2<f32>(1.0, 0.0));
    let c = random2d(i + vec2<f32>(0.0, 1.0));
    let d = random2d(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) +
            (c - a) * u.y * (1.0 - u.x) +
            (d - b) * u.x * u.y;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let center_uv = uv - 0.5;
    let dist = length(center_uv);
    let angle = atan2(center_uv.y, center_uv.x);

    let time = globals.time * material.time_scale;
    let noise_val = noise(vec2<f32>(angle * 2.0, time));

    let radius = 0.3 + noise_val * 0.05;
    let alpha = 1.0 - smoothstep(radius, radius + 0.01, dist);

    let highlight_dist = length(center_uv - vec2<f32>(-0.1, 0.1));
    let highlight = 1.0 - smoothstep(0.05, 0.1, highlight_dist);

    var final_color = material.color;
    final_color = mix(final_color, vec4<f32>(1.0, 1.0, 1.0, 1.0), highlight * 0.5);
    final_color.a = final_color.a * alpha;

    if (final_color.a < 0.1) {
        discard;
    }

    return final_color;
}
