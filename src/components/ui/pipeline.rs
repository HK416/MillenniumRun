use glam::Mat4;

use crate::{
    assets::bundle::AssetBundle, 
    nodes::path::UI_SHADER_PATH,
    render::shader::WgslDecoder,
    system::error::AppResult, 
};

use super::{
    uniform::{
        Uniform,
        UniformBuffer, 
    }, 
    vertex::VertexInput,
};



#[derive(Debug)]
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_variable: UniformBuffer,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        multiview: Option<std::num::NonZeroU32>,
        asset_bundle: &AssetBundle,
        ortho: Mat4,
    ) -> AppResult<Self> {
        use std::mem::size_of;
        use bytemuck::offset_of;

        // (한국어) 유니폼 쉐이더 변수를 생성합니다.
        // (English Translation) Create a uniform shader variable.
        let uniform = Uniform { ortho };
        let uniform_variable = UniformBuffer::new(device, &uniform);


        // (한국어) 텍스처 바인드 그룹 레이아웃을 생성합니다.
        // (English Translation) Create a texture bind group layout.
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("User Interface - Texture Bind Group Layout."),
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
                        count: None,
                    },
                ],
            }
        );

        // (한국어) 쉐이더 모듈을 생성합니다.
        // (English Translation) Create a shader module.
        let module = asset_bundle.get(UI_SHADER_PATH)?
            .read(&WgslDecoder::new(Some("User Interface - Shader Module"), device))?;

        // (한국어) 파이프라인 레이아웃을 생성합니다.
        // (English Translation) Create a pipeline layout.
        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("User Interface - Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_variable.bind_group_layout,
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[]
            }
        );

        // (한국어) 렌더링 파이프라인을 생성합니다.
        // (English Translation) Create a rendering pipeline.
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("User Interface - Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: "vs_main",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<VertexInput>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: (offset_of!(VertexInput, anchor) + size_of::<f32>() * 0) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: (offset_of!(VertexInput, anchor) + size_of::<f32>() * 1) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: (offset_of!(VertexInput, anchor) + size_of::<f32>() * 2) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 3,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: (offset_of!(VertexInput, anchor) + size_of::<f32>() * 3) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 4,
                                    format: wgpu::VertexFormat::Sint32,
                                    offset: (offset_of!(VertexInput, margin) + size_of::<i32>() * 0) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 5,
                                    format: wgpu::VertexFormat::Sint32,
                                    offset: (offset_of!(VertexInput, margin) + size_of::<i32>() * 1) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 6,
                                    format: wgpu::VertexFormat::Sint32,
                                    offset: (offset_of!(VertexInput, margin) + size_of::<i32>() * 2) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 7,
                                    format: wgpu::VertexFormat::Sint32,
                                    offset: (offset_of!(VertexInput, margin) + size_of::<i32>() * 3) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 8,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: offset_of!(VertexInput, color) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 9,
                                    format: wgpu::VertexFormat::Float32x3,
                                    offset: offset_of!(VertexInput, scale) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 10,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: offset_of!(VertexInput, depth) as wgpu::BufferAddress,
                                },
                            ]
                        }
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
                    module: &module, 
                    entry_point: "fs_main", 
                    targets: &[
                        Some(wgpu::ColorTargetState { 
                            format: render_format, 
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                            write_mask: wgpu::ColorWrites::ALL 
                        }
                    )], 
                }),
                multiview,
            }
        );

        Ok(Self { 
            pipeline, 
            uniform_variable, 
            texture_bind_group_layout 
        })
    }

    /// #### 한국어 </br>
    /// 유저 인터페이스의 유니폼 버퍼를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the uniform buffer of the user interface. </br>
    /// 
    #[inline]
    pub fn update_uniform(&self, queue: &wgpu::Queue, ortho: Mat4) {
        let uniform = Uniform { ortho };
        self.uniform_variable.update(queue, 0, bytemuck::bytes_of(&uniform));
    }

    /// #### 한국어 </br>
    /// 렌더 패스에 렌더링 파이프라인과 바인딩 그룹을 바인딩 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Bind the rendering pipeline and binding group to the render pass.
    /// 
    #[inline]
    pub fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.uniform_variable.bind_group, &[]);
    } 
}