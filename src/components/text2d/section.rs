//! #### 한국어 </br>
//! 2차원 텍스트 구획에 관한 자료형들을 정의합니다. </br>
//! 
//! #### English (Translation) </br>
//! Defines data types for two-dimensional text sections. </br>
//!
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;

use ab_glyph::{Font, ScaleFont};
use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec4, Vec3, Vec2, Quat};

use crate::components::{
    text2d::{
        brush::Text2dBrush,
        character::{
            TextAlign, 
            Character,
            InstanceData,
            CharContext, 
        },
    },
    anchor::Anchor,
    margin::Margin,
};

const FONT_SCALE: f32 = 128.0;



/// #### 한국어 </br>
/// 2차원 텍스트 구획의 유니폼 데이터를 담고있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains uniform data for a two-dimensional text section. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct UniformData {
    pub transform: Mat4,
    pub anchor: Anchor,
    pub margin: Margin,
    pub color: Vec4,
}

impl Default for UniformData {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Mat4::IDENTITY, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        }
    }
}



/// #### 한국어 </br>
/// 2차원 텍스트 구획을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that creates two-dimensional text sections. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct Section2dBuilder<'a, F: Font> {
    pub name: Option<&'a str>, 
    pub font: &'a F, 
    pub text: &'a str, 
    pub color: Vec4, 
    pub align: TextAlign, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub text_brush: &'a Text2dBrush, 
}

#[allow(dead_code)]
impl<'a, F: Font> Section2dBuilder<'a, F> {
    #[inline]
    pub fn new(
        name: Option<&'a str>, 
        font: &'a F, 
        text: &'a str, 
        text_brush: &'a Text2dBrush
    ) -> Self {
        Self { 
            name,
            font, 
            text, 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            align: TextAlign::default(), 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            text_brush, 
        }
    }

    #[inline]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        return self;
    }

    #[inline]
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        return self;
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
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        return self;
    }

    #[inline]
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation.normalize();
        return self;
    }

    #[inline]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        return self;
    }

    #[inline]
    pub fn build(self, device: &wgpu::Device, queue: &wgpu::Queue) -> Section2d {
        Section2d::new(self, device, queue)
    }
}



/// #### 한국어 </br>
/// 2차원 텍스트를 렌더링 할 때 사용되는 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data used when rendering two-dimensional text. </br>
/// 
#[derive(Debug)]
pub struct Section2d {
    characters: Vec<Character>, 
    texture_bind_groups: HashMap<char, wgpu::BindGroup>, 
    buffer_bind_group: wgpu::BindGroup, 
    buffer: wgpu::Buffer, 
    pub data: Mutex<UniformData>,
}

#[allow(dead_code)]
impl Section2d {
    fn new<'a, F: Font>(
        builder: Section2dBuilder<'a, F>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let len = builder.text.trim().chars().count();
        assert!(len > 0, "The length of the given text must be greater then zero.");

        let data = UniformData {
            transform: Mat4::from_scale_rotation_translation(
                builder.scale,
                builder.rotation,
                builder.translation
            ), 
            anchor: builder.anchor, 
            margin: builder.margin, 
            color: builder.color, 
        };
        let buffer = create_section_data_buffer(builder.name, &data, device);
        let buffer_bind_group = create_section_data_bind_group(
            builder.name, 
            device, 
            &buffer, 
            &builder.text_brush.buffer_layout
        );

        let font = builder.font.as_scaled(FONT_SCALE);
        let (origin, offset_x, offset_y) = get_text_align_fn(builder.align);
        let (characters, texture_bind_groups) = create_characters(
            builder.name, 
            font, 
            builder.text, 
            origin, 
            device, 
            queue, 
            &builder.text_brush.tex_sampler, 
            &builder.text_brush.texture_layout, 
            offset_x, 
            offset_y
        );

        Self { 
            characters, 
            texture_bind_groups, 
            buffer_bind_group, 
            buffer, 
            data: data.into(),
        }
    }

    /// #### 한국어 </br>
    /// 텍스트의 문자를 빌립니다. (reference) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrow a character from a text. (reference) </br>
    /// 
    #[inline]
    pub fn chars(&self) -> &[Character] {
        &self.characters
    }

    /// #### 한국어 </br>
    /// 텍스트의 문자를 빌립니다. (mutable) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrow a character from a text. (mutable) </br>
    /// 
    #[inline]
    pub fn chars_mut(&mut self) -> &mut [Character] {
        &mut self.characters
    }

    /// #### 한국어 </br>
    /// 구획 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the section data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F) 
    where F: Fn(&mut MutexGuard<'_, UniformData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&*guard));
    }

    #[inline]
    pub(super) fn bind_buffer<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.buffer_bind_group, &[]);
    }

    #[inline]
    pub(super) fn bind_texture<'pass>(&'pass self, ch: &char, rpass: &mut wgpu::RenderPass<'pass>) {
        if let Some(bind_group) = self.texture_bind_groups.get(ch) {
            rpass.set_bind_group(2, &bind_group, &[]);
        }
    }
}



