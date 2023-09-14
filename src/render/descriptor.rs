use std::borrow::Cow;
use std::num::NonZeroU32;

use crate::{
    app::abort::AppResult,
    render::{
        objects::{
            BindGroupLayoutPool,
            BufferPool,
            PipelineLayoutPool,
            RenderPipelinePool,
            ShaderModulePool,
            TextureViewPool,
            TexturePool,
            utils::{
                ref_bind_group_layout_obj,
                ref_buffer_obj,
                ref_pipeline_layout_obj,
                ref_shader_module_obj,
                ref_texture_obj,
                ref_texture_view_obj,
            }
        },
        identifier::IDHandle,
    },
};



/// #### 한국어 </br>
/// [`wgpu::ShaderSource`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::ShaderSource`]. </br>
/// 
#[derive(Debug, Clone)]
pub enum ShaderSourceDesc {
    SpirV(Vec<u32>),
    Wgsl(String),
}

/// #### 한국어 </br>
/// [`wgpu::ShaderModuleDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::ShaderModuleDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct ShaderModuleDesc {
    pub label: Option<String>,
    pub source: ShaderSourceDesc,
}

impl ShaderModuleDesc {
    pub fn build<'a>(
        &'a self,
        device: &wgpu::Device,
        pool: &mut ShaderModulePool,
    ) -> IDHandle {
        pool.insert(
            device, 
            wgpu::ShaderModuleDescriptor { 
                label: self.label.as_deref(), 
                source: match &self.source {
                    ShaderSourceDesc::SpirV(src) => wgpu::ShaderSource::SpirV(Cow::from(src)),
                    ShaderSourceDesc::Wgsl(src) => wgpu::ShaderSource::Wgsl(Cow::from(src)),
                },
            }
        )
    }
}



/// #### 한국어 </br>
/// [`wgpu::VertexBufferLayout`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::VertexBufferLayout`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct VertexBufferLayoutDesc {
    pub array_stride: wgpu::BufferAddress,
    pub step_mode: wgpu::VertexStepMode,
    pub attributes: Vec<wgpu::VertexAttribute>,
}

/// #### 한국어 </br>
/// [`wgpu::VertexState`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::VertexState`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct VertexStateDesc {
    pub module: IDHandle,
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayoutDesc>
}

/// #### 한국어 </br>
/// [`wgpu::FragmentState`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::FragmentState`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct FragmentStateDesc {
    pub module: IDHandle,
    pub entry_point: String,
    pub targets: Vec<Option<wgpu::ColorTargetState>>,
}

/// #### 한국어 </br>
/// [`wgpu::RenderPipelineDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::RenderPipelineDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct RenderPipelineDesc {
    pub label: Option<String>,
    pub layout: Option<IDHandle>,
    pub vertex: VertexStateDesc,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub fragment: Option<FragmentStateDesc>,
    pub multiview: Option<NonZeroU32>,
}

impl RenderPipelineDesc {
    pub fn build<'a>(
        &'a self,
        device: &wgpu::Device,
        layouts: &PipelineLayoutPool,
        shaders: &ShaderModulePool,
        pool: &mut RenderPipelinePool,
    ) -> AppResult<IDHandle> {
        let layout = match &self.layout {
            Some(id) => Some(ref_pipeline_layout_obj(id, layouts)?.as_ref()),
            None => None,
        };

        let buffers: Vec<_> = self.vertex
            .buffers
            .iter()
            .map(|ci| wgpu::VertexBufferLayout {
                array_stride: ci.array_stride.clone(),
                step_mode: ci.step_mode.clone(),
                attributes: &ci.attributes,
            })
            .collect();
        let vertex = wgpu::VertexState {
            module: ref_shader_module_obj(&self.vertex.module, shaders)?.as_ref(),
            entry_point: &self.vertex.entry_point,
            buffers: &buffers
        };

        let fragment = match &self.fragment {
            Some(ci) => Some(wgpu::FragmentState { 
                module: ref_shader_module_obj(&ci.module, shaders)?.as_ref(),
                entry_point: &ci.entry_point, 
                targets: &ci.targets 
            }),
            None => None,
        };

        Ok(pool.insert(
            device, 
            &wgpu::RenderPipelineDescriptor { 
                label: self.label.as_deref(), 
                layout,
                vertex, 
                primitive: self.primitive.clone(), 
                depth_stencil: self.depth_stencil.clone(), 
                multisample: self.multisample.clone(), 
                fragment, 
                multiview: self.multiview.clone(), 
            }
        ))
    }
}



