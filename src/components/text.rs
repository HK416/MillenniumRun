use std::mem::size_of;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use wgpu::util::DeviceExt;
use ab_glyph::{FontArc, Font, ScaleFont};
use bytemuck::{Pod, Zeroable, offset_of};
use glam::{Mat4, Quat, Vec4, Vec3, Vec2};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        anchor::Anchor, 
        margin::Margin, 
    }, 
    render::shader::WgslDecoder, 
    system::error::AppResult, 
};



/// #### 한국어 </br>
/// 문자를 렌더링 할 때 필요한 버텍스 입력 데이터 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the vertex input data structure needed when rendering characters. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
struct VertexInput {
    pub transform: Mat4, 
    pub color: Vec4, 
    pub size: Vec2, 
}

impl Default for VertexInput {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Mat4::IDENTITY, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 문자를 렌더링 할 때 필요한 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data needed when rendering characters. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharData {
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub color: Vec4, 
    pub size: Vec2, 
}

impl CharData {
    #[inline]
    fn to_data(&self) -> VertexInput {
        VertexInput { 
            transform: Mat4::from_scale_rotation_translation(
                self.scale, 
                self.rotation, 
                self.translation
            ), 
            color: self.color, 
            size: self.size, 
        }
    }
}

impl Default for CharData {
    #[inline]
    fn default() -> Self {
        Self { 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 문자의 렌더링 데이터를 담고있는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains character rendering data. </br>
/// 
#[derive(Debug)]
pub struct Char {
    ch: char, 
    buffer: wgpu::Buffer, 
    pub data: Mutex<CharData>, 
}

#[allow(dead_code)]
impl Char {
    #[inline]
    pub fn char(&self) -> char {
        self.ch
    }

    /// #### 한국어 </br>
    /// 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, CharData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&guard.to_data()));
    }

    #[inline]
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    #[inline]
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}



/// #### 한국어 </br>
/// 문자를 렌더링하는데 필요한 텍스트 구획의 유니폼 버퍼 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// A uniform buffer structure for text section data needed to render characters. </br>
/// 
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct TextUniform {
    pub transform: Mat4, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub color: Vec4, 
}

impl Default for TextUniform {
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
/// 문자를 렌더링하는데 필요한 텍스트 구획의 데이터를 담고있는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains text section data needed to render characters. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextData {
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub color: Vec4, 
}

impl TextData {
    #[inline]
    fn to_data(&self) -> TextUniform {
        TextUniform { 
            transform: Mat4::from_scale_rotation_translation(
                self.scale, 
                self.rotation, 
                self.translation
            ), 
            anchor: self.anchor, 
            margin: self.margin, 
            color: self.color 
        }
    }
}

impl Default for TextData {
    #[inline]
    fn default() -> Self {
        Self {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
        }
    }
}



/// #### 한국어 </br>
/// 텍스트의 렌더링 데이터를 담고있는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains text rendering data. </br>
/// 
#[derive(Debug)]
pub struct Text {
    name: String, 
    font: FontArc, 
    buffer: wgpu::Buffer, 
    buffer_bind_group: wgpu::BindGroup, 
    texture_bind_groups: HashMap<char, wgpu::BindGroup>, 
    characters: Vec<Option<Char>>, 
    pub data: Mutex<TextData>, 
}

#[allow(dead_code)]
impl Text {
    fn new<'a>(
        builder: TextBuilder<'a>, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        let name = builder.name.unwrap_or_else(|| "Unknown");

        // (한국어) 유니폼 버퍼를 생성합니다.
        // (English Translation) Creates a uniform buffer.
        let data = TextData {
            scale: builder.scale, 
            rotation: builder.rotation, 
            translation: builder.translation, 
            anchor: builder.anchor, 
            margin: builder.margin, 
            color: builder.color,
        };
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Uniform(Text({}))", name)), 
                contents: bytemuck::bytes_of(&data.to_data()), 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
        );

        // (한국어) 유니폼 버퍼 바인드 그룹을 생성합니다.
        // (English Translation) Create a uniform buffer bind group.
        let buffer_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BindGroup(Uniform(Text({})))", name)),
                layout: &builder.brush.buffer_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::Buffer(
                            buffer.as_entire_buffer_binding()
                        ),
                    },
                ],
            },
        );

        let mut texture_bind_groups = HashMap::new();
        let characters = create_characters(
            name, 
            builder.font, 
            builder.text, 
            device, 
            queue, 
            &builder.brush.tex_sampler, 
            &builder.brush.texture_layout, 
            &mut texture_bind_groups
        );

        Self { 
            name: name.to_string(), 
            font: builder.font.clone(), 
            buffer, 
            buffer_bind_group, 
            texture_bind_groups, 
            characters, 
            data: data.into() 
        }
    }

    /// #### 한국어 </br>
    /// 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, TextData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&guard.to_data()));
    }

    /// #### 한국어 </br>
    /// 텍스트의 내용을 변경합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Change the content of the text. </br>
    /// 
    #[inline]
    pub fn change(
        &mut self, 
        text: &str, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        text_brush: &TextBrush
    ) {
        self.characters = create_characters(
            &self.name, 
            &self.font, 
            text, 
            device, 
            queue, 
            &text_brush.tex_sampler, 
            &text_brush.texture_layout, 
            &mut self.texture_bind_groups
        );
    }

    #[inline]
    fn bind_buffer<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.buffer_bind_group, &[])
    }

    #[inline]
    fn bind_texture<'pass>(&'pass self, ch: char, rpass: &mut wgpu::RenderPass<'pass>) {
        if let Some(bind_group) = self.texture_bind_groups.get(&ch) {
            rpass.set_bind_group(2, bind_group, &[])
        }
    }
}

