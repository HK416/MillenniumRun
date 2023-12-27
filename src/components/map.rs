use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};

use glam::{Mat4, Vec4, Vec3, Vec2};
use bytemuck::{Pod, Zeroable, offset_of};

use crate::{
    assets::bundle::AssetBundle,
    components::{
        lights::PointLights, 
        transform::Transform, 
    },
    render::shader::WgslDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 타일의 인스턴스 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the instance data of the tile. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Transform, 
    pub texcoord_top: f32, 
    pub texcoord_left: f32, 
    pub texcoord_bottom: f32, 
    pub texcoord_right: f32, 
    pub color: Vec4, 
    pub size: Vec2, 
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Transform::default(), 
            texcoord_top: 0.0, 
            texcoord_left: 0.0, 
            texcoord_bottom: 1.0, 
            texcoord_right: 1.0, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 타일 스프라이트를 그리는 도구 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a tool for drawing tile sprites. </br> 
/// 
#[derive(Debug)]
pub struct TileBrush {
    pipeline: wgpu::RenderPipeline, 
    instance_buffer: wgpu::Buffer, 
    pub instances: Mutex<Vec<InstanceData>>, 
}

impl TileBrush {
    pub fn new(
        device: &wgpu::Device, 
        camera_layout: &wgpu::BindGroupLayout, 
        light_layout: &wgpu::BindGroupLayout, 
        render_format: wgpu::TextureFormat, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState, 
        multiview: Option<std::num::NonZeroU32>, 
        asset_bundle: &AssetBundle, 
        capacity: usize, 
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        let bind_group_layouts = &[camera_layout, light_layout];
        let pipeline = create_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format, 
            depth_stencil, 
            multisample, 
            multiview
        );

        let instances = vec![InstanceData::default(); capacity];
        let instance_buffer = create_instance_buffer(device, &instances);

        Ok(Self { 
            pipeline, 
            instance_buffer, 
            instances: instances.into() 
        }.into())
    }

    /// #### 한국어 </br>
    /// 인스턴스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation)
    /// Updates the instance data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, Vec<InstanceData>>) {
        let mut guard = self.instances.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&*guard));
    }

    #[inline]
    pub fn num_of_instances(&self) -> u32 {
        self.instances.lock().expect("Failed to access variable.").len() as u32
    }

    #[inline]
    pub fn draw<'pass>(
        &'pass self, 
        light: &'pass PointLights, 
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(1, &light.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        rpass.draw(0..4, 0..self.num_of_instances());
    }
}


/// #### 한국어 </br>
/// 쉐이더 파일에서 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a shader module from the shader file. </br>
/// 
#[inline]
fn create_shader_module(
    device: &wgpu::Device, 
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::TILE_SPRITE_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("TileSprite"), device })?;
    asset_bundle.release(path::TILE_SPRITE_SHADER_PATH);
    return Ok(module);
}


/// #### 한국어 </br>
/// 타일 스프라이트의 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a rendering pipeline for tile sprite.
/// 
fn create_pipeline(
    device: &wgpu::Device, 
    module: &wgpu::ShaderModule, 
    bind_group_layouts: &[&wgpu::BindGroupLayout], 
    render_format: wgpu::TextureFormat, 
    depth_stencil: Option<wgpu::DepthStencilState>, 
    multisample: wgpu::MultisampleState, 
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(TileSprite)"), 
            bind_group_layouts, 
            push_constant_ranges: &[], 
        }, 
    );

    // (한국어) 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a rendering pipeline. 
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(TileSprite)"), 
            layout: Some(&pipeline_layout), 
            vertex: wgpu::VertexState {
                module, 
                entry_point: "vs_main", 
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<InstanceData>() as wgpu::BufferAddress, 
                        step_mode: wgpu::VertexStepMode::Instance, 
                        attributes: &[
                            wgpu::VertexAttribute { 
                                shader_location: 0, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 1, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 2, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 3, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 4, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_top) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 5, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_left) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 6, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_bottom) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 7, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_right) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 8, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: offset_of!(InstanceData, color) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 9, 
                                format: wgpu::VertexFormat::Float32x2, 
                                offset: offset_of!(InstanceData, size) as wgpu::BufferAddress, 
                            }, 
                        ],
                    },
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, 
                strip_index_format: Some(wgpu::IndexFormat::Uint16), 
                front_face: wgpu::FrontFace::Cw, 
                cull_mode: Some(wgpu::Face::Back), 
                polygon_mode: wgpu::PolygonMode::Fill, 
                ..Default::default()
            }, 
            depth_stencil, 
            multisample, 
            fragment: Some(wgpu::FragmentState {
                module, 
                entry_point: "fs_main", 
                targets: &[
                    Some(wgpu::ColorTargetState {
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                        format: render_format, 
                        write_mask: wgpu::ColorWrites::ALL, 
                    }),
                ],
            }), 
            multiview, 
        },
    )
}


/// #### 한국어 </br>
/// 타일 스프라이트의 인스턴스 버퍼를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a instance buffer for tile sprite. </br>
/// 
#[inline]
fn create_instance_buffer(
    device: &wgpu::Device, 
    instances: &[InstanceData]
) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("VertexBuffer(Instance(TileSprite))"), 
            contents: bytemuck::cast_slice(instances), 
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
        }, 
    )
}



/// #### 한국어 </br>
/// 타일의 데이터를 담고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the data of the tile. </br>
/// 
#[derive(Debug)]
pub struct Tile {
    pub visited: bool, 
    pub color: Vec4, 
    pub transform: Transform, 
}


/// #### 한국어 </br>
/// 타일 집합의 데이터를 담고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the data of the tile set. </br>
/// 
#[derive(Debug)]
pub struct Table {
    pub tiles: Vec<Vec<Tile>>, 
    pub num_rows: usize, 
    pub num_cols: usize, 
    pub origin: Vec3, 
    pub size: Vec2, 
}

impl Table {
    pub fn new(
        num_rows: usize, 
        num_cols: usize, 
        edge_color: Vec4, 
        fill_color: Vec4, 
        origin: Vec3, 
        size: Vec2
    ) -> Self {
        assert!(num_rows > 2 && num_cols > 2, "The given row and column must be greater than 2!");
        assert!(size.x > 0.0 && size.y > 0.0, "The given size must be greater than 0!");

        let mut tiles = Vec::with_capacity(num_rows);
        for row in 0..num_rows {
            let mut lines = Vec::with_capacity(num_cols);
            for col in 0..num_cols {
                let x = position(origin.x, size.x, col);
                let y = position(origin.y, size.y, row);
                let transform = Mat4::from_translation((x, y, origin.z).into()).into();
                let color = if row == 0 || row == num_rows - 1
                || col == 0 || col == num_cols - 1 {
                    edge_color
                } else {
                    fill_color
                };

                lines.push(Tile { 
                    visited: false, 
                    transform, 
                    color 
                });
            }
            tiles.push(lines);
        }

        Self { 
            tiles, 
            num_rows, 
            num_cols, 
            origin, 
            size 
        }
    }
}



/// #### 한국어 </br>
/// 타일의 위치를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Returns the position of the tile. </br>
/// 
#[inline]
pub fn position(pos: f32, size: f32, index: usize) -> f32 {
    pos + 0.5 * size + size * index as f32
}
