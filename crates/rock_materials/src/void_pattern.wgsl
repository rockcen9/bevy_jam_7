#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    resolution: vec2<f32>,
    alpha: f32,
    _padding: f32,  // Align to 16 bytes
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Convert UV to screen coordinates, matching shadertoy formula
    var v = settings.resolution;
    var u = 0.2 * (mesh.uv * v + mesh.uv * v - v) / v.y;

    var z = vec4<f32>(1.0, 2.0, 3.0, 0.0);
    var o = vec4<f32>(1.0, 2.0, 3.0, 0.0);

    var a = 0.5;
    var t = globals.time;

    // Main loop - 19 iterations creating complex patterns
    for (var i: f32 = 0.0; i < 19.0; i += 1.0) {
        // Add contribution to output
        o += (1.0 + cos(z + t)) / length((1.0 + i * dot(v, v)) * sin(1.5 * u / (0.5 - dot(u, u)) - 9.0 * u.yx + t));

        // Update v
        t += 1.0;
        a += 0.03;
        v = cos(t - 7.0 * u * pow(a, i)) - 5.0 * u;

        // Create rotation matrix for u transformation
        // mat2(cos(i + .02*t - z.wxzw*11.))
        let angle_base = i + 0.02 * t;
        let rot_mat = mat2x2<f32>(
            cos(angle_base - z.w * 11.0), -sin(angle_base - z.w * 11.0),
            sin(angle_base - z.x * 11.0), cos(angle_base - z.z * 11.0)
        );

        var u_rotated = rot_mat * u;

        // Update u with complex transformation
        // Use tanh for stability (prevents black artifacts)
        u += tanh(40.0 * dot(u_rotated, u_rotated) * cos(100.0 * u.yx + t)) / 200.0
           + 0.2 * a * u
           + cos(4.0 / exp(dot(o, o) / 100.0) + t) / 300.0;
    }

    // Final color calculation
    o = 25.6 / (min(o, vec4<f32>(13.0)) + 164.0 / o) - dot(u, u) / 250.0;

    // Use smoothstep threshold so near-black areas become fully transparent
    let brightness = max(max(o.r, o.g), o.b);
    let threshold_alpha = smoothstep(0.05, 0.3, brightness);
    return vec4<f32>(o.rgb, settings.alpha * threshold_alpha);
}
