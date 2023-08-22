use bytemuck::{Zeroable, Pod};
use wgpu::util::DeviceExt;



#[derive(Debug)]
pub struct IndexBuffer {
    index_count: u32,
    index_buffer: wgpu::Buffer,
    index_format: wgpu::IndexFormat,
}

impl IndexBuffer {
    pub fn from_indices_u16(
        label: Option<&str>,
        device: &wgpu::Device,
        indices: &[u16],
    ) -> Self {
        let index_count = indices.len() as u32;
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_format = wgpu::IndexFormat::Uint16;

        Self { index_count, index_buffer, index_format }
    }

    pub fn from_indices_u32(
        label: Option<&str>,
        device: &wgpu::Device,
        indices: &[u32]
    ) -> Self {
        let index_count = indices.len() as u32;
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_format = wgpu::IndexFormat::Uint32;

        Self { index_count, index_buffer, index_format }
    }
}



#[derive(Debug)]
pub struct VertexBuffer {
    vertex_count: u32,
    vertex_stride: u32,
    vertex_buffer: wgpu::Buffer,
    attributes: Vec<wgpu::VertexAttribute>,
    input_rate: wgpu::VertexStepMode,
}

impl VertexBuffer {
    pub fn from_vertices<Vertex, Attributes>(
        label: Option<&str>,
        device: &wgpu::Device,
        vertices: &[Vertex],
        location: u32,
        attributes: Attributes,
        input_rate: wgpu::VertexStepMode,
    ) -> Self 
    where 
        Vertex: Zeroable + Pod,
        Attributes: IntoIterator<Item = (wgpu::VertexFormat, u64)>,
        Attributes::IntoIter: ExactSizeIterator
    {
        let vertex_count = vertices.len() as u32;
        let vertex_stride = std::mem::size_of::<Vertex>() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let attributes = attributes.into_iter()
            .enumerate()
            .map(|(idx, (format, offset))| {
                wgpu::VertexAttribute { 
                    format,
                    offset,
                    shader_location: location + idx as u32,
                }
            })
            .collect();
        
        Self { vertex_count, vertex_stride, vertex_buffer, attributes, input_rate }
    }
}
