struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) transform_col_0: vec4<f32>,
    @location(1) transform_col_1: vec4<f32>,
    @location(2) transform_col_2: vec4<f32>,
    @location(3) transform_col_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct SpriteData {
    uv_top: f32,
    uv_left: f32,
    uv_bottom: f32,
    uv_right: f32,
    color: vec4<f32>,
    size: vec2<f32>,
}

struct CameraData {
    camera: mat4x4<f32>,
    orthographic: mat4x4<f32>,
    position: vec3<f32>,
    scale_factor: f32,
}

@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(1) @binding(0)
var<uniform> sprite: SpriteData;
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
            position = vec3<f32>(-0.5 * sprite.size.x, -0.5 * sprite.size.y, 0.0);
            texcoord = vec2<f32>(sprite.uv_left, sprite.uv_bottom);
            break;
        }
        case 1u: {
            position = vec3<f32>(-0.5 * sprite.size.x, 0.5 * sprite.size.y, 0.0);
            texcoord = vec2<f32>(sprite.uv_left, sprite.uv_top);
            break;
        }
        case 2u: {
            position = vec3<f32>(0.5 * sprite.size.x, -0.5 * sprite.size.y, 0.0);
            texcoord = vec2<f32>(sprite.uv_right, sprite.uv_bottom);
            break;
        }
        case 3u: {
            position = vec3<f32>(0.5 * sprite.size.x, 0.5 * sprite.size.y, 0.0);
            texcoord = vec2<f32>(sprite.uv_right, sprite.uv_top);
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
    out.clip_position = cam.orthographic * cam.camera * transform * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    out.color = sprite.color;
    return out;
}


@fragment
fn textured_fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(texture, tex_sampler, in.texcoord);
    return in.color * color;
}
