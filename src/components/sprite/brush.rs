use std::sync::Arc;
use std::mem::size_of;

use glam::Mat4;
use bytemuck::offset_of;

use crate::{
    assets::bundle::AssetBundle, 
    components::sprite::{Sprite, objects::InstanceData},
    nodes::path::SPRITE_SHADER_PATH,
    render::shader::WgslDecoder,
    system::error::AppResult, 

};



#[derive(Debug)]
pub struct SpriteBrush {
    textured_pipeline: wgpu::RenderPipeline,
    buffer_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl SpriteBrush {
    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        render_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        multiview: Option<std::num::NonZeroU32>,
        asset_bundle: &AssetBundle
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        cleanup_assets(asset_bundle);

        let buffer_bind_group_layout = create_buffer_bind_group_layout(device);
        let texture_bind_group_layout = create_texture_bind_group_layout(device);
        let bind_group_layouts = &[camera_bind_group_layout, &buffer_bind_group_layout, &texture_bind_group_layout];
        let render_format_cloned = render_format.clone();
        let depth_stencil_cloned = depth_stencil.clone();
        let multisample_cloned = multisample.clone();
        let multiview_cloned = multiview.clone();
        let textured_pipeline = create_textured_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format_cloned, 
            depth_stencil_cloned, 
            multisample_cloned, 
            multiview_cloned
        );

        Ok(Self {
            textured_pipeline,
            buffer_bind_group_layout,
            texture_bind_group_layout,
        }.into())
    }

    /// #### 한국어 </br>
    /// 스프라이트 오브젝트 버퍼 바인드 그룹 레이아웃을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the buffer bind group layout from the sprite object. </br>
    /// 
    #[inline]
    pub fn ref_buffer_layout(&self) -> &wgpu::BindGroupLayout {
        &self.buffer_bind_group_layout
    }

    /// #### 한국어 </br>
    /// 스프라이트 오브젝트 텍스처 바인드 그룹 레이아웃을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the texture bind group layout from the sprite object. </br>
    /// 
    #[inline]
    pub fn ref_texture_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }

    /// #### 한국어 </br>
    /// 주어진 스프라이트 오브젝트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given user sprite objects on the screen. </br>
    /// 
    pub fn draw_textured<'pass, Iter>(
        &'pass self,
        rpass: &mut wgpu::RenderPass<'pass>,
        iter: Iter
    ) where Iter: Iterator<Item = &'pass dyn Sprite> {
        rpass.set_pipeline(&self.textured_pipeline);
        for sprite in iter {
            sprite.bind(rpass);
            sprite.draw(rpass);
        }
    }
}


/// #### 한국어 </br>
/// 스프라이트 오브젝트의 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a shader module for the sprite object. </br>
/// 
fn create_shader_module(
    device: &wgpu::Device,
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    asset_bundle.get(SPRITE_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Sprite"), device })
}


/// #### 한국어 </br>
/// 사용한 에셋을 해제합니다. </br>
/// 
/// #### English (Translation) </br>
/// Release used assets. </br>
/// 
#[inline]
fn cleanup_assets(asset_bundle: &AssetBundle) {
    asset_bundle.release(SPRITE_SHADER_PATH);
}


/// #### 한국어 </br>
/// 스프라이트 오브젝트 버퍼 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a sprite object buffer bind group layout. </br>
/// 
fn create_buffer_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Buffer(Sprite))"),
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
        }
    )
}


/// #### 한국어 </br>
/// 스프라이트 오브젝트 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a sprite object texture bind group layout. </br>
/// 
fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(Sprite))"),
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
/// 조명에 영향을 받지 않는 스프라이트 오브젝트 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a sprite object rendering pipeline that is not affected by lighting. </br>
/// 
fn create_textured_pipeline(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    render_format: wgpu::TextureFormat,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 스프라이트 오브젝트 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a sprite object rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Color(Sprite))"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 스프라이트 오브젝트 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a sprite object rendering pipeline.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Color(Sprite))"),
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
                        ],
                    },
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
                entry_point: "textured_fs_main",
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
