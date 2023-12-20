use std::mem::size_of;
use std::sync::Arc;

use glam::Mat4;
use bytemuck::offset_of;

use crate::{
    assets::bundle::AssetBundle, 
    components::text2d::{
        section::Section2d,
        character::{
            Character, 
            InstanceData, 
        },
    },
    render::shader::WgslDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 2차원 텍스트를 화면에 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// A tool for drawing two-dimensional text on the screen. </br>
/// 
#[derive(Debug)]
pub struct Text2dBrush {
    pipeline: wgpu::RenderPipeline,
    pub tex_sampler: wgpu::Sampler,
    pub buffer_layout: wgpu::BindGroupLayout,
    pub texture_layout: wgpu::BindGroupLayout,
}

impl Text2dBrush {
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
        let tex_sampler = create_texture_sampler(device);
        let buffer_layout = create_buffer_layout(device);
        let texture_layout = create_texture_layout(device);
        let bind_group_layouts = &[camera_layout, &buffer_layout, &texture_layout];
        let pipeline = create_render_pipeline(
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
            tex_sampler,
            buffer_layout,
            texture_layout, 
        }.into())
    }

    /// #### 한국어 </br>
    /// 주어진 텍스트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given text on the screen. </br>
    /// 
    pub fn draw<'pass, Iter>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>, iter: Iter) 
    where Iter: Iterator<Item = &'pass Section2d> {
        rpass.set_pipeline(&self.pipeline);
        for section in iter {
            section.bind_buffer(rpass);
            for ch in section.chars() {
                if let Character::Drawable(ch, context) = ch {
                    section.bind_texture(ch, rpass);
                    context.draw(rpass);
                }
            }
        }
    }
}



/// #### 한국어 </br>
/// 폰트 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a font shader module. </br>
/// 
fn create_shader_module(device: &wgpu::Device, asset_bundle: &AssetBundle) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::UI_TEXT_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Text2d"), device })?;
    asset_bundle.release(path::UI_TEXT_SHADER_PATH);
    Ok(module)
}


/// #### 한국어 </br>
/// 폰트의 텍스처 샘플러를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a texture sampler for the font. </br>
/// 
fn create_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(
        &wgpu::SamplerDescriptor {
            label: Some("Sampler(Text2d)"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }
    )
}



/// #### 한국어 </br>
/// 텍스트의 버퍼 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a buffer bind group layout for text. </br>
/// 
fn create_buffer_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(Text2d))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
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


/// #### 한국어 </br>
/// 텍스트의 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a texture bind group layout for text. </br>
/// 
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(Text2d))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { 
                            filterable: true 
                        }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering
                    ),
                    count: None
                }
            ],
        },
    )
}


/// #### 한국어 </br>
/// 텍스트의 2차원 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a two-dimensional rendering pipeline for text. </br>
/// 
fn create_render_pipeline(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    render_format: wgpu::TextureFormat,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 텍스트의 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a rendering pipeline layout for text.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Text2d)"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 텍스트의 2차원 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a two-dimensional rendering pipeline for text.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Text2d)"),
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
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(InstanceData, color) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: offset_of!(InstanceData, size) as wgpu::BufferAddress,
                            },
                        ]
                    }
                ]
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
                    })
                ],
            }),
            multiview
        }
    )
}