/// #### 한국어 </br>
/// 텍스트를 생성하는 빌더 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder structure that creates text. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct TextBuilder<'a> {
    pub name: Option<&'a str>, 
    pub font: &'a FontArc, 
    pub text: &'a str, 
    pub color: Vec4, 
    pub scale: Vec3, 
    pub rotation: Quat, 
    pub translation: Vec3, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub brush: &'a TextBrush, 
}

#[allow(dead_code)]
impl<'a> TextBuilder<'a> {
    #[inline]
    pub fn new(
        name: Option<&'a str>, 
        font: &'a FontArc, 
        text: &'a str, 
        brush: &'a TextBrush
    ) -> Self {
        Self { 
            name, 
            font, 
            text, 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            brush 
        }
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
        self.rotation = rotation.normalize();
        return self;
    }

    #[inline]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
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
    pub fn build(self, device: &wgpu::Device, queue: &wgpu::Queue) -> Text {
        Text::new(self, device, queue)
    }
}

/// #### 한국어 </br>
/// 텍스트의 문자들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates characters of text. </br>
/// 
fn create_characters(
    name: &str, 
    font: &FontArc, 
    text: &str, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_layout: &wgpu::BindGroupLayout, 
    texture_bind_groups: &mut HashMap<char, wgpu::BindGroup>
) -> Vec<Option<Char>> {
    let font = font.as_scaled(128.0);
    let lines: Vec<_> = text.trim().split('\n').collect();
    let mut str: Vec<Vec<_>> = Vec::with_capacity(lines.len());

    let v_advance = font.height() + font.line_gap();
    let mut caret_x = 0.0;
    let mut caret_y = -v_advance;
    let mut maximum_width = caret_x;
    let mut maximum_height = -caret_y;
    for line in lines {
        let mut chars = Vec::with_capacity(line.trim().chars().count());
        for ch in line.trim().chars() {
            let glyph = font.scaled_glyph(ch);
            let h_advance = font.h_advance(glyph.id);
            chars.push(font.outline_glyph(glyph).map(|outline| {
                // (한국어) 문자의 글리프 데이터를 가져옵니다.
                // (English Translation) Get glyph data for a character.
                let bound = outline.px_bounds();
                let width = bound.width();
                let height = bound.height();
                let bearing_x = bound.min.x;
                let bearing_y = bound.min.y;

                // (한국어) 문자의 텍스처 데이터를 가져옵니다.
                // (English Translation) Get texture data for a character.
                let w = width as usize;
                let h = height as usize;
                let mut data = vec![0u8; w * h];
                outline.draw(|x, y, v| {
                    data[(y as usize) * w + (x as usize)] = (v * 255.0) as u8;
                });

                // (한국어) 
                // 텍스처 캐시에 해당 문자의 텍스처가 없는 경우, 해당 문자의 텍스처를 생성합니다.
                //
                // (English Translation)
                // If there is no texture for that character in the texture cache,
                // creates a texture for that character.
                //
                texture_bind_groups.entry(ch).or_insert_with(|| {
                    // (한국어) 문자의 텍스처와 텍스처 뷰를 생성합니다.
                    // (English Translation) Create a texture and texture view for teh character.
                    let texture = device.create_texture_with_data(
                        queue, 
                        &wgpu::TextureDescriptor {
                            label: Some(&format!("Texture(Text({}))", name)), 
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
                            view_formats: &[]
                        }, 
                        wgpu::util::TextureDataOrder::LayerMajor,
                        &data
                    );
                    let texture_view = texture.create_view(
                        &wgpu::TextureViewDescriptor {
                            ..Default::default()
                        }
                    );

                    // (한국어) 문자 텍스처의 바인드 그룹을 생성합니다.
                    // (English Translation) Creates a bind group of character texture.
                    device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(&format!("BindGroup(Texture(Text({})))", name)), 
                            layout: texture_layout, 
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::TextureView(
                                        &texture_view
                                    ),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::Sampler(
                                        tex_sampler
                                    ),
                                },
                            ],
                        },
                    )
                });

                let x = caret_x + bearing_x;
                let y = caret_y - height - bearing_y;

                (ch, CharData {
                    translation: (x, y, 0.0).into(), 
                    size: (width, height).into(),
                    ..Default::default()
                })
            }));

            // (한국어) 캐럿의 위치를 갱신합니다.
            // (English Translation) Updates the caret position.
            caret_x += h_advance;
        }

        // (한국어) 위치를 조정합니다.
        // (English Translation) Adjusts the position.
        let offset_x = -0.5 * caret_x;
        for char in chars.iter_mut() {
            if let Some((_, data)) = char {
                data.translation.x += offset_x;
            }
        }
        str.push(chars);

        // (한국어) 최대 가로 길이를 갱신합니다.
        // (English Translation) Updates the maximum width.
        maximum_width = maximum_width.max(caret_x);

        // (한국어) 캐럿의 위치를 갱신합니다.
        // (English Translation) Updates the caret position.
        caret_x = 0.0;
        caret_y -= v_advance;
    }

    // (한국어) 최대 세로 값을 갱신합니다.
    // (English Translation) Updates the maximum width.
    maximum_height = maximum_height.max(-caret_y);

    // (한국어) 문자들의 크기와 위치를 조정합니다.
    // (English Translation) Adjust the size and position of characters. 
    // 
    // FIXME: WTF 이거 어떻게 작동되는거야...
    //
    let offset_y = -0.5 * (caret_y + 0.5 * font.height());
    for char in str.iter_mut().flatten() {
        if let Some((_, data)) = char {
            data.translation.y += offset_y;

            data.translation.x /= data.size.y;
            data.translation.y /= maximum_height;
            data.size.x /= data.size.y;
            data.size.y /= maximum_height;
        }
    }

    str.into_iter().flatten()
        .map(|char| {
            char.map(|(ch, data)| {
                // (한국어) 문자의 버텍스 입력 버퍼를 생성합니다.
                // (English Translation) Creates a vertex input buffer for characters. 
                let buffer = device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Vertex(Text({}))", name)), 
                        contents: bytemuck::bytes_of(&data.to_data()), 
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
                    },
                );

                Char {
                    ch, 
                    buffer, 
                    data: data.into(),
                }
            })
        })
        .collect()
}



