#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Group 2: Our custom Material
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var<uniform> settings: PaperCutoutSettings;

struct PaperCutoutSettings {
    outline_width: f32,
    alpha: f32,
}

// Sample alpha in texture-UV space.
// Returns 0 for any UV outside [0,1] so the border zone reads as transparent.
fn sample_tex_alpha(tex_uv: vec2<f32>) -> f32 {
    let in_bounds = all(tex_uv >= vec2<f32>(0.0)) && all(tex_uv <= vec2<f32>(1.0));
    let clamped   = clamp(tex_uv, vec2<f32>(0.0), vec2<f32>(1.0));
    return select(0.0, textureSample(base_texture, base_sampler, clamped).a, in_bounds);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let ow = settings.outline_width;

    // The mesh is (1 + 2*ow) times larger than the sprite texture.
    // Remap mesh UV [0,1] → texture UV so the texture occupies the inner region.
    //   tex_uv = mesh_uv * (1 + 2*ow) - ow
    let tex_uv = mesh.uv * (1.0 + 2.0 * ow) - ow;

    // Is this pixel inside the texture area?
    let in_bounds = all(tex_uv >= vec2<f32>(0.0)) && all(tex_uv <= vec2<f32>(1.0));
    let clamped   = clamp(tex_uv, vec2<f32>(0.0), vec2<f32>(1.0));

    let original     = textureSample(base_texture, base_sampler, clamped);
    let pixel_alpha  = select(0.0, original.a,   in_bounds);
    let pixel_color  = select(vec3<f32>(0.0), original.rgb, in_bounds);

    // Sample 8 neighbours in texture-UV space to detect the alpha boundary.
    let dirs = array<vec2<f32>, 8>(
        vec2<f32>( ow,  0.0),
        vec2<f32>(-ow,  0.0),
        vec2<f32>( 0.0,  ow),
        vec2<f32>( 0.0, -ow),
        vec2<f32>( ow,  ow),
        vec2<f32>(-ow,  ow),
        vec2<f32>( ow, -ow),
        vec2<f32>(-ow, -ow),
    );

    var max_neighbour_alpha: f32 = 0.0;
    for (var i: i32 = 0; i < 8; i++) {
        max_neighbour_alpha = max(max_neighbour_alpha, sample_tex_alpha(tex_uv + dirs[i]));
    }

    // This pixel is in the outline zone if it is transparent but a neighbour is opaque.
    let in_outline = step(0.1, max_neighbour_alpha) * (1.0 - step(0.1, pixel_alpha));

    let final_color = mix(pixel_color, vec3<f32>(1.0, 1.0, 1.0), in_outline);
    let final_alpha = mix(pixel_alpha, 1.0,                        in_outline);

    return vec4<f32>(final_color, final_alpha * settings.alpha);
}
