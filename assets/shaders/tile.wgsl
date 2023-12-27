struct VertexInput {
    @builtin(vertex_index) vertex_index: u32, 
    @location(0) transform_col_0: vec4<f32>, 
    @location(1) transform_col_1: vec4<f32>, 
    @location(2) transform_col_2: vec4<f32>, 
    @location(3) transform_col_3: vec4<f32>, 
    @location(4) texcoord_top: f32, 
    @location(5) texcoord_left: f32, 
    @location(6) texcoord_bottom: f32, 
    @location(7) texcoord_right: f32, 
    @location(8) color: vec4<f32>, 
    @location(9) size: vec2<f32>, 
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, 
    @location(0) world_position: vec3<f32>, 
    @location(1) color: vec4<f32>, 
    @location(2) texcoord: vec2<f32>, 
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



@vertex 
fn vs_main(in: VertexInput) -> VertexOutput {
    var position: vec3<f32>;
    var texcoord: vec2<f32>;


    switch (in.vertex_index) {
        case 0u: {
            position = vec3<f32>(-0.5 * in.size.x, -0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(in.texcoord_left, in.texcoord_bottom);
            break;
        }
        case 1u: {
            position = vec3<f32>(-0.5 * in.size.x, 0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(in.texcoord_left, in.texcoord_top);
            break;
        }
        case 2u: {
            position = vec3<f32>(0.5 * in.size.x, -0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(in.texcoord_right, in.texcoord_bottom);
            break;
        }
        case 3u: {
            position = vec3<f32>(0.5 * in.size.x, 0.5 * in.size.y, 0.0);
            texcoord = vec2<f32>(in.texcoord_right, in.texcoord_top);
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
    let world_position = transform * vec4<f32>(position, 1.0);


    var out: VertexOutput;
    out.clip_position = cam.projection * cam.camera * world_position;
    out.world_position = world_position.xyz;
    out.color = in.color;
    out.texcoord = texcoord;
    return out;
}



@fragment 
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = in.color;
    var light_color: vec3<f32>;
    for (var i = 0u; i < lights.num_points; i++) {
        let atte = attenuation(in.world_position.xyz,lights.point_lights[i]);
        light_color += lights.point_lights[i].color * atte;
    }

    return vec4<f32>(base_color.rgb + light_color, base_color.a);
}



fn attenuation(pos: vec3<f32>, light: PointLight) -> f32 {
    let distance = length(light.position - pos);
    let attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * distance * distance);
    return attenuation;
}
