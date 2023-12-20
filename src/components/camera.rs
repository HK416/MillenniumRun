use std::mem::size_of;

use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};
use winit::dpi::PhysicalPosition;

use crate::components::transform::{
    Transform, 
    Projection
};



/// #### 한국어 </br>
/// 뷰포트 영역 데이터를 담고있는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains viewport area data. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub __padding0: [u8; size_of::<f32>() * 2],
}

impl Default for Viewport {
    #[inline]
    fn default() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0, 
            width: 800.0, 
            height: 600.0, 
            min_z: 0.0, 
            max_z: 1.0, 
            __padding0: [0; size_of::<f32>() * 2] 
        }
    }
}


/// #### 한국어 </br>
/// 카메라 유니폼 버퍼의 데이터 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the data structure of the camera uniform buffer. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct CameraData {
    pub camera: Mat4,
    pub projection: Mat4,
    pub position: Vec3,
    pub scale_factor: f32,
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            camera: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            scale_factor: 1.0,
        }
    }
}



/// #### 한국어 </br>
/// 월드 좌표상에 존재하는 게임 카메라 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game camera object that exists in world coordinates. </br>
/// 
#[derive(Debug)]
pub struct GameCamera {
    bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    viewport_buffer: wgpu::Buffer,
    pub viewport: Viewport,
    pub transform: Transform,
    pub projection: Projection,
    pub scale_factor: f32,
}

impl GameCamera {
    pub fn new(
        name: Option<&str>,
        viewport: Viewport,
        transform: Transform, 
        projection: Projection,
        scale_factor: f32,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout
    ) -> Self {
        // (한국어) 라벨 데이터를 생성합니다.
        // (English Translation) Create a label data.
        let label = format!("GameCamera({})", name.unwrap_or("Unknown"));
        
        // (한국어) 카메라 데이터 유니폼 버퍼를 생성합니다.
        // (English Translation) Create a camera data uniform buffer.
        use wgpu::util::DeviceExt;
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Uniform(CameraData({}))", label)),
                contents: bytemuck::bytes_of(&CameraData {
                    camera: transform.camera_transform(),
                    projection: projection.projection_transform(),
                    position: transform.get_position(),
                    scale_factor: scale_factor,
                }),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 뷰포트 데이터 유니폼 버퍼를 생성합니다.
        // (English Translation) Create a viewport data uniform buffer.
        let viewport_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Uniform(Viewport({}))", label)),
                contents: bytemuck::bytes_of(&viewport),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 카메라 바인드 그룹을 생성합니다.
        // (English Translation) Create a camera data bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BindGroup({})", label)),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            camera_buffer.as_entire_buffer_binding()
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(
                            viewport_buffer.as_entire_buffer_binding()
                        ),
                    },
                ],
            }
        );

        Self { 
            bind_group,
            camera_buffer,
            viewport_buffer,
            viewport,
            transform,
            projection, 
            scale_factor,
        }
    }   

    /// #### 한국어 </br>
    /// 카메라의 유니폼 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the camera's uniform buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    #[inline]
    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.viewport_buffer, 0, bytemuck::bytes_of(&self.viewport));
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&CameraData {
            camera: self.transform.camera_transform(),
            projection: self.projection.projection_transform(),
            position: self.transform.get_position(),
            scale_factor: self.scale_factor,
        }));
    }

    /// #### 한국어 </br>
    /// 카메라 데이터 유니폼 버퍼를 렌더 패스에 바인드 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Bind the camera data uniform buffer to the render pass. </br>
    /// 
    #[inline]
    pub fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(0, &self.bind_group, &[]);
    }

    /// #### 한국어 </br>
    /// 윈도우 좌표계의 x축, y축 위치를 월드 좌표계의 x축, y축 위치로 변환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Converts the x- and y-axis positions of the window coordinate system 
    /// to the x- and y-axis positions of the world coordinate system. </br>
    /// 
    pub fn to_world_coordinates(&self, pos: &PhysicalPosition<f64>) -> (f32, f32) {
        use glam::vec4;
        let x = -1.0 + 2.0 * (pos.x as f32 - self.viewport.x) / self.viewport.width;
        let y = -1.0 + 2.0 * (pos.y as f32 - self.viewport.y) / self.viewport.height;
        let inv_projection = self.projection.projection_transform().inverse();
        let inv_camrea = self.transform.camera_transform().inverse();
        let point = inv_camrea * inv_projection * vec4(x, y, 0.0, 1.0);
        (point.x, point.y)
    }
}



/// #### 한국어 </br>
/// 카메라 데이터 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a camera data bind group layout. </br>
/// 
pub fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(GameCamera)"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None,
                },
            ],
        },
    )
}
