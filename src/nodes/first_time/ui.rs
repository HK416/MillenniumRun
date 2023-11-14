use glam::{Mat4, Vec4, Vec3};

use crate::components::{
    anchor::Anchor, 
    margin::Margin, 
    ui::{
        vertex::VertexInput, 
        objects::{
            UiObject, 
            UiButtonObject, 
        },
    }, 
};



/// #### 한국어 </br>
/// 사용자 인터페이스 버튼을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that creates user interface button. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct UiButtonBuilder<'a> {
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

impl<'a> UiButtonBuilder<'a> {
    #[inline]
    pub fn new(
        label: Option<&'a str>, 
        tex_sampler: &'a wgpu::Sampler,
        texture_view: &'a wgpu::TextureView, 
        bind_group_layout: &'a wgpu::BindGroupLayout,
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
    pub fn build(self, device: &wgpu::Device) -> UiButton {
        UiButton::new(self, device)
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 버튼 정보를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains user interface button information. </br>
/// 
#[derive(Debug)]
pub struct UiButton {
    pub data: VertexInput,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl UiButton {
    fn new<'a>(builder: UiButtonBuilder<'a>, device: &wgpu::Device) -> Self {
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

        // (한국어) 정점 입력 버퍼를 생성합니다.
        // (English Translation) Create a vertex input buffer.
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Ui({}) - Vertex Input Buffer", label)),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 바인드 그룹을 생성합니다.
        // (English Translation) Create a bind group.
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
    /// 버튼의 영역에 교차하는 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns `true` if it intersects the button's area. </br>
    /// 
    pub fn intersects(&self, x: f32, y: f32, ortho: &Mat4) -> bool {
        let width = 2.0 / ortho.x_axis.x;
        let height = 2.0 / ortho.y_axis.y;
        let left = 0.5 * ((-1.0 * ortho.z_axis.x * width) - width);
        let bottom = 0.5 * ((-1.0 * ortho.z_axis.y * height) + height);

        let anchor = self.data.anchor;
        let margin = self.data.margin;
        let px_top = bottom + height * anchor.top() + margin.top() as f32;
        let px_left = left + width * anchor.left() + margin.left() as f32;
        let px_bottom = bottom + height * anchor.bottom() + margin.bottom() as f32;
        let px_right = left + width * anchor.right() + margin.right() as f32;

        let px_x = left + x * width;
        let px_y = bottom + y * height;

        return px_left <= px_x && px_x <= px_right 
        && px_bottom <= px_y && px_y <= px_top;
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

impl UiObject for UiButton {
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

impl UiButtonObject for UiButton {
    #[inline]
    fn mouse_pressed(&self, x: f32, y: f32, ortho: &Mat4) -> bool {
        self.intersects(x, y, ortho)
    }

    #[inline]
    fn mouse_released(&self, x: f32, y: f32, ortho: &Mat4) -> bool {
        self.intersects(x, y, ortho)
    }
}
