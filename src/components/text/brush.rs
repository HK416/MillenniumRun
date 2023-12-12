use std::sync::Arc;
use std::mem::size_of;

use glam::Mat4;
use bytemuck::offset_of;

use crate::{
    assets::bundle::AssetBundle, 
    components::text::{
        Section,
        section::{ d2::Section2d, d3::Section3d},
        character::{Character, CharacterData},
    },
    nodes::path::{TEXT2D_SHADER_PATH, TEXT3D_SHADER_PATH},
    render::shader::WgslDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 텍스트를 화면에 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// A tool for drawing text on the screen. </br>
/// 
#[derive(Debug)]
pub struct TextBrush {
    pipeline_2d: wgpu::RenderPipeline,
    pipeline_3d: wgpu::RenderPipeline,
    tex_sampler: wgpu::Sampler,
    buffer_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl TextBrush {
    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        render_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        multiview: Option<std::num::NonZeroU32>,
        asset_bundle: &AssetBundle
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module_2d(device, asset_bundle)?;
        cleanup_assets(asset_bundle);

        let tex_sampler = create_texture_sampler(device);
        let buffer_bind_group_layout = create_buffer_bind_group_layout(device);
        let texture_bind_group_layout = create_texture_bind_group_layout(device);
        let bind_group_layouts = &[camera_bind_group_layout, &buffer_bind_group_layout, &texture_bind_group_layout];
        let render_format_cloned = render_format.clone();
        let depth_stencil_cloned = depth_stencil.clone();
        let multisample_cloned = multisample.clone();
        let multiview_cloned = multiview.clone();
        let pipeline_2d = create_render_pipeline_2d(
            device, 
            &module, 
            bind_group_layouts, 
            render_format_cloned, 
            depth_stencil_cloned, 
            multisample_cloned, 
            multiview_cloned
        );

        let module = create_shader_module_3d(device, asset_bundle)?;
        cleanup_assets(asset_bundle);

        let render_format_cloned = render_format.clone();
        let depth_stencil_cloned = depth_stencil.clone();
        let multisample_cloned = multisample.clone();
        let multiview_cloned = multiview.clone();
        let pipeline_3d = create_render_pipeline_3d(
            device, 
            &module, 
            bind_group_layouts, 
            render_format_cloned, 
            depth_stencil_cloned, 
            multisample_cloned, 
            multiview_cloned
        );

        Ok(Self {
            pipeline_2d,
            pipeline_3d,
            tex_sampler,
            buffer_bind_group_layout,
            texture_bind_group_layout, 
        }.into())
    }

    /// #### 한국어 </br>
    /// 텍스처 샘플러를 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the texture sampler. </br>
    /// 
    #[inline]
    pub fn ref_texture_sampler(&self) -> &wgpu::Sampler {
        &self.tex_sampler
    }

    /// #### 한국어 </br>
    /// 버퍼 바인드 그룹 레이아웃을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the buffer bind group layout. </br>
    /// 
    #[inline]
    pub fn ref_buffer_layout(&self) -> &wgpu::BindGroupLayout {
        &self.buffer_bind_group_layout
    }

    /// #### 한국어 </br>
    /// 텍스처 바인드 그룹 레이아웃을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the texture bind group layout. </br>
    /// 
    #[inline]
    pub fn ref_texture_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }

    /// #### 한국어 </br>
    /// 주어진 텍스트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given text on the screen. </br>
    /// 
    pub fn draw_2d<'pass, Iter>(
        &'pass self, 
        rpass: &mut wgpu::RenderPass<'pass>,
        iter: Iter
    ) where Iter: Iterator<Item = &'pass dyn Section> {
        rpass.set_pipeline(&self.pipeline_2d);
        for section in iter {
            section.bind(rpass);
            section.draw(rpass);
        }
    }

    /// #### 한국어 </br>
    /// 주어진 텍스트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given text on the screen. </br>
    /// 
    pub fn draw_3d<'pass, Iter>(
        &'pass self, 
        rpass: &mut wgpu::RenderPass<'pass>,
        iter: Iter
    ) where Iter: Iterator<Item = &'pass dyn Section> {
        rpass.set_pipeline(&self.pipeline_3d);
        for section in iter {
            section.bind(rpass);
            section.draw(rpass);
        }
    }
}



/// #### 한국어 </br>
/// 폰트 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a font shader module. </br>
/// 
fn create_shader_module_2d(
    device: &wgpu::Device,
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    asset_bundle.get(TEXT2D_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Text(2D)"), device })
}


/// #### 한국어 </br>
/// 폰트 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a font shader module. </br>
/// 
fn create_shader_module_3d(
    device: &wgpu::Device,
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    asset_bundle.get(TEXT3D_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Text(3D)"), device })
}


/// #### 한국어 </br>
/// 사용한 에셋을 해제합니다. </br>
/// 
/// #### English (Translation) </br>
/// Release used assets. </br>
/// 
#[inline]
fn cleanup_assets(asset_bundle: &AssetBundle) {
    asset_bundle.release(TEXT2D_SHADER_PATH);
    asset_bundle.release(TEXT3D_SHADER_PATH);
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
            label: Some("Sampler(Text)"),
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
fn create_buffer_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(Text))"),
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
fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(Text))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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
fn create_render_pipeline_2d(
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
            label: Some("PipelineLayout(Text(2D))"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 텍스트의 2차원 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a two-dimensional rendering pipeline for text.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Text(2D))"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<CharacterData>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(CharacterData, color) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: offset_of!(CharacterData, size) as wgpu::BufferAddress,
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


/// #### 한국어 </br>
/// 텍스트의 2차원 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a two-dimensional rendering pipeline for text. </br>
/// 
fn create_render_pipeline_3d(
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
            label: Some("PipelineLayout(Text(3D))"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 텍스트의 2차원 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a two-dimensional rendering pipeline for text.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Text(3D))"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<CharacterData>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(CharacterData, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(CharacterData, color) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: offset_of!(CharacterData, size) as wgpu::BufferAddress,
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