/// #### 한국어 </br>
/// 텍스트 구획 데이터 버퍼를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a text section data buffer. </br>
/// 
fn create_section_data_buffer(
    name: Option<&str>, 
    data: &UniformData, 
    device: &wgpu::Device
) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;

    // (한국어) 라벨 데이터를 생성합니다.
    // (English Translation) Create a label data.
    let label = format!("Text2d({})", name.unwrap_or("Unknown"));

    // (한국어) 텍스트 구획 데이터 버퍼를 생성합니다.
    // (English Translation) Create a text section data buffer.
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Uniform({})", label)),
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }
    )
}


/// #### 한국어 </br>
/// 텍스트 구획 데이터 바인드 그룹을 생성합니다. </br>
/// 
/// #### English (Translation) </br> 
/// Create a text section data bind group. </br>
/// 
fn create_section_data_bind_group(
    name: Option<&str>, 
    device: &wgpu::Device, 
    buffer: &wgpu::Buffer, 
    buffer_layout: &wgpu::BindGroupLayout
) -> wgpu::BindGroup {
    // (한국어) 라벨 데이터를 생성합니다.
    // (English Translation) Create a label data.
    let label = format!("Text2d({})", name.unwrap_or("Unknown"));

    // (한국어) 텍스트 구획 데이터 바인드 그룹을 생성합니다.
    // (English Translation) Create a text section data bind group.
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: Some(&format!("BindGroup(Uniform({}))", label)),
            layout: buffer_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        buffer.as_entire_buffer_binding()
                    ),
                },
            ],
        }
    )
}



type OffsetFn = dyn Fn(f32, f32) -> f32;

#[inline]
fn offset_center(x: f32, v: f32) -> f32 {
    x - 0.5 * v
}

#[inline]
fn offset_left(x: f32, _w: f32) -> f32 {
    x
}

#[inline]
fn offset_right(x: f32, w: f32) -> f32 {
    x - w
}

#[inline    ]
fn offset_top(y: f32, _h: f32) -> f32 {
    y
}

#[inline]
fn offset_bottom(y: f32, h: f32) -> f32 {
    y - h
}

#[inline]
fn get_text_align_fn(align: TextAlign) -> (Vec2, &'static OffsetFn, &'static OffsetFn) {
    match align {
        TextAlign::TopLeft(origin) => (origin, &offset_left, &offset_top),
        TextAlign::TopCenter(origin) => (origin, &offset_center, &offset_top),
        TextAlign::TopRight(origin) => (origin, &offset_right, &offset_top),
        TextAlign::Center(origin) => (origin, &offset_center, &offset_center),
        TextAlign::BottomLeft(origin) => (origin, &offset_left, &offset_bottom),
        TextAlign::BottomCenter(origin) => (origin, &offset_center, &offset_bottom),
        TextAlign::BottomRight(origin) => (origin, &offset_right, &offset_bottom),
    }
}


