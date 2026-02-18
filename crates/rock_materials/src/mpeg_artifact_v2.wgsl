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
    intensity: f32,
    alpha: f32,
    fill: f32,
}

// Simplex noise helpers
fn mod289_vec3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_vec2(x: vec2<f32>) -> vec2<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(x: vec3<f32>) -> vec3<f32> {
    return mod289_vec3(((x * 34.0) + 1.0) * x);
}

fn snoise(v: vec2<f32>) -> f32 {
    let C = vec4<f32>(
        0.211324865405187,
        0.366025403784439,
        -0.577350269189626,
        0.024390243902439
    );

    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    var i1: vec2<f32>;
    if (x0.x > x0.y) {
        i1 = vec2<f32>(1.0, 0.0);
    } else {
        i1 = vec2<f32>(0.0, 1.0);
    }

    var x12 = x0.xyxy + C.xxzz;
    x12 = vec4<f32>(x12.xy - i1, x12.zw);

    i = mod289_vec2(i);
    let p = permute(permute(i.y + vec3<f32>(0.0, i1.y, 1.0)) + i.x + vec3<f32>(0.0, i1.x, 1.0));

    var m = max(vec3<f32>(0.5) - vec3<f32>(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3<f32>(0.0));
    m = m * m;
    m = m * m;

    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    var g: vec3<f32>;
    g.x = a0.x * x0.x + h.x * x0.y;
    g.y = a0.y * x12.x + h.y * x12.y;
    g.z = a0.z * x12.z + h.z * x12.w;

    return 130.0 * dot(m, g);
}

fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let time = globals.time * 2.0;

    // MPEG artifact noise
    var noise = max(0.0, snoise(vec2<f32>(time, uv.y * 0.3)) - 0.3) * (1.0 / 0.7);
    noise = noise + (snoise(vec2<f32>(time * 10.0, uv.y * 2.4)) - 0.5) * 0.15;
    noise = noise * settings.intensity;

    let xpos = uv.x - noise * noise * 0.25;
    var color = textureSample(base_texture, base_sampler, vec2<f32>(xpos, uv.y));

    let interference = vec3<f32>(rand(vec2<f32>(uv.y * time)));
    color = vec4<f32>(mix(color.rgb, interference, noise * 0.3), color.a);

    let frag_y = mesh.position.y;
    if (floor((frag_y * 0.25) % 2.0) == 0.0) {
        color = vec4<f32>(color.rgb * (1.0 - (0.15 * noise)), color.a);
    }

    let g = mix(color.r, textureSample(base_texture, base_sampler, vec2<f32>(xpos + noise * 0.05, uv.y)).g, 0.25);
    let b = mix(color.r, textureSample(base_texture, base_sampler, vec2<f32>(xpos - noise * 0.05, uv.y)).b, 0.25);

    color = vec4<f32>(color.r, g, b, color.a);

    // Fill bar: grows bottom → top
    // uv.y = 0 is top, uv.y = 1 is bottom; threshold = 1.0 - fill
    let threshold = 1.0 - settings.fill;
    let in_fill = smoothstep(threshold - 0.005, threshold + 0.005, uv.y);

    // Mix fill_color onto the texture inside the filled region
    let tinted = mix(color.rgb, fill_color.rgb, fill_color.a);
    color = vec4<f32>(mix(color.rgb, tinted, in_fill), color.a);

    return vec4<f32>(color.rgb, color.a * settings.alpha);
}
