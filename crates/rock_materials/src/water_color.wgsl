#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

// Group 0: Bevy built-in global variables (time)
@group(0) @binding(1) var<uniform> globals: Globals;

// Group 2: Our custom Material
@group(2) @binding(0) var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    resolution: vec2<f32>,
    iters: f32,   // 9.0 = normal world map, 12.0 = swamp world
    speed: f32,   // scroll speed (default 1.0; higher = faster, 0.0 = frozen)
    alpha: f32,
    _padding: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let scale = 10000.0;
    let trans = globals.time * scale * settings.speed / 8.0;

    // Convert UV [0,1] to screen-like pixel coordinates, then scale
    let frag_coord = mesh.uv * settings.resolution;
    var coord = (scale * frag_coord / settings.resolution) + vec2<f32>(trans, 0.0);

    // Fractional iters: run floor+1 iterations, mix between floor and ceil results
    let iters_floor = i32(settings.iters);
    let iters_fract = fract(settings.iters);
    var result = 0.0;
    var col = vec3<f32>(0.0);
    var col_prev = vec3<f32>(0.0);
    var col_at_floor = vec3<f32>(0.0);

    for (var i = 0; i <= iters_floor; i++) {
        col_prev = col;
        coord.y -= (4.0 - result);
        coord += coord.yy / 8.0;
        coord = coord.yx / 3.0;
        coord.x *= -1.5;

        // Bitwise integer pattern — core of the water color algorithm
        let ix = i32(coord.x * 2.0 - coord.y / 2.0);
        let iy = i32(coord.y * 2.0 + coord.x / 2.0);
        let val = ((ix & iy) % 3 + 3) % 3;
        result = (result + f32(val)) / 2.0;

        col.x = result;
        col = (col.yzx * 3.0 + col_prev) / 4.0;

        if i == iters_floor - 1 {
            col_at_floor = col;
        }
    }

    // Smooth blend between floor and ceil iteration counts
    let final_col = mix(col_at_floor, col, iters_fract);
    return vec4<f32>(final_col, settings.alpha);
}
