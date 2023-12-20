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
    @location(2) texcoord: vec2<f32>,
    @location(3) texture_index: u32,
}

struct CameraData {
    camera: mat4x4<f32>, 
    projection: mat4x4<f32>, 
    position: vec3<f32>, 
    scale_factor: f32, 
}

struct PointLight {
    color: vec3<f32>, 
    position: vec3<f32>, 
    constant: f32, 
    linear: f32, 
    quadratic: f32, 
}

struct LightUniformData {
    point_lights: array<PointLight, 64>,
    num_points: u32,
}


@group(0) @binding(0)
var<uniform> cam: CameraData;
@group(1) @binding(0)
var<uniform> lights: LightUniformData;
@group(2) @binding(0)
var texture: texture_2d_array<f32>;
@group(2) @binding(1)
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
    let base_color: vec4<f32> = in.color * textureSample(texture, tex_sampler, in.texcoord, in.texture_index);

    var light_color: vec3<f32>;
    for (var i = 0u; i < lights.num_points; i++) {
        let atte = attenuation(in.clip_position.xyz, lights.point_lights[i]);
        light_color += lights.point_lights[i].color * atte;
    }

    return vec4<f32>(base_color.rgb + light_color, base_color.a);
}


fn attenuation(pos: vec3<f32>, light: PointLight) -> f32 {
    let distance = length(light.position - pos);
    let attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * distance * distance);
    return attenuation;
}
