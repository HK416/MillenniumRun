struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) width: f32,
    @location(1) height: f32,
    @location(2) color: vec4<f32>,
    @location(3) transform_col_0: vec4<f32>,
    @location(4) transform_col_1: vec4<f32>,
    @location(5) transform_col_2: vec4<f32>,
    @location(6) transform_col_3: vec4<f32>,
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
@group(2) @binding(0)
var tex_sampler: sampler;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let transform = mat4x4<f32>(
        in.transform_col_0,
        in.transform_col_1,
        in.transform_col_2,
        in.transform_col_3,
    );

    var position: vec3<f32>;
    var texcoord: vec2<f32>;

    switch (in.vertex_index) {
        case 0u: {
            position = vec3<f32>(0.0, 0.0, 0.0);
            texcoord = vec2<f32>(0.0, 1.0);
            break;
        }
        case 1u: {
            position = vec3<f32>(0.0, in.height, 0.0);
            texcoord = vec2<f32>(0.0, 0.0);
            break;
        }
        case 2u: {
            position = vec3<f32>(in.width, 0.0, 0.0);
            texcoord = vec2<f32>(1.0, 1.0);
            break;
        }
        case 3u: {
            position = vec3<f32>(in.width, in.height, 0.0);
            texcoord = vec2<f32>(1.0, 0.0);
            break;
        }
        default { }
    }

    var out: VertexOutput;
    out.clip_position = data.ortho * transform * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var alpha: f32 = textureSample(texture, tex_sampler, in.texcoord).r;
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