/// #### 한국어 </br>
/// 텍스트를 화면에 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// A tool for drawing text on the screen. </br>
/// 
#[derive(Debug)]
pub struct TextBrush {
    pipeline: wgpu::RenderPipeline, 
    pub tex_sampler: wgpu::Sampler, 
    pub buffer_layout: wgpu::BindGroupLayout, 
    pub texture_layout: wgpu::BindGroupLayout, 
}

impl TextBrush {
    pub fn new(
        device: &wgpu::Device, 
        camera_layout: &wgpu::BindGroupLayout, 
        render_format: wgpu::TextureFormat, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState, 
        multiview: Option<std::num::NonZeroU32>, 
        asset_bundle: &AssetBundle
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        let tex_sampler = create_texture_sampler(device);
        let buffer_layout = create_buffer_layout(device);
        let texture_layout = create_texture_layout(device);
        let bind_group_layouts = &[camera_layout, &buffer_layout, &texture_layout];
        let pipeline = create_render_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format, 
            depth_stencil, 
            multisample, 
            multiview
        );

        Ok(Self {
            pipeline, 
            tex_sampler, 
            buffer_layout, 
            texture_layout, 
        }.into())
    }
    
    /// #### 한국어 </br>
    /// 주어진 텍스트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given text on the screen. </br>
    /// 
    pub fn draw<'pass, I>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>, iter: I)
    where I: Iterator<Item = &'pass Text> {
        rpass.set_pipeline(&self.pipeline);
        for text in iter {
            text.bind_buffer(rpass);
            for ch in text.characters.iter() {
                if let Some(ch) = ch {
                    text.bind_texture(ch.char(), rpass);
                    ch.bind(rpass);
                    ch.draw(rpass);
                }
            }
        }
    }
}

