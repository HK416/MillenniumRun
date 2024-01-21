//! #### 한국어 </br>
//! 광원에 영향을 받는 스프라이트 객체를 정의합니다. </br>
//! 
//! #### English (Translation) </br>
//! Defines a sprite object that is affected by a light source. </br>
//! 
use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};

use glam::{Mat4, Quat, Vec4, Vec3, Vec2};
use bytemuck::{Pod, Zeroable, offset_of};

use crate::{
    assets::bundle::AssetBundle,
    render::shader::WgslDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 스프라이트 객체를 렌더링하는데 사용되는 인스턴스 데이터 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an instance data structure used to render sprite objects. </br.
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
struct Data {
    transform: Mat4,
    color: Vec4,
    size: Vec2,
    texture_index: u32,
}

impl Default for Data {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Mat4::IDENTITY, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 }, 
            texture_index: 0, 
        }
    }
}



/// #### 한국어 </br>
/// 스프라이트 객체를 렌더링하는데 사용되는 인스턴스 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains instance data used for rendering sprite objects. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instance {
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub color: Vec4, 
    pub size: Vec2, 
    pub texture_index: u32, 
}

impl Instance {
    #[inline]
    fn to_data(&self) -> Data {
        Data {
            transform: Mat4::from_scale_rotation_translation(
                self.scale, 
                self.rotation, 
                self.translation
            ),
            color: self.color, 
            size: self.size, 
            texture_index: self.texture_index, 
        }
    }
}

impl Default for Instance {
    #[inline]
    fn default() -> Self {
        Self {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 }, 
            texture_index: 0, 
        }
    }
}




/// #### 한국어 </br>
/// 조명에 영향을 받는 스프라이트 데이터 버퍼가 포함되어 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the sprite data buffer affected by lighting. </br>
/// 
#[derive(Debug)]
pub struct Sprite {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub instances: Mutex<Vec<Instance>>,
    capacity: usize, 
}

impl Sprite {
    pub fn new<Iter>(
        device: &wgpu::Device, 
        tex_sampler: &wgpu::Sampler, 
        texture_view: &wgpu::TextureView, 
        sprite_brush: &SpriteBrush, 
        iter: Iter
    ) -> Self 
    where 
        Iter: IntoIterator<Item = Instance>, 
        Iter::IntoIter: ExactSizeIterator, 
    {
        use wgpu::util::DeviceExt;

        // (한국어) 인스턴스 데이터 버퍼를 생성합니다.
        // (English Translation) Create a instance data buffer.
        let instances: Vec<Instance> = iter.into_iter().collect();
        let data: Vec<Data> = instances.iter().map(|it| it.to_data()).collect();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex(InstanceData(Sprite))"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            },
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(Texture(Sprite))"),
                layout: &sprite_brush.texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            texture_view
                        ),
                    },
                    wgpu::BindGroupEntry{
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            tex_sampler
                        ),
                    },
                ],
            },
        );

        Self { 
            buffer, 
            bind_group, 
            capacity: instances.len(), 
            instances: instances.into(), 
        }
    }

    /// #### 한국어 </br>
    /// 인스턴스 데이터 버퍼를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the instance data buffer. </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, Vec<Instance>>) {
        let mut guard = self.instances.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);

        let n = self.capacity.min(guard.len());
        let data: Vec<Data> = guard.iter().take(n).map(|it| it.to_data()).collect();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }

    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        let guard = self.instances.lock().expect("Failed to access variable.");
        let n = self.capacity.min(guard.len()) as u32;

        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
        rpass.draw(0..4, 0..n);
    }
}



/// #### 한국어 </br>
/// 조명에 영향을 받는 스프라이트 객체를 그리는 도구 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a tool for drawing sprite objects affected by lighting. </br>
/// 
#[derive(Debug)]
pub struct SpriteBrush {
    pipeline: wgpu::RenderPipeline,
    pub texture_layout: wgpu::BindGroupLayout,
}

impl SpriteBrush {
    pub fn new(
        device: &wgpu::Device, 
        camera_layout: &wgpu::BindGroupLayout,
        render_format: wgpu::TextureFormat, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState, 
        multiview: Option<std::num::NonZeroU32>, 
        asset_bundle: &AssetBundle
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        let texture_layout = create_texture_layout(device);
        let bind_group_layouts = &[camera_layout, &texture_layout];
        let pipeline = create_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format, 
            depth_stencil, 
            multisample, 
            multiview
        );

        Ok(Self {
            pipeline, 
            texture_layout, 
        }.into())
    }

    /// #### 한국어 </br>
    /// 주어진 스프라이트 객체들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given sprite objects on the screen. </br>
    /// 
    pub fn draw<'pass, Iter>(
        &'pass self, 
        rpass: &mut wgpu::RenderPass<'pass>, 
        iter: Iter
    ) where Iter: Iterator<Item = &'pass Sprite> {
        rpass.set_pipeline(&self.pipeline);
        for sprite in iter {
            sprite.draw(rpass);
        }
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
    let module = asset_bundle.get(path::SPRITE_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("LightSprite"), device })?;
    asset_bundle.release(path::SPRITE_SHADER_PATH);
    return Ok(module);
}


/// #### 한국어 </br>
/// 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a texture bind group layout. </br>
/// 
#[inline]
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(LightSprite))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { 
                            filterable: true 
                        }, 
                        view_dimension: wgpu::TextureViewDimension::D2Array, 
                        multisampled: false, 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, 
                    visibility: wgpu::ShaderStages::FRAGMENT, 
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering
                    ), 
                    count: None, 
                },
            ],
        },
    )
}



/// #### 한국어 </br>
/// 조명에 영향을 받는 스프라이트 오브젝트의 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a rendering pipeline for sprite objects affected by lighting. </br>
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
    // (English Translation) Creates a rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("Pipelinelayout(LightSprite)"),
            bind_group_layouts,
            push_constant_ranges: &[],
        },
    );

    // (한국어) 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a rendering pipeline.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(LightSprite)"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout { 
                        array_stride: size_of::<Data>() as wgpu::BufferAddress, 
                        step_mode: wgpu::VertexStepMode::Instance, 
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(Data, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(Data, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(Data, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(Data, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(Data, color) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: offset_of!(Data, size) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 6,
                                format: wgpu::VertexFormat::Uint32,
                                offset: offset_of!(Data, texture_index) as wgpu::BufferAddress,
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