/// #### 한국어 </br>
/// [`wgpu::BindGroupLayoutDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::BindGroupLayoutDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct BindGroudLayoutDesc {
    pub label: Option<String>,
    pub entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindGroudLayoutDesc {
    pub fn build<'a>(
        &'a self,
        device: &wgpu::Device,
        pool: &mut BindGroupLayoutPool,
    ) -> IDHandle {
        pool.insert(
            device, 
            &wgpu::BindGroupLayoutDescriptor { 
                label: self.label.as_deref(), 
                entries: &self.entries 
            }
        )
    }
}



/// #### 한국어 </br>
/// [`wgpu::BufferDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::BufferDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub label: Option<String>,
    pub size: wgpu::BufferAddress,
    pub usage: wgpu::BufferUsages,
    pub mapped_at_creation: bool,
}

impl BufferDesc {
    pub fn build<'a>(
        &'a self,
        device: &wgpu::Device,
        pool: &'a mut BufferPool
    ) -> IDHandle {
        pool.insert(
            device, 
            &wgpu::BufferDescriptor {
                label: self.label.as_deref(),
                size: self.size,
                usage: self.usage,
                mapped_at_creation: self.mapped_at_creation,
            }
        )
    }
}



/// #### 한국어 </br>
/// [`wgpu::util::BufferInitDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::util::BufferInitDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct BufferInitDesc {
    pub label: Option<String>,
    pub contents: Vec<u8>,
    pub usage: wgpu::BufferUsages,
}

impl BufferInitDesc {
    pub fn build<'a>(
        &'a self,
        device: &wgpu::Device,
        pool: &'a mut BufferPool,
    ) -> IDHandle {
        pool.insert_with_init(
            device, 
            &wgpu::util::BufferInitDescriptor {
                label: self.label.as_deref(),
                contents: &self.contents,
                usage: self.usage
            }
        )
    }
}



/// #### 한국어 </br>
/// [`wgpu::PipelineLayoutDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::PipelineLayoutDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct PipelineLayoutDesc {
    pub label: Option<String>,
    pub bind_group_layouts: Vec<IDHandle>,
    pub push_constant_ranges: Vec<wgpu::PushConstantRange>,
}

impl PipelineLayoutDesc {
    pub fn build<'a>(
        &'a self,
        device: &'a wgpu::Device,
        layouts: &'a BindGroupLayoutPool,
        pool: &'a mut PipelineLayoutPool,
    ) -> AppResult<IDHandle> {
        let mut bind_group_layouts = Vec::with_capacity(self.bind_group_layouts.len());
        for id in self.bind_group_layouts.iter() {
            bind_group_layouts.push(ref_bind_group_layout_obj(id, layouts)?.as_ref());
        }

        Ok(pool.insert(
            device, 
            &wgpu::PipelineLayoutDescriptor { 
                label: self.label.as_deref(), 
                bind_group_layouts: &bind_group_layouts, 
                push_constant_ranges: &self.push_constant_ranges 
            }
        ))
    }
}



/// #### 한국어 </br>
/// [`wgpu::RenderPassColorAttachment`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::RenderPassColorAttachment`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct RenderPassColorAttachmentDesc {
    pub view: Option<IDHandle>,
    pub resolve_target: Option<IDHandle>,
    pub ops: wgpu::Operations<wgpu::Color>,
}

/// #### 한국어 </br>
/// [`wgpu::RenderPassDepthStencilAttachment`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::RenderPassDepthStencilAttachment`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct RenderPassDepthStencilAttachmentDesc {
    pub view: IDHandle,
    pub depth_ops: Option<wgpu::Operations<f32>>,
    pub stencil_ops: Option<wgpu::Operations<u32>>,
}

/// #### 한국어 </br>
/// [`wgpu::RenderPassDescriptor`]의 래퍼 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a wrapper data type for [`wgpu::RenderPassDescriptor`]. </br>
/// 
#[derive(Debug, Clone)]
pub struct RenderPassDesc {
    pub label: Option<String>,
    pub color_attachments: Vec<Option<RenderPassColorAttachmentDesc>>,
    pub depth_stencil_attachments: Option<RenderPassDepthStencilAttachmentDesc>,
}

impl RenderPassDesc {
    pub fn begin<'a>(
        &'a self,
        view: &'a wgpu::TextureView,
        views: &'a TextureViewPool,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> AppResult<wgpu::RenderPass<'a>> {
        // TODO: Texture view not implemented yet...
        let mut color_attachments = Vec::with_capacity(self.color_attachments.len());
        for attachment in self.color_attachments.iter() {
            color_attachments.push(match attachment {
                Some(ci) => Some(wgpu::RenderPassColorAttachment {
                    view: match &ci.view {
                        Some(id) => ref_texture_view_obj(id, views)?.as_ref(),
                        None => view,
                    },
                    ops: ci.ops,
                    resolve_target: match &ci.resolve_target {
                        Some(id) => Some(ref_texture_view_obj(id, views)?.as_ref()),
                        None => None
                    }
                }),
                None => None,
            });
        }

        let depth_stencil_attachment = match &self.depth_stencil_attachments {
            Some(ci) => Some(wgpu::RenderPassDepthStencilAttachment {
                view: ref_texture_view_obj(&ci.view, views)?.as_ref(),
                depth_ops: ci.depth_ops,
                stencil_ops: ci.stencil_ops
            }),
            None => None,
        };

        Ok(encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: self.label.as_deref(), 
            color_attachments: &color_attachments, 
            depth_stencil_attachment,
        }))
    }
}



