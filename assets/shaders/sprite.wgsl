struct VertexInput {
    @builtin(vertex_index) vertex_index: u32, 
    @location(0) transform_col_0: vec4<f32>, 
    @location(1) transform_col_1: vec4<f32>, 
    @location(2) transform_col_2: vec4<f32>, 
    @location(3) transform_col_3: vec4<f32>, 
    @location(4) color: vec4<f32>, 
    @location(5) size: vec2<f32>, 
    @location(6) texture_index: u32, 
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) texcoord: vec2<f32>,
    @location(2) texture_index: u32,
}

struct CameraData {
    camera: mat4x4<f32>, 
    projection: mat4x4<f32>, 
    position: vec3<f32>, 
    scale_factor: f32, 
}



@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(1) @binding(0)
var texture: texture_2d_array<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;


@vertex 
fn vs_main(in: VertexInput) -> VertexOutput {
    var position: vec3<f32>;
    var texcoord: vec2<f32>;
    switch (in.vertex_index) {
        case 0u: {
            position = vec3<f32>(-0.5 * in.size.x, -0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(0.0, 1.0);
            break;
        }
        case 1u: {
            position = vec3<f32>(-0.5 * in.size.x, 0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(0.0, 0.0);
            break;
        }
        case 2u: {
            position = vec3<f32>(0.5 * in.size.x, -0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(1.0, 1.0);
            break;
        }
        case 3u: {
            position = vec3<f32>(0.5 * in.size.x, 0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(1.0, 0.0);
            break;
        }
        default { }
    }

    let transform = mat4x4<f32>(
        in.transform_col_0,
        in.transform_col_1,
        in.transform_col_2,
        in.transform_col_3,
    );

    var out: VertexOutput;
    out.clip_position = cam.projection * cam.camera * transform * vec4<f32>(position, 1.0);
    out.color = in.color;
    out.texcoord = texcoord;
    out.texture_index = in.texture_index;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color: vec4<f32> = textureSample(texture, tex_sampler, in.texcoord, in.texture_index);
    return in.color * texture_color;
}
