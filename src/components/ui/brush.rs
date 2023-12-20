use std::sync::Arc;
use std::mem::size_of;

use glam::Mat4;
use bytemuck::offset_of;

use crate::{
    assets::bundle::AssetBundle,
    components::ui::objects::{
        UiObject,
        InstanceData, 
    },
    render::shader::WgslDecoder,
    system::error::AppResult,
};


/// #### 한국어 </br>
/// 사용자 인터페이스를 화면에 그리는 브러쉬 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a brush that draws the user interface on the screen. </br>
/// 
#[derive(Debug)]
pub struct UiBrush {
    pipeline: wgpu::RenderPipeline,
    pub texture_layout: wgpu::BindGroupLayout,
}

impl UiBrush {
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
            texture_layout,
        }.into())
    }

    /// #### 한국어 </br>
    /// 주어진 사용자 인터페이스 오브젝트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given user interface objects on the screen. </br>
    /// 
    pub fn draw<'pass, Iter>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>, iter: Iter) 
    where Iter: Iterator<Item = &'pass UiObject> {
        rpass.set_pipeline(&self.pipeline);
        for ui in iter {
            ui.bind(rpass);
            ui.draw(rpass);
        }
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스의 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a shader module for the user interface. </br>
/// 
fn create_shader_module(
    device: &wgpu::Device,
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::UI_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Ui"), device })?;
    asset_bundle.release(path::UI_SHADER_PATH);
    return Ok(module);
}



/// #### 한국어 </br>
/// 사용자 인터페이스 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface bind group layout. </br>
/// 
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(UserInterface)))"),
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
                },
            ],
        },
    )
}


/// #### 한국어 </br>
/// 사용자 인터페이스 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface rendering pipeline. </br>
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
    // (한국어) 사용자 인터페이스 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a user interface rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Ui)"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 사용자 인터페이스 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a user interface rendering pipeline.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Ui)"),
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
                                offset: (offset_of!(InstanceData, anchor) + size_of::<f32>() * 0) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32,
                                offset: (offset_of!(InstanceData, anchor) + size_of::<f32>() * 1) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32,
                                offset: (offset_of!(InstanceData, anchor) + size_of::<f32>() * 2) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32,
                                offset: (offset_of!(InstanceData, anchor) + size_of::<f32>() * 3) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 8,
                                format: wgpu::VertexFormat::Sint32,
                                offset: (offset_of!(InstanceData, margin) + size_of::<i32>() * 0) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 9,
                                format: wgpu::VertexFormat::Sint32,
                                offset: (offset_of!(InstanceData, margin) + size_of::<i32>() * 1) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 10,
                                format: wgpu::VertexFormat::Sint32,
                                offset: (offset_of!(InstanceData, margin) + size_of::<i32>() * 2) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 11,
                                format: wgpu::VertexFormat::Sint32,
                                offset: (offset_of!(InstanceData, margin) + size_of::<i32>() * 3) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 12,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(InstanceData, color) as wgpu::BufferAddress,
                            },
                        ]
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
