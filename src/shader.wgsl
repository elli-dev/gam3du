struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
    @location(2) time: vec2<u32>,
    @location(3) line_pattern: u32,
};

@group(0)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@group(0)
@binding(2)
var<uniform> time: vec2<u32>;

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
    //@location(3) line_pattern: u32,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coord;
    result.position = transform * position;
    result.time = time;
    result.line_pattern = 0xaau; //line_pattern;
    return result;
}

@group(0)
@binding(1)
var r_color: texture_2d<u32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    // let is_set3: bool = (vertex.line_pattern & (1u << 3)) != 0;

    // let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    // let v = f32(tex.x) / 255.0;
    // return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
    
    //vertex.tex_coord.x + vertex.tex_coord.y < 1 ?
    //    vec4<f32>(0.0, 0.0, 0.0, 1.0) :
    //    vec4<f32>(abs(sin((vertex.tex_coord.x * 3.1416 * 10))), abs(sin((vertex.tex_coord.y * 3.1416 * 10))), 0.5, 1.0);

    let subseconds = f32(vertex.time.y) / 4294967296.0;
    let time = f32(vertex.time.x) + subseconds;

    if (vertex.tex_coord.x + vertex.tex_coord.y < sin((vertex.tex_coord.x + time * 0.1) * 3.1416 * 10))
    {
        return vec4<f32>(sin(cos(vertex.tex_coord.y * 3.1416 * 10)), 0.0, 0.0, 1.0);
    }
    else
    {
        return vec4<f32>(abs(sin((vertex.tex_coord.x * 3.1416 * 10))), abs(sin(((vertex.tex_coord.y + time * 0.25) * 3.1416 * 10))), 0.5, 1.0);
    }

    //let result = (vertex.tex_coord.x + vertex.tex_coord.y < 1) ?
    //    vec4<f32>(0.0, 0.0, 0.0, 1.0) :
    //    vec4<f32>(abs(sin((vertex.tex_coord.x * 3.1416 * 10))), abs(sin((vertex.tex_coord.y * 3.1416 * 10))), 0.5, 1.0);
//
    //return result;
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}


@fragment
fn fs_floor_tile(vertex: VertexOutput) -> @location(0) vec4<f32> {
    if (vertex.tex_coord.x > 0.05 && vertex.tex_coord.x < 0.95 &&vertex.tex_coord.y > 0.05 && vertex.tex_coord.y < 0.95) {
        return vec4<f32>(vertex.tex_coord.x, vertex.tex_coord.y, 0.5, 1.0);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

}