/// #### 한국어 </br>
/// 텍스트 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a text shader module. </br>
/// 
fn create_shader_module(device: &wgpu::Device, asset_bundle: &AssetBundle) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::UI_TEXT_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Text"), device })?;
    asset_bundle.release(path::UI_TEXT_SHADER_PATH);
    Ok(module)
}

/// #### 한국어 </br>
/// 텍스트의 텍스처 샘플러를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a texture sampler for the text. </br>
/// 
#[inline]
fn create_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(
        &wgpu::SamplerDescriptor {
            label: Some("Sampler(Text)"), 
            address_mode_u: wgpu::AddressMode::ClampToEdge, 
            address_mode_v: wgpu::AddressMode::ClampToEdge, 
            address_mode_w: wgpu::AddressMode::ClampToEdge, 
            min_filter: wgpu::FilterMode::Linear, 
            mag_filter: wgpu::FilterMode::Linear, 
            mipmap_filter: wgpu::FilterMode::Linear, 
            ..Default::default()
        }
    )
}

/// #### 한국어 </br>
/// 텍스트의 버퍼 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a buffer bind group layout for text. </br>
/// 
#[inline]
fn create_buffer_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Uniform(Text))"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::VERTEX, 
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None, 
                },
            ],
        },
    )
}

/// #### 한국어 </br>
/// 텍스트의 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a texture bind group layout for text. </br>
/// 
#[inline]
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(Text))"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, 
                    visibility: wgpu::ShaderStages::FRAGMENT, 
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { 
                            filterable: true 
                        }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false 
                    },
                    count: None, 
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering
                    ),
                    count: None
                },
            ],
        },
    )
}

/// #### 한국어 </br>
/// 텍스트의 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a rendering pipeline for text. </br>
/// 
fn create_render_pipeline(
    device: &wgpu::Device, 
    module: &wgpu::ShaderModule, 
    bind_group_layouts: &[&wgpu::BindGroupLayout], 
    render_format: wgpu::TextureFormat, 
    depth_stencil: Option<wgpu::DepthStencilState>, 
    multisample: wgpu::MultisampleState, 
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Text)"), 
            bind_group_layouts, 
            push_constant_ranges: &[], 
        },
    );

    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Text)"), 
            layout: Some(&pipeline_layout), 
            vertex: wgpu::VertexState {
                module, 
                entry_point: "vs_main", 
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<VertexInput>() as wgpu::BufferAddress, 
                        step_mode: wgpu::VertexStepMode::Instance, 
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 1, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 2, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 3, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress, 
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: offset_of!(VertexInput, color) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2, 
                                offset: offset_of!(VertexInput, size) as wgpu::BufferAddress, 
                            }
                        ],
                    },
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, 
                strip_index_format: Some(wgpu::IndexFormat::Uint16), 
                polygon_mode: wgpu::PolygonMode::Fill, 
                ..Default::default()
            }, 
            depth_stencil, 
            multisample, 
            fragment: Some(wgpu::FragmentState {
                module, 
                entry_point: "fs_main", 
                targets: &[
                    Some(wgpu::ColorTargetState {
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                        format: render_format, 
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            multiview,
        },
    )
}
