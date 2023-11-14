struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) anchor_top: f32,
    @location(1) anchor_left: f32,
    @location(2) anchor_bottom: f32,
    @location(3) anchor_right: f32,
    @location(4) margin_top: i32,
    @location(5) margin_left: i32,
    @location(6) margin_bottom: i32,
    @location(7) margin_right: i32,
    @location(8) color: vec4<f32>,
    @location(9) scale: vec3<f32>,
    @location(10) depth: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Uniform {
    ortho: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> data: Uniform;
@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let scale = mat4x4<f32>(
        vec4<f32>(in.scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, in.scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, in.scale.z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );

    let width: f32 = 2.0 / data.ortho[0][0];
    let height: f32 = 2.0 / data.ortho[1][1];
    let left: f32 = 0.5 * ((-1.0 * data.ortho[3][0] * width) - width);
    let bottom: f32 = -0.5 * ((1.0 * data.ortho[3][1] * height) + height);

    let px_top: f32 = bottom + height * in.anchor_top + f32(in.margin_top);
    let px_left: f32 = left + width * in.anchor_left + f32(in.margin_left);
    let px_bottom: f32 = bottom + height * in.anchor_bottom + f32(in.margin_bottom);
    let px_right: f32 = left + width * in.anchor_right + f32(in.margin_right);

    var position: vec3<f32>;
    var texcoord: vec2<f32>;

    switch (in.vertex_index) {
        case 0u: {
            position = vec3<f32>(px_left, px_bottom, in.depth);
            texcoord = vec2<f32>(0.0, 1.0);
            break;
        }
        case 1u: {
            position = vec3<f32>(px_left, px_top, in.depth);
            texcoord = vec2<f32>(0.0, 0.0);
            break;
        }
        case 2u: {
            position = vec3<f32>(px_right, px_bottom, in.depth);
            texcoord = vec2<f32>(1.0, 1.0);
            break;
        }
        case 3u: {
            position = vec3<f32>(px_right, px_top, in.depth);
            texcoord = vec2<f32>(1.0, 0.0);
            break;
        }
        default { }
    }

    var out: VertexOutput;
    out.clip_position = data.ortho * scale * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(texture, tex_sampler, in.texcoord);
    return in.color * color;
}
