use std::ops::Range;

use crate::render::{
    descriptor::RenderPassDesc,
    identifier::IDHandle,
};


#[derive(Debug, Clone)]
pub enum DrawCommand {
    SetPipeline {
        pipeline: IDHandle,
    },

    SetIndexBuffer {
        format: wgpu::IndexFormat,
        buffer: IDHandle,
        buffer_range: Range<wgpu::BufferAddress>,
    },

    SetVertexBuffer {
        slot: u32,
        buffer: IDHandle,
        buffer_range: Range<wgpu::BufferAddress>,
    },

    Draw { 
        vertices: Range<u32>, 
        instances: Range<u32>, 
    },

    DrawIndexed {
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    }
}


#[derive(Debug, Clone)]
pub struct SubmitRenderPass {
    pub desc: RenderPassDesc,
    pub commands: Vec<DrawCommand>,
}
