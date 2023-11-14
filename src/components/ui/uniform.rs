use glam::Mat4;
use bytemuck::{Pod, Zeroable};



/// #### 한국어 </br>
/// 유저 인터페이스 렌더링에 사용되는 유니폼 버퍼 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a uniform buffer structure used for user interface rendering. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Uniform {
    pub ortho: Mat4,
}

impl Default for Uniform {
    #[inline]
    fn default() -> Self {
        Self { 
            ortho: Mat4::IDENTITY,
        }
    }
}



/// #### 한국어 </br>
/// 유저 인터페이스 렌더링에 사용되는 유니폼 버퍼 쉐이더 변수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a uniform buffer shader variable used for text rendering. </br>
/// 
#[derive(Debug)]
pub struct UniformBuffer {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new(device: &wgpu::Device, content: &Uniform) -> Self {
        use wgpu::util::DeviceExt;

        // (한국어) 유니폼 버퍼를 생성합니다.
        // (English Translation) Create a uniform buffer.
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("User Interface - Uniform Buffer"),
                contents: bytemuck::bytes_of(content),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 바인드 그룹 레이아웃을 생성합니다.
        // (English Translation) Create a bind group layout.
        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("User Interface - Uniform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    },
                ],
            }
        );

        // (한국어) 바인드 그룹을 생성합니다.
        // (English Translation) Create a bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("User Interface - Uniform Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                ],
            }
        );

        Self { bind_group_layout, bind_group, buffer }
    }

    /// #### 한국어 </br>
    /// 유니폼 버퍼를 갱신합니다. </br>
    /// <b>명령어 버퍼가 명령어 대기열에 전달된 이후 유니폼 버퍼가 갱신됩니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Update the uniform buffer. </br>
    /// The uniform buffer is updated after the command buffer is passed to the command queue. </br>
    /// 
    #[inline]
    pub fn update(&self, queue: &wgpu::Queue, offset: wgpu::BufferAddress, data: &[u8]) {
        queue.write_buffer(&self.buffer, offset, data);
    }
}
