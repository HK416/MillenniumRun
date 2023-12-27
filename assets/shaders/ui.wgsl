struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) local_transform_col_0: vec4<f32>, 
    @location(1) local_transform_col_1: vec4<f32>, 
    @location(2) local_transform_col_2: vec4<f32>, 
    @location(3) local_transform_col_3: vec4<f32>, 
    @location(4) global_transform_col_0: vec4<f32>, 
    @location(5) global_transform_col_1: vec4<f32>, 
    @location(6) global_transform_col_2: vec4<f32>, 
    @location(7) global_transform_col_3: vec4<f32>, 
    @location(8) anchor: vec4<f32>,
    @location(9) margin: vec4<i32>,
    @location(10) color: vec4<f32>,
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

struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(0) @binding(1)
var<uniform> view: Viewport;
@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let anchor_top = in.anchor.x;
    let anchor_left = in.anchor.y;
    let anchor_bottom = in.anchor.z;
    let anchor_right = in.anchor.w;

    let margin_top = in.margin.x;
    let margin_left = in.margin.y;
    let margin_bottom = in.margin.z;
    let margin_right = in.margin.w;

    // (한국어) 사용자 인터페이스의 영역을 계산합니다.
    // (English Translation) Calculate the area of the user interface.
    let min_x: f32 = 2.0 * view.x / view.width - 1.0;
    let min_y: f32 = 2.0 * view.y / view.height - 1.0;
    let top: f32 = min_y + 2.0 * anchor_top + 2.0 * f32(margin_top) * cam.scale_factor / view.height;
    let left: f32 = min_x + 2.0 * anchor_left + 2.0 * f32(margin_left) * cam.scale_factor / view.width;
    let bottom: f32 = min_y + 2.0 * anchor_bottom + 2.0 * f32(margin_bottom) * cam.scale_factor / view.height;
    let right: f32 = min_x + 2.0 * anchor_right + 2.0 * f32(margin_right) * cam.scale_factor / view.width;

    // (한국어) 사용자 인터페이스의 뷰포트 좌표계상 가로와 세로의 길이를 계산합니다.
    // (English Translation) Calculates the width and height of the user interface in the viewport coordinates.
    let x: f32 = 0.5 * (left + right);
    let y: f32 = 0.5 * (top + bottom);
    let width: f32 = right - left;
    let height: f32 = top - bottom;

    // (한국어) 정점을 생성합니다.
    // (English Translation) Create a vertex.
    var position: vec3<f32>;
    var texcoord: vec2<f32>;
    switch (in.vertex_index) {
        case 0u: {
            position = vec3<f32>(-0.5 * width, -0.5 * height, 0.0);
            texcoord = vec2<f32>(0.0, 1.0);
            break;
        }
        case 1u: {
            position = vec3<f32>(-0.5 * width, 0.5 * height, 0.0);
            texcoord = vec2<f32>(0.0, 0.0);
            break;
        }
        case 2u: {
            position = vec3<f32>(0.5 * width, -0.5 * height, 0.0);
            texcoord = vec2<f32>(1.0, 1.0);
            break;
        }
        case 3u: {
            position = vec3<f32>(0.5 * width, 0.5 * height, 0.0);
            texcoord = vec2<f32>(1.0, 0.0);
            break;
        }
        default { }
    }

    let local_transform = mat4x4<f32>(
        in.local_transform_col_0, 
        in.local_transform_col_1, 
        in.local_transform_col_2, 
        in.local_transform_col_3
    );

    let global_transform = mat4x4<f32>(
        in.global_transform_col_0, 
        in.global_transform_col_1, 
        in.global_transform_col_2, 
        in.global_transform_col_3
    );

    var out: VertexOutput;
    let local_position = (local_transform * vec4<f32>(position, 1.0)) + vec4<f32>(x, y, 0.0, 0.0);
    out.clip_position = global_transform * local_position;
    out.texcoord = texcoord;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(texture, tex_sampler, in.texcoord);
    return in.color * color;
}
