#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: PoisonSettings;

struct PoisonSettings {
    poison_amount: f32,      // 0.0 - 1.0, controls effect intensity
    pulse_speed: f32,        // Speed of pulsing effect (default: 3.0)
    poison_color: vec4<f32>, // Poison tint color (default: green)
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    // Sample the original texture
    let texture_color = textureSample(base_texture, base_sampler, uv);

    // Create pulsing effect: sin oscillation from 0.0 to 1.0
    let pulse = sin(globals.time * settings.pulse_speed) * 0.5 + 0.5;

    // Calculate poison intensity with pulsing
    let poison_intensity = settings.poison_amount * pulse * 0.5;

    // Mix original color with poison color
    let poisoned_color = mix(texture_color.rgb, settings.poison_color.rgb, poison_intensity);

    // Return final color with original alpha
    return vec4<f32>(poisoned_color, texture_color.a);
}