/// #### 한국어 </br>
/// 텍스트를 그릴때 사용되는 글리프 데이터와 텍스처 데이터를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates glyph data and texture data used when drawing text. </br>
/// 
fn create_characters<F, SF>(
    name: Option<&str>,
    font: SF,
    text: &str,
    origin: Vec2,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex_sampler: &wgpu::Sampler,
    texture_layout: &wgpu::BindGroupLayout,
    offset_x: &'static dyn Fn(f32, f32) -> f32,
    offset_y: &'static dyn Fn(f32, f32) -> f32,
) -> (Vec<Character>, HashMap<char, wgpu::BindGroup>)
where F: Font, SF: ScaleFont<F> {
    use wgpu::util::DeviceExt;

    // (한국어) 라벨 데이터를 생성합니다.
    // (English Translation) Create a label data.
    let label = format!("Text({})", name.unwrap_or("Unknown"));

    let len = text.trim().chars().count();
    let lines: Vec<_>= text.trim().split('\n').collect();
    let mut texture_bind_groups = HashMap::with_capacity(len);
    let mut characters: Vec<Vec<_>> = Vec::with_capacity(lines.len());

    let v_advance = font.height() + font.line_gap();
    let mut caret_x = 0.0;
    let mut caret_y = -v_advance;
    let mut max_width = caret_x;
    let mut max_height = -caret_y;
    for line in lines {
        let mut line_chars = Vec::with_capacity(line.trim().chars().count());
        for ch in line.trim().chars() {
            let glyph = font.scaled_glyph(ch);
            let h_advance = font.h_advance(glyph.id);
            line_chars.push((ch, font.outline_glyph(glyph).map(|outline| {
                // (한국어) 문자의 글리프 데이터를 가져옵니다.
                // (English Translation) Get glyph data for a character.
                let bound = outline.px_bounds();
                let width = bound.width();
                let height = bound.height();
                let bearing_x = bound.min.x;
                let bearing_y = bound.min.y;

                // (한국어) 문자의 이미지 데이터를 가져옵니다.
                // (English Translation) Get image data for a character.
                let w = width as usize;
                let h = height as usize;
                let mut data = vec![0u8; w * h];
                outline.draw(|x, y, v| data[(y as usize) * w + (x as usize)] = (v * 255.0) as u8);

                // (한국어) 
                // 텍스처 캐시에 해당 문자의 텍스처가 없는 경우, 
                // 해당 문자의 텍스처를 생성합니다.
                //
                // (English Translation) 
                // If there is no texture for that character in the texture cache,
                // creates a texture for that character.
                //
                texture_bind_groups.entry(ch).or_insert_with(|| {
                    // (한국어) 문자의 텍스처와 텍스처 뷰를 생성합니다.
                    // (English Translation) Create a texture and texture views for the character.
                    let texture = device.create_texture_with_data(
                        queue,
                        &wgpu::TextureDescriptor {
                            label: Some(&format!("Texture({})", label)),
                            size: wgpu::Extent3d {
                                width: w as u32,
                                height: h as u32,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: wgpu::TextureFormat::R8Unorm,
                            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                            view_formats: &[],
                        },
                        &data,
                    );
                    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

                    // (한국어) 문자 텍스처의 바인드 그룹을 생성합니다.
                    // (English Translation) Creates a bind group of character textures.
                    let bind_group = device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(&format!("BindGroup(Texture({}))", label)),
                            layout: texture_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::TextureView(&texture_view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::Sampler(tex_sampler),
                                },
                            ],
                        }
                    );

                    bind_group
                });

                let x_pos = caret_x + bearing_x;
                let y_pos = caret_y - height - bearing_y;

                InstanceData {
                    transform: Mat4::from_translation((x_pos, y_pos, 0.0).into()),
                    color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
                    size: (width, height).into(),
                }
            })));

            // (한국어) 캐럿의 위치를 갱신합니다.
            // (English Translation) Updates the caret position.
            caret_x += h_advance;
        }

        // (한국어) 위치를 조정합니다.
        // (English Translation) Adjusts the position.
        let offset_x = offset_x(origin.x, caret_x);
        characters.push(line_chars.into_iter().map(|(ch, glyph)| (
            ch, glyph.map(|mut glyph| { glyph.transform.w_axis.x += offset_x; glyph })
        )).collect());

        // (한국어) 최대 가로 값을 갱신합니다.
        // (English Translation) Updates the maximum width.
        max_width = max_width.max(caret_x);

        // (한국어) 캐럿의 위치를 갱신합니다.
        // (English Translation) Updates the caret position.
        caret_x = 0.0;
        caret_y -= v_advance;
    }

    // (한국어) 최대 세로 값을 갱신합니다.
    // (English Translation) Updates the maximum width.
    max_height = max_height.max(-caret_y);

    // (한국어) 위치를 조정합니다.
    // (English Translation) Adjusts the position.
    let offset_y = offset_y(origin.y, caret_y + 0.5 * font.height());
    let characters: Vec<_> = characters.into_iter().flatten().map(|(ch, glyph)| {
        if let Some(mut glyph) = glyph {
            // (한국어) 위치를 조정합니다.
            // (English Translation) Adjusts the position.
            glyph.transform.w_axis.y += offset_y;

            glyph.transform.w_axis.x /= glyph.size.y;
            glyph.transform.w_axis.y /= max_height;
            glyph.size.x /= glyph.size.y;
            glyph.size.y /= max_height;

            // (한국어) 문자의 글리프 데이터 버퍼를 생성합니다.
            // (English Translation) Creates a glyph data buffer for a character.
            let buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Vertex(GlyphData({}))", label)),
                    contents: bytemuck::bytes_of(&glyph),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                },
            );

            Character::Drawable(
                ch, 
                CharContext { 
                    buffer, 
                    data: glyph.into(), 
                })
        } else {
            Character::Control(ch)
        }
    }).collect();

    (characters, texture_bind_groups)
}
