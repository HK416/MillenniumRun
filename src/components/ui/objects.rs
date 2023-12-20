use std::sync::{Mutex, MutexGuard};

use glam::{Vec4, Vec3, Mat4, Quat};
use bytemuck::{Pod, Zeroable};
use winit::dpi::PhysicalPosition;

use crate::components::{
    collider2d::Collider2d,
    ui::brush::UiBrush,
    camera::GameCamera,
    anchor::Anchor,
    margin::Margin,
};



/// #### 한국어 </br>
/// 사용자 인터페이스를 렌더링하는데 사용되는 인스턴스 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains instance data used to render the user interface. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Mat4,
    pub anchor: Anchor,
    pub margin: Margin,
    pub color: Vec4,
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self {
            transform: Mat4::IDENTITY,
            anchor: Anchor::default(),
            margin: Margin::default(),
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
        }
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 오브젝트를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder that creates user interface objects. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct UiObjectBuilder<'a> {
    pub name: Option<&'a str>, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub color: Vec4, 
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub tex_sampler: &'a wgpu::Sampler, 
    pub texture_view: &'a wgpu::TextureView, 
    pub ui_brush: &'a UiBrush, 
}

#[allow(dead_code)]
impl<'a> UiObjectBuilder<'a> {
    #[inline]
    pub fn new(
        name: Option<&'a str>, 
        tex_sampler: &'a wgpu::Sampler, 
        texture_view: &'a wgpu::TextureView, 
        ui_brush: &'a UiBrush
    ) -> Self {
        Self {
            name, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            tex_sampler, 
            texture_view, 
            ui_brush, 
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
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        return self;
    }

    #[inline]
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation.normalize();
        return self;
    }

    #[inline]
    pub fn build(self, device: &wgpu::Device) -> UiObject {
        UiObject::new(self, device)
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a user interface object. </br>
/// 
#[derive(Debug)]
pub struct UiObject {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub data: Mutex<InstanceData>,
}

impl UiObject {
    fn new<'a>(builder: UiObjectBuilder<'a>, device: &wgpu::Device) -> Self {
        use wgpu::util::DeviceExt;

        // (한국어) 라벨 데이터를 생성합니다.
        // (English Translation) Create a label data.
        let label = format!("UiObject({})", builder.name.unwrap_or("Unknown"));

        // (한국어) 사용자 인터페이스 데이터 버퍼를 생성합니다.
        // (English Translation) Create a user interface data buffer.
        let data = InstanceData {
            transform: Mat4::from_scale_rotation_translation(
                builder.scale, 
                builder.rotation, 
                builder.translation
            ), 
            anchor: builder.anchor, 
            margin: builder.margin, 
            color: builder.color, 
        };
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertex(InstanceData({}))", label)),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BingGroup(Texture({}))", label)),
                layout: &builder.ui_brush.texture_layout,
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

        Self { 
            buffer, 
            bind_group, 
            data: data.into(), 
        }
    }

    /// #### 한국어 </br>
    /// 사용자 인터페이스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the user interface data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    #[inline]
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F) 
    where F: Fn(&mut MutexGuard<'_, InstanceData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&*guard));
    }

    #[inline]
    pub(super) fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    #[inline]
    pub(super) fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}

impl Collider2d<(&PhysicalPosition<f64>, &GameCamera)> for UiObject {
    fn test(&self, other: &(&PhysicalPosition<f64>, &GameCamera)) -> bool {
        let (pos, view, scale) = {
            let guard = other.1.data.lock().expect("Failed to access variable.");
            (other.0, guard.viewport, guard.scale_factor)
        };
        
        let guard = self.data.lock().expect("Failed to access variable.");
        let anchor = guard.anchor;
        let margin = guard.margin;

        let top = view.y + anchor.top() * view.height + margin.top() as f32 * scale;
        let left = view.x + anchor.left() * view.width + margin.left() as f32 * scale;
        let bottom = view.y + anchor.bottom() * view.height + margin.bottom() as f32 * scale;
        let right = view.x + anchor.right() * view.width + margin.right() as f32 * scale;

        let x = pos.x as f32;
        let y = pos.y as f32;

        return left <= x && x <= right
        && bottom <= y && y <= top;
    }
}
