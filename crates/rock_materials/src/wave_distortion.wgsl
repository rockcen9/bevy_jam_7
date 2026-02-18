#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    wave_center: vec2<f32>,
    wave_params: vec3<f32>,  // [frequency, falloff, thickness]
    alpha: f32,
    start_time: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time;
    var uv = mesh.uv;

    // Time since this wave was spawned — fires once, no repeat
    let current_time = max(0.0, time - settings.start_time) * 0.15;

    let wave_params = settings.wave_params;

    // Get aspect ratio from texture dimensions
    let texture_dims = textureDimensions(base_texture);
    let ratio = f32(texture_dims.y) / f32(texture_dims.x);

    // Adjust wave center and UV for aspect ratio
    var wave_center = settings.wave_center;
    wave_center.y *= ratio;

    var tex_coord = uv;
    tex_coord.y *= ratio;

    // Calculate distance from wave center
    let dist = distance(tex_coord, wave_center);

    // Base is fully transparent
    var out_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    // Fade the ring out as it expands — gone before it reaches the mesh edge
    let fade = 1.0 - smoothstep(0.2, 0.45, current_time);

    // Only emit color within the ring band
    if (dist <= (current_time + wave_params.z) &&
        dist >= (current_time - wave_params.z)) {

        let diff = dist - current_time;
        // intensity peaks at ring center, falls off toward edges
        let intensity = 1.0 - pow(abs(diff / wave_params.z), wave_params.y);

        out_color = vec4<f32>(1.0, 1.0, 1.0, intensity * fade * settings.alpha);
    }

    return out_color;
}
