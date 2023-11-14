use glam::{Vec4, Vec3};

use crate::components::{
    anchor::Anchor,
    margin::Margin,
    ui::{
        vertex::VertexInput,
        objects::UiObject,
    },
};



/// #### 한국어 </br>
/// 사용자 인터페이스 이미지를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that creates user interface image. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct UiImageBuilder<'a> {
    label: Option<&'a str>,
    anchor: Anchor,
    margin: Margin,
    color: Vec4,
    scale: Vec3,
    depth: f32,
    tex_sampler: &'a wgpu::Sampler,
    texture_view: &'a wgpu::TextureView,
    bind_group_layout: &'a wgpu::BindGroupLayout,
}

#[allow(dead_code)]
impl<'a> UiImageBuilder<'a> {
    #[inline]
    pub fn new(
        label: Option<&'a str>,
        tex_sampler: &'a wgpu::Sampler,
        texture_view: &'a wgpu::TextureView,
        bind_group_layout: &'a wgpu::BindGroupLayout
    ) -> Self {
        Self {
            label,
            anchor: Anchor::default(),
            margin: Margin::default(),
            color: Vec4::ONE,
            scale: Vec3::ONE,
            depth: 0.0,
            tex_sampler,
            texture_view,
            bind_group_layout,
        }
    }

    #[inline]
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        return self;
    }

    #[inline]
    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        return self;
    }

    #[inline]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        return self;
    }

    #[inline]
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        return self;
    }

    #[inline]
    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        return self;
    }

    #[inline]
    pub fn build(self, device: &wgpu::Device) -> UiImage {
        UiImage::new(self, device)
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 이미지 정보를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains user interface image information. </br>
/// 
#[derive(Debug)]
pub struct UiImage {
    pub data: VertexInput,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl UiImage {
    fn new<'a>(builder: UiImageBuilder<'a>, device: &wgpu::Device) -> Self {
        use wgpu::util::DeviceExt;

        let label = match builder.label {
            Some(label) => label.to_string(),
            None => "Unknown".to_string()
        };

        let data = VertexInput {
            anchor: builder.anchor,
            margin: builder.margin,
            color: builder.color,
            scale: builder.scale,
            depth: builder.depth
        };

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Ui({}) - Vertex Input Buffer", label)),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("Ui({}) - Bind Group", label)),
                layout: builder.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(builder.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(builder.tex_sampler),
                    },
                ],
            }
        );

        Self { data, buffer, bind_group }
    }

    /// #### 한국어 </br>
    /// 정점 입력 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the vertex input buffer. </br>
    /// The contents of the buffer are not updated immediately. </br>
    /// 
    #[inline]
    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.data));
    }
}

impl UiObject for UiImage {
    #[inline]
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    #[inline]
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}
