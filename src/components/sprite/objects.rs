use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec4, Vec2, Vec3, Quat};

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
    pub transform: Mat4,
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self { transform: Mat4::IDENTITY }
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
pub struct SpriteData {
    pub texcoord: Texcoord,
    pub color: Vec4,
    pub size: Vec2,
}

impl Default for SpriteData {
    #[inline]
    fn default() -> Self {
        Self { 
            texcoord: Texcoord { top: 0.0, left: 0.0, bottom: 1.0, right: 1.0 }, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
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
    pub size: Vec2,
    pub color: Vec4,
    pub scale: Vec3,
    pub rotation: Quat,
    pub translation: Vec3,
    pub tex_sampler: &'a wgpu::Sampler,
    pub texture_view: &'a wgpu::TextureView,
    pub texture_layout: &'a wgpu::BindGroupLayout, 
}

impl<'a> SpriteBuilder<'a> {
    #[inline]
    pub const fn new(
        name: Option<&'a str>, 
        tex_sampler: &'a wgpu::Sampler,
        texture_view: &'a wgpu::TextureView,
        texture_layout: &'a wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            name,
            texcoord: Texcoord { top: 0.0, left: 0.0, bottom: 1.0, right: 1.0 },
            size: Vec2 { x: 1.0, y: 1.0 },
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            tex_sampler,
            texture_view,
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
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        return self;
    }

    #[inline]
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        return self;
    }

    #[inline]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        return self;
    }

    #[inline]
    pub fn build(self, device: &wgpu::Device) -> SpriteObject {
        SpriteObject::new(self, device)
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
    pub data: SpriteData,
    pub transform: Transform,
    instance_buffer: wgpu::Buffer,
    sprite_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl SpriteObject {
    fn new<'a>(builder: SpriteBuilder<'a>, device: &wgpu::Device) -> Self {
        // (한국어) 라벨 데이터를 생성합니다.
        // (English Translation) Create a label data.
        let label = format!("SpriteObject({})", builder.name.unwrap_or("Unknown"));

        // (한국어) 스프라이트 오브젝트 인스턴스 데이터 버퍼를 생성합니다.
        // (English Translation) Creates a sprite object instance data buffer.
        use wgpu::util::DeviceExt;
        let transform = Transform::from(
            Mat4::from_scale_rotation_translation(
                builder.scale,
                builder.rotation,
                builder.translation
            )
        ); 
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("InstanceData({})", label)),
                contents: bytemuck::bytes_of(&InstanceData { transform: transform.into() }),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );
        
        // (한국어) 스프라이트 오브젝트 스프라이트 데이터 버퍼를 생성합니다.
        // (English Translation) Creates a sprite object sprite data buffer.
        let data = SpriteData { 
            texcoord: builder.texcoord, 
            color: builder.color, 
            size: builder.size 
        };
        let sprite_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("SpriteData({})", label)),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group.
        let bind_group = device.create_bind_group(
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
            data,
            transform, 
            instance_buffer, 
            sprite_buffer, 
            bind_group, 
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
    #[inline]
    pub fn update_instance(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.instance_buffer, 
            0, 
            bytemuck::bytes_of(&InstanceData { transform: self.transform.into() }
        ))
    }

    /// #### 한국어 </br>
    /// 스프라이트 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the sprite data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    #[inline]
    pub fn update_sprite(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.sprite_buffer, 0, bytemuck::bytes_of(&self.data))
    }
}

impl Sprite for SpriteObject {
    #[inline]
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        rpass.set_vertex_buffer(1, self.sprite_buffer.slice(..));
    }

    #[inline]
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}
