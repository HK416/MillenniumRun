use std::mem::size_of;
use std::sync::{Mutex, MutexGuard};

use glam::{Vec4, Vec2};
use bytemuck::{Pod, Zeroable};

use crate::components::{
    sprite::Sprite,
    transform::Transform,
};



/// #### 한국어 </br>
/// 텍스처 좌표계 정보를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains texture coordinate system information. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Texcoord {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

impl Default for Texcoord {
    #[inline]
    fn default() -> Self {
        Self { 
            top: 0.0, 
            left: 0.0, 
            bottom: 1.0, 
            right: 1.0 
        }
    }
}



/// #### 한국어 </br>
/// 스프라이트 오브젝트를 렌더링 할 때 사용되는 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data used when rendering sprite objects. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Transform,
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self { transform: Transform::default() }
    }
}



/// #### 한국어 </br>
/// 스프라이트 오브젝트를 렌더링 할 때 사용되는 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data used when rendering sprite objects. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct SpriteData {
    pub texcoord: Texcoord,
    pub color: Vec4,
    pub size: Vec2,
    pub _padding0: [u8; size_of::<f32>() * 2],
}

impl Default for SpriteData {
    #[inline]
    fn default() -> Self {
        Self { 
            texcoord: Texcoord { top: 0.0, left: 0.0, bottom: 1.0, right: 1.0 }, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 }, 
            _padding0: [0u8; size_of::<f32>() * 2],
        }
    }
}


/// #### 한국어 </br>
/// 스프라이트 오브젝트를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder that creates sprite objects. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct SpriteBuilder<'a> {
    pub name: Option<&'a str>,
    pub texcoord: Texcoord,
    pub color: Vec4,
    pub size: Vec2,
    pub tex_sampler: &'a wgpu::Sampler,
    pub texture_view: &'a wgpu::TextureView,
    pub buffer_layout: &'a wgpu::BindGroupLayout,
    pub texture_layout: &'a wgpu::BindGroupLayout, 
}

impl<'a> SpriteBuilder<'a> {
    #[inline]
    pub const fn new(
        name: Option<&'a str>, 
        tex_sampler: &'a wgpu::Sampler,
        texture_view: &'a wgpu::TextureView,
        buffer_layout: &'a wgpu::BindGroupLayout,
        texture_layout: &'a wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            name,
            texcoord: Texcoord { top: 0.0, left: 0.0, bottom: 1.0, right: 1.0 },
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
            size: Vec2 { x: 1.0, y: 1.0 },
            tex_sampler,
            texture_view,
            buffer_layout,
            texture_layout,
        }
    }

    #[inline]
    pub fn with_texcoord(mut self, texcoord: Texcoord) -> Self {
        self.texcoord = texcoord;
        return self;
    }

    #[inline]
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        return self;
    }

    #[inline]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        return self;
    }

    #[inline]
    pub fn build<Iter>(self, device: &wgpu::Device, iter: Iter) -> SpriteObject 
    where Iter: IntoIterator<Item = InstanceData>, Iter::IntoIter: ExactSizeIterator {
        SpriteObject::new(self, device, iter)
    }
}


/// #### 한국어 </br>
/// 스프라이트 이미지 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a sprite image object. </br>
/// 
#[derive(Debug)]
pub struct SpriteObject {
    pub instances: Mutex<Vec<InstanceData>>,
    instance_buffer: wgpu::Buffer,
    pub data: Mutex<SpriteData>,
    sprite_buffer: wgpu::Buffer,
    sprite_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,

}

impl SpriteObject {
    fn new<'a, Iter>(builder: SpriteBuilder<'a>, device: &wgpu::Device, iter: Iter) -> Self 
    where Iter: IntoIterator<Item = InstanceData>, Iter::IntoIter: ExactSizeIterator {
        // (한국어) 라벨 데이터를 생성합니다.
        // (English Translation) Create a label data.
        let label = format!("Sprite({})", builder.name.unwrap_or("Unknown"));

        // (한국어) 스프라이트 오브젝트 인스턴스 데이터 버퍼를 생성합니다.
        // (English Translation) Creates a sprite object instance data buffer.
        use wgpu::util::DeviceExt;
        let instances: Vec<_> = iter.into_iter().collect();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("VertexBuffer(InstanceData({}))", label)),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );
        
        // (한국어) 스프라이트 오브젝트 스프라이트 데이터 버퍼를 생성합니다.
        // (English Translation) Creates a sprite object sprite data buffer.
        let data = SpriteData { 
            texcoord: builder.texcoord, 
            color: builder.color, 
            size: builder.size, 
            ..Default::default()
        };
        let sprite_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("UniformBuffer(SpriteData({}))", label)),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        // (한국어) 스프라이트 바인드 그룹을 생성합니다.
        // (English Translation) Create a sprite bind group.
        let sprite_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BindGroup({})", label)),
                layout: builder.buffer_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            sprite_buffer.as_entire_buffer_binding()
                        ),
                    },
                ]
            }
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group.
        let texture_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BingGroup({})", label)),
                layout: builder.texture_layout,
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
            instances: instances.into(),
            instance_buffer,
            data: data.into(),
            sprite_buffer,
            sprite_bind_group,
            texture_bind_group,
        }
    }

    /// #### 한국어 </br>
    /// 인스턴스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the instance data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update_instance<F>(&self, queue: &wgpu::Queue, mapping_func: F) 
    where F: Fn(&mut MutexGuard<'_, Vec<InstanceData>>) {
        let mut guard = self.instances.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&*guard));
    }

    /// #### 한국어 </br>
    /// 스프라이트 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the sprite data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update_sprite<F>(&self, queue: &wgpu::Queue, mapping_func: F) 
    where F: Fn(&mut MutexGuard<'_, SpriteData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.sprite_buffer, 0, bytemuck::bytes_of(&*guard));
    }
}

impl Sprite for SpriteObject {
    #[inline]
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.sprite_bind_group, &[]);
        rpass.set_bind_group(2, &self.texture_bind_group, &[]);
        rpass.set_vertex_buffer(0, self.instance_buffer.slice(..));
    }

    #[inline]
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}
