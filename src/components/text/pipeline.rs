use glam::Mat4;
use ab_glyph::{Font, ScaleFont};

use crate::{
    assets::bundle::AssetBundle,
    nodes::path::FONT_SHADER_PATH,
    render::shader::WgslDecoder,
    system::error::AppResult,
};

use super::{
    section::Section,
    uniform::{
        Uniform,
        UniformBuffer,
    }, 
    texture::{
        TextureMap,
        TextureSampler,
    },
    vertex::VertexInput, 
};



/// #### 한국어 </br>
/// 텍스트 렌더링에 사용되는 렌더링 파이프라인 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the rendering pipeline used for text rendering. </br>
/// 
#[derive(Debug)]
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_variable: UniformBuffer,
    texture_variable: TextureMap,
    sampler_variable: TextureSampler,
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

        // (한국어) 텍스처 쉐이더 변수를 생성합니다.
        // (English Translation) Create a texture shader variable.
        let texture_variable = TextureMap::new(device);

        // (한국어) 샘플러 쉐이더 변수를 생성합니다.
        // (English Translation) Create a sampler shader variable.
        let sampler_variable = TextureSampler::new(device);

        // (한국어) 쉐이더 모듈을 생성합니다.
        // (English Translation) Create a shader module.
        let module = asset_bundle.get(FONT_SHADER_PATH)?
            .read(&WgslDecoder::new(Some("Text - Shader Module"), device))?;

        // (한국어) 파이프라인 레이아웃을 생성합니다.
        // (English Translation) Create a pipeline layout.
        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Text - Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_variable.bind_group_layout,
                    &texture_variable.bind_group_layout,
                    &sampler_variable.bind_group_layout,
                ],
                push_constant_ranges: &[]
            }
        );

        // (한국어) 렌더링 파이프라인을 생성합니다.
        // (English Translation) Create a rendering pipeline.
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Text - Render Pipeline"),
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
                                    offset: offset_of!(VertexInput, width) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32,
                                    offset: offset_of!(VertexInput, height) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: offset_of!(VertexInput, color) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 3,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: (offset_of!(VertexInput, transform) + size_of::<f32>() * 0) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 4,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: (offset_of!(VertexInput, transform) + size_of::<f32>() * 4) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 5,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: (offset_of!(VertexInput, transform) + size_of::<f32>() * 8) as wgpu::BufferAddress,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 6,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: (offset_of!(VertexInput, transform) + size_of::<f32>() * 12) as wgpu::BufferAddress,
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
                    module: &module,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: render_format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        },
                    )],
                }),
                multiview,
            }
        );

        Ok(Self { 
            pipeline, 
            uniform_variable, 
            texture_variable, 
            sampler_variable
        })
    }

    /// #### 한국어 </br>
    /// 텍스트의 유니폼 버퍼를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the uniform buffer of the text. </br>
    /// 
    #[inline]
    pub fn update_uniform(&self, queue: &wgpu::Queue, ortho: Mat4) {
        let uniform = Uniform { ortho };
        self.uniform_variable.update(queue, 0, bytemuck::bytes_of(&uniform))
    }
    
    /// #### 한국어 </br>
    /// 텍스트의 텍스처를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the texture of the text. </br>
    /// 
    #[inline]
    pub fn update_texture<F: Font, SF: ScaleFont<F>>(
        &mut self,
        font: &SF,
        section: &Section,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        for ch in section.all_chars() {
            if ch.buffer().is_some() {
                self.texture_variable.update(ch.char(), font, device, queue);
            }
        }
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
        rpass.set_bind_group(2, &self.sampler_variable.bind_group, &[]);
    } 

    /// #### 한국어 </br>
    /// 텍스트 그리기를 명령합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Commands drawing text. </br>
    /// 
    #[inline]
    pub fn draw<'pass>(&'pass self, section: &'pass Section, rpass: &mut wgpu::RenderPass<'pass>) {
        for ch in section.chars() {
            if let Some(buffer) = ch.buffer() {
                if let Some(bind_group) = self.texture_variable.ref_bind_group(ch.char()) {
                    rpass.set_bind_group(1, bind_group, &[]);
                    rpass.set_vertex_buffer(0, buffer.slice(..));
                    rpass.draw(0..4, 0..1);
                }
            }
        }
    }
}
