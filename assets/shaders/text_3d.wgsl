struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) transform_col_0: vec4<f32>,
    @location(1) transform_col_1: vec4<f32>,
    @location(2) transform_col_2: vec4<f32>,
    @location(3) transform_col_3: vec4<f32>,
    @location(4) color: vec4<f32>,
    @location(5) size: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct CameraData {
    camera: mat4x4<f32>,
    projection: mat4x4<f32>,
    position: vec3<f32>,
    scale_factor: f32,
}

struct Section3d {
    transform: mat4x4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(1) @binding(0)
var<uniform> section: Section3d;
@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var tex_sampler: sampler;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // (한국어) 정점을 생성합니다.
    // (English Translation) Create a vertex.
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

    // (한국어) 정점의 위치를 계산합니다.
    // (English Translation) Calculate the position of the vertex.
    let pos_x: f32 = in.transform_col_3[0] + 0.5 * in.size.x;
    let pos_y: f32 = in.transform_col_3[1] + 0.5 * in.size.y;
    let pos_z: f32 = in.transform_col_3[2];
    let pos_w: f32 = in.transform_col_3[3];
    
    let transform: mat4x4<f32> = mat4x4<f32>(
        in.transform_col_0,
        in.transform_col_1,
        in.transform_col_2,
        vec4<f32>(pos_x, pos_y, pos_z, pos_w),
    );

    var out: VertexOutput;
    out.clip_position = cam.projection * cam.camera * section.transform * transform * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    out.color = section.color * in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var alpha: f32 = textureSample(texture, tex_sampler, in.texcoord).r;
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