/// #### 한국어
/// 복사 명령어 설명자 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a copy command descriptor. </br>
/// 
#[derive(Debug, Clone)]
pub enum CopyDesc {
    BufferToBuffer {
        src: IDHandle,
        src_offset: wgpu::BufferAddress,
        dst: IDHandle,
        dst_offset: wgpu::BufferAddress,
        size: wgpu::BufferAddress,
    },
    BufferToTexture {
        buffer: IDHandle,
        layout: wgpu::ImageDataLayout,
        texture: IDHandle,
        mip_level: u32,
        origin: wgpu::Origin3d,
        aspect: wgpu::TextureAspect,
        size: wgpu::Extent3d,
    },
    TextureToBuffer {
        texture: IDHandle,
        mip_level: u32,
        origin: wgpu::Origin3d,
        aspect: wgpu::TextureAspect,
        buffer: IDHandle,
        layout: wgpu::ImageDataLayout,
        size: wgpu::Extent3d
    },
    TextureToTexture {
        src_texture: IDHandle,
        src_mip_level: u32,
        src_origin: wgpu::Origin3d,
        src_aspect: wgpu::TextureAspect,
        dst_texture: IDHandle,
        dst_mip_level: u32,
        dst_origin: wgpu::Origin3d,
        dst_aspect: wgpu::TextureAspect,
        size: wgpu::Extent3d,
    },
}

impl CopyDesc {
    pub fn copy(
        &self,
        buffers: &BufferPool,
        textures: &TexturePool,
        encoder: &mut wgpu::CommandEncoder
    ) -> AppResult<()> {
        match self {
            Self::BufferToBuffer { src, src_offset, dst, dst_offset, size } => {
                encoder.copy_buffer_to_buffer(
                    ref_buffer_obj(src, buffers)?.as_ref(), 
                    src_offset.clone(), 
                    ref_buffer_obj(dst, buffers)?.as_ref(), 
                    dst_offset.clone(), 
                    size.clone()
                );
            },
            Self::BufferToTexture { buffer, layout, texture, mip_level, origin, aspect, size } => {
                encoder.copy_buffer_to_texture(
                    wgpu::ImageCopyBuffer {
                        buffer: ref_buffer_obj(buffer, buffers)?.as_ref(),
                        layout: layout.clone()
                    }, 
                    wgpu::ImageCopyTextureBase { 
                        texture: ref_texture_obj(texture, textures)?.as_ref(), 
                        mip_level: mip_level.clone(), 
                        origin: origin.clone(), 
                        aspect: aspect.clone() 
                    }, 
                    size.clone()
                );
            },
            Self::TextureToBuffer { texture, mip_level, origin, aspect, buffer, layout, size } => {
                encoder.copy_texture_to_buffer(
                    wgpu::ImageCopyTextureBase { 
                        texture: ref_texture_obj(texture, textures)?.as_ref(), 
                        mip_level: mip_level.clone(), 
                        origin: origin.clone(), 
                        aspect: aspect.clone() 
                    }, 
                    wgpu::ImageCopyBufferBase { 
                        buffer: ref_buffer_obj(buffer, buffers)?.as_ref(), 
                        layout: layout.clone() 
                    }, 
                    size.clone()
                );
            },
            Self::TextureToTexture { src_texture, src_mip_level, src_origin, src_aspect, dst_texture, dst_mip_level, dst_origin, dst_aspect, size } => {
                encoder.copy_texture_to_texture(
                    wgpu::ImageCopyTextureBase { 
                        texture: ref_texture_obj(src_texture, textures)?.as_ref(), 
                        mip_level: src_mip_level.clone(), 
                        origin: src_origin.clone(), 
                        aspect: src_aspect.clone() 
                    }, 
                    wgpu::ImageCopyTextureBase { 
                        texture: ref_texture_obj(dst_texture, textures)?.as_ref(), 
                        mip_level: dst_mip_level.clone(), 
                        origin: dst_origin.clone(), 
                        aspect: dst_aspect.clone() 
                    }, 
                    size.clone()
                );
            }
        }
        Ok(())
    }
}
