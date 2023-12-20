use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};

use glam::Vec3;
use bytemuck::{Pod, Zeroable};



/// #### 한국어 </br>
/// 조명의 최대 갯수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the maximum number of lights. </br>
/// 
const MAX_LIGHTS: usize = 64;

/// #### 한국어 </br>
/// 점 조명 데이터를 담고있는 구조체 입니다. </br>
/// 다음 방정식을 사용하여 점 조명에 영향을 받는지 평가합니다: </br>
/// `attenuation = 1.0 / (constant + linear * distance + quadratic * distance^2)` </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains point lighting data. </br>
/// Use the following equation to evaluate whether point lighting is affected: </br>
/// `attenuation = 1.0 / (constant + linear * distance + quadratic * distance^2)` </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct PointLight {
    pub color: Vec3,
    pub _padding0: [u8; size_of::<u8>() * 16 - size_of::<Vec3>()],
    pub position: Vec3,
    pub _padding1: [u8; size_of::<u8>() * 16 - size_of::<Vec3>()],
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub _padding2: [u8; size_of::<u8>() * 16 - size_of::<f32>() * 3],
}

impl Default for PointLight {
    #[inline]
    fn default() -> Self {
        Self { 
            color: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            _padding0: [0u8; size_of::<u8>() * 16 - size_of::<Vec3>()], 
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            _padding1: [0u8; size_of::<u8>() * 16 - size_of::<Vec3>()],
            constant: 1.0, 
            linear: 0.09, 
            quadratic: 0.032, 
            _padding2: [0u8; size_of::<u8>() * 16 - size_of::<f32>() * 3] 
        }
    }
}



/// #### 한국어 </br>
/// 조명의 유니폼 버퍼 데이터를 담고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains uniform buffer data for lighting. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct LightUniformData {
    pub point_lights: [PointLight; MAX_LIGHTS],
    pub num_points: u32,
    pub _padding0: [u8; size_of::<u8>() * 16 - size_of::<u32>()],
}

impl Default for LightUniformData {
    #[inline]
    fn default() -> Self {
        Self { 
            point_lights: [PointLight::default(); MAX_LIGHTS], 
            num_points: 0, 
            _padding0: [0u8; size_of::<u8>() * 16 - size_of::<u32>()], 
        }
    }
}



#[derive(Debug)]
pub struct PointLights {
    buffer: wgpu::Buffer,
    pub data: Mutex<LightUniformData>,
    pub bind_group: wgpu::BindGroup,
    pub buffer_layout: wgpu::BindGroupLayout,
}

impl PointLights {
    pub fn new(device: &wgpu::Device) -> Arc<Self> {
        use wgpu::util::DeviceExt;

        let buffer_layout = create_bind_group_layout(device);

        let data = LightUniformData::default();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform(PointLight)"),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(Uniform(PointLight))"),
                layout: &buffer_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            buffer.as_entire_buffer_binding()
                        ),
                    },
                ],
            },
        );

        Self { 
            buffer, 
            data: data.into(), 
            bind_group, 
            buffer_layout, 
        }.into()
    }

    /// #### 한국어 </br>
    /// 유니폼 버퍼의 데이터를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the data in the uniform buffer. </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, LightUniformData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&*guard));
    }
}

/// #### 한국어 </br>
/// 조명 유니폼 데이터의 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a bind group layout for lighting uniform data. </br>
/// 
#[inline]
fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(PointLight))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },
            ],
        },
    )
}
