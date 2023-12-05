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

struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

struct Section2d {
    transform: mat4x4<f32>,
    anchor_top: f32,
    anchor_left: f32,
    anchor_bottom: f32,
    anchor_right: f32,
    margin_top: i32,
    margin_left: i32,
    margin_bottom: i32,
    margin_right: i32,
    color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(0) @binding(1)
var<uniform> view: Viewport;
@group(1) @binding(0)
var<uniform> section: Section2d;
@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var tex_sampler: sampler;

// <한국어> 
// 문자를 출력하는 정점 쉐이더 입니다.
// 주어진 사용자 인터페이스의 높이에 따라 문자의 크기가 달라집니다.
// 
// <English (Translation)>
// This is a vertex shader that outputs text.
// The size of the text changes depending on the height of the given user interface.
//
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // (한국어) 사용자 인터페이스의 영역을 계산합니다.
    // (English Translation) Calculate the area of the user interface.
    let min_x: f32 = 2.0 * view.x / view.width - 1.0;
    let min_y: f32 = 2.0 * view.y / view.height - 1.0;
    let top: f32 = min_y + 2.0 * section.anchor_top + 2.0 * f32(section.margin_top) * cam.scale_factor / view.height;
    let left: f32 = min_x + 2.0 * section.anchor_left + 2.0 * f32(section.margin_left) * cam.scale_factor / view.width;
    let bottom: f32 = min_y + 2.0 * section.anchor_bottom + 2.0 * f32(section.margin_bottom) * cam.scale_factor / view.height;
    let right: f32 = min_x + 2.0 * section.anchor_right + 2.0 * f32(section.margin_right) * cam.scale_factor / view.width;
    
    let x: f32 = 0.5 * (left + right);
    let y: f32 = 0.5 * (top + bottom);
    let px_ui_h: f32 = (top - bottom) * view.height;

    // (한국어) 문자의 픽셀 단위 가로와 세로 길이를 계산합니다.
    // (English Translation) Calculate the width and height of a character in pixels.
    let px_height: f32 = in.size.y * px_ui_h;
    let px_width: f32 = in.size.x * px_height;

    // (한국어) 문자의 뷰포트 좌표계상의 가로와 세로 길이를 계산합니다.
    // (English Translation) Calculates the width and height of the character in the viewport coordinates.
    let width:f32 = px_width / view.width;
    let height: f32 = px_height / view.height;

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

    // (한국어) 정점의 위치를 계산합니다.
    // (English Translation) Calculate the position of the vertex.
    let pos_x: f32 = x + px_height * in.transform_col_3[0] / view.width + 0.5 * width;
    let pos_y: f32 = y + px_ui_h * in.transform_col_3[1] / view.height + 0.5 * height;
    let pos_z: f32 = in.transform_col_3[2];
    let pos_w: f32 = in.transform_col_3[3];
    
    let transform: mat4x4<f32> = mat4x4<f32>(
        in.transform_col_0,
        in.transform_col_1,
        in.transform_col_2,
        vec4<f32>(pos_x, pos_y, pos_z, pos_w),
    );

    var out: VertexOutput;
    out.clip_position = section.transform * transform * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    out.color = section.color * in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var alpha: f32 = textureSample(texture, tex_sampler, in.texcoord).r;
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
