use std::collections::HashMap;

use wgpu::util::DeviceExt;
use glam::{Mat4, Vec4, Vec3, Vec2, Quat};
use ab_glyph::{Font, ScaleFont, FontArc};

use super::vertex::{VertexInput, VertexInputBuilder};



/// #### 한국어 </br>
/// 문자의 글리프 정보를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains glyph information of the character. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Glyph {
    pub size: Vec2,
    pub bearing: Vec2,
    pub advance: f32,
}



/// #### 한국어 </br>
/// 한 문자를 표현하는 정점 버퍼를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains a vertex buffer representing on character. </br>
/// 
#[derive(Debug)]
pub struct Character {
    ch: char,
    data: Option<VertexInput>,
    buffer: Option<wgpu::Buffer>,
}

impl Character {
    #[inline]
    pub fn char(&self) -> char {
        self.ch
    }

    #[inline]
    pub fn buffer(&self) -> Option<&wgpu::Buffer> {
        self.buffer.as_ref()
    }
}



/// #### 한국어 </br>
/// 문자의 반복자 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Iterator of characters. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct CharacterIter<'a> {
    index: usize,
    last_index: usize,
    text: &'a [Character],
}

impl<'a> Iterator for CharacterIter<'a> {
    type Item = &'a Character;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.index <= self.last_index{ self.text.get(self.index) } else { None };
        self.index += 1;
        return item;
    }
}



/// #### 한국어 </br>
/// 텍스트의 정렬 위치 정보를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains information on the alignment position of the text. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    TopLeft(Vec2),
    TopRight(Vec2),
    TopCenter(Vec2),
    BottomLeft(Vec2),
    BottomRight(Vec2),
    BottomCenter(Vec2),
    Center(Vec2),
}

impl Default for Align {
    #[inline]
    fn default() -> Self {
        Self::Center(Vec2 { x: 0.0, y: 0.0 })
    }
}



#[derive(Debug, Clone, Copy)]
pub struct SectionBuilder<'a> {
    font: &'a FontArc,
    font_size: f32,
    text: &'a str,
    align: Align,
    depth: f32,
    scale: Vec3,
    rotation: Quat,
    color: Vec4,
    first: Option<usize>,
    last: Option<usize>,
}

#[allow(dead_code)]
impl<'a> SectionBuilder<'a> {
    #[inline]
    pub fn new(font: &'a FontArc, font_size: f32, text: &'a str) -> Self {
        assert!(text.trim().chars().count() > 0, "The length of the given text must be greater than 0.");
        Self { 
            font,
            font_size,
            text, 
            align: Align::default(), 
            depth: 0.0,
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotation: Quat::IDENTITY,
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            first: None, 
            last: None,
        }
    }

    #[inline]
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        return self;
    }

    #[inline]
    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        return self;
    }

    #[inline]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        return self;
    }

    #[inline]
    pub fn with_first(mut self, first: usize) -> Self {
        assert!(first < self.text.trim().chars().count(), "Index out of range!");
        self.first = Some(first);
        return self;
    }

    #[inline]
    pub fn with_last(mut self, last: usize) -> Self {
        assert!(last < self.text.trim().chars().count(), "Index out of range!");
        self.last = Some(last);
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
    pub fn build(self, device: &wgpu::Device) -> Section {
        Section::new(
            device, 
            self.font, 
            self.font_size, 
            self.text, 
            self.align, 
            self.depth, 
            self.scale,
            self.rotation,
            self.color, 
            match self.first {
                Some(idx) => idx,
                None => 0,
            }, 
            match self.last {
                Some(idx) => idx,
                None => self.text.len() - 1
            },
        )
    }
}



/// #### 한국어 </br>
/// 텍스트의 구획정보를 담고있는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains section information of text. </br>
/// 
#[derive(Debug)]
pub struct Section {
    text: Vec<Character>,
    first: usize,
    last: usize,
    glyphs: HashMap<char, Glyph>,
}

impl Section {
    #[inline]
    fn new(
        device: &wgpu::Device,
        font: &FontArc,
        font_size: f32,
        text: &str,
        align: Align,
        depth: f32,
        scale: Vec3,
        rotation: Quat,
        color: Vec4,
        first: usize,
        last: usize,
    ) -> Self {
        let font = font.as_scaled(font_size);
        let (glyphs, text) = match align {
            Align::BottomLeft(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth,
                scale,
                rotation,
                color, 
                text, 
                |offset, _| { offset }, 
                |offset, caret| { offset - caret },
            ),
            Align::BottomRight(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth,
                scale,
                rotation,
                color,
                text, 
                |offset, caret| { offset - caret }, 
                |offset, caret| { offset - caret },
            ),
            Align::BottomCenter(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth, 
                scale, 
                rotation, 
                color, 
                text, 
                |origin, caret| { origin - 0.5 * caret }, 
                |offset, caret| { offset - caret }, 
            ),
            Align::TopLeft(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth,
                scale,
                rotation,
                color, 
                text, 
                |offset, _| { offset }, 
                |offset, _| { offset },
            ),
            Align::TopRight(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth,
                scale,
                rotation,
                color, 
                text, 
                |offset, caret| { offset - caret }, 
                |offset, _| { offset },
            ),
            Align::TopCenter(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth, 
                scale, 
                rotation, 
                color, 
                text, 
                |origin, caret| { origin - 0.5 * caret }, 
                |offset, _| { offset },
            ),
            Align::Center(origin) => Self::align(
                device, 
                font, 
                origin, 
                depth,
                scale,
                rotation,
                color, 
                text, 
                |origin, caret| { origin - 0.5 * caret }, 
                |origin, caret| { origin - 0.5 * caret },
            )
        };

        Self { 
            text, 
            first, 
            last, 
            glyphs
        }
    }

    /// #### 한국어 </br>
    /// 정점의 위치를 계산하고, 문자의 글리프와 정점을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Calculate vertex positions and generate glyphs and vertices for the character. </br>
    /// 
    fn generate_vertices<F: Font, SF: ScaleFont<F>>(
        font: &SF,
        text: &str,
        depth: f32,
        scale: Vec3,
        rotation: Quat,
        color: Vec4,
        caret: &mut Vec2,
        glyphs: &mut HashMap<char, Glyph>,
    ) -> Vec<(char, Option<VertexInputBuilder>)> {
        let mut vertices = Vec::with_capacity(text.len());
        for ch in text.trim().chars() {
            let glyph = font.scaled_glyph(ch);
            let h_advance = font.h_advance(glyph.id);
            vertices.push((ch, match font.outline_glyph(glyph) {
                Some(outline) => {
                    let bound = outline.px_bounds();
                    let width = bound.width();
                    let height = bound.height();
                    let bearing_x = bound.min.x;
                    let bearing_y = bound.min.y;

                    glyphs.entry(ch)
                        .or_insert(Glyph { 
                            size: Vec2 { x: width, y: height }, 
                            bearing: Vec2 { x: bearing_x, y: bearing_y }, 
                            advance: h_advance 
                        });

                    let x_pos = caret.x + bearing_x;
                    let y_pos = caret.y - height - bearing_y;
                    
                    Some(VertexInputBuilder {
                        width,
                        height,
                        scale,
                        rotation,
                        position: Vec3 { x: x_pos, y: y_pos, z: depth },
                        color,
                    })
                },
                None => None,
            }));
            caret.x += h_advance;
        }
        return vertices;
    }

    /// #### 한국어 </br>
    /// 주어진 텍스트 정렬 위치에 맞춰 문자를 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates characters based on the given text alignment position. </br>
    /// 
    fn align<F, SF, Fx, Fy>(
        device: &wgpu::Device,
        font: SF,
        origin: Vec2,
        depth: f32,
        scale: Vec3,
        rotation: Quat,
        color: Vec4,
        text: &str,
        offset_x: Fx,
        offset_y: Fy,
    ) -> (HashMap<char, Glyph>, Vec<Character>)
    where
        F: Font,
        SF: ScaleFont<F>,
        Fx: Fn(f32, f32) -> f32,
        Fy: Fn(f32, f32) -> f32,
    {
        let len = text.trim().len();
        let lines: Vec<&str> = text.trim().split('\n').collect();
        let mut glyphs = HashMap::with_capacity(len);
        let mut vertices: Vec<Vec<_>> = Vec::with_capacity(lines.len());

        let v_advance = font.height() + font.line_gap();
        let mut caret = Vec2 { x: 0.0, y: -v_advance };
        for line in lines {
            let line_vertices = Self::generate_vertices(
                &font, 
                line, 
                depth, 
                scale,
                rotation,
                color, 
                &mut caret,
                &mut glyphs
            );

            let offset_x = offset_x(origin.x, caret.x);
            vertices.push(
                line_vertices.into_iter()
                    .map(|(ch, mut builder)| {
                        if let Some(builder) = &mut builder {
                            builder.position.x += offset_x;
                        };
                        (ch, builder)
                    })
                    .collect()
            );

            caret.x = 0.0;
            caret.y -= v_advance;
        }

        let offset_y = offset_y(origin.y, caret.y + 0.5 * font.height());
        let character: Vec<_> = vertices.into_iter()
            .flatten()
            .map(|(ch, builder)| {
                let data = builder.map(|mut builder| {
                    builder.position.y += offset_y;
                    builder.build()
                });
                let buffer = data.map(|vertex| 
                    device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(format!("Character({}) - Vertex Input Buffer", ch).as_str()),
                            contents: bytemuck::bytes_of(&vertex),
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                        }
                    )
                );

                Character { ch, data, buffer }
            })
            .collect();

        return (glyphs, character);
    }

    /// #### 한국어 </br>
    /// 텍스트의 색상 값을 설정합니다. </br>
    /// 이 함수는 값이 바로 갱신되지 않습니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set the color value of the text. </br>
    /// This function does not update the value immediately. </br>
    /// 
    pub fn set_color_all(&mut self, color: Vec4) {
        for ch in self.text.iter_mut() {
            if let Some(data) = &mut ch.data {
                data.color = color;
            }
        }
    }

    /// #### 한국어 </br>
    /// 텍스트의 정점 입력 버퍼를 갱신시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the text's vertex input buffer. </br>
    /// 
    pub fn update(&self, queue: &wgpu::Queue) {
        for ch in self.text.iter() {
            if ch.data.is_some() && ch.buffer.is_some() {
                queue.write_buffer(ch.buffer().unwrap(), 0, bytemuck::bytes_of(&ch.data.unwrap()));
            }
        }
    }


    /// #### 한국어 </br>
    /// 보이는 범위의 문자의 반복자를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns an iterator of characters in the visible range. </br>
    /// 
    #[inline]
    pub fn chars<'a>(&'a self) -> CharacterIter<'a> {
        CharacterIter { 
            index: self.first,
            last_index: self.last, 
            text: &self.text, 
        }
    }

    /// #### 한국어 </br>
    /// 전체 범위의 문자의 반복자를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns an iterator of characters in the all range. </br>
    /// 
    #[inline]
    pub fn all_chars<'a>(&'a self) -> CharacterIter<'a> {
        CharacterIter {
            index: 0,
            last_index: self.text.len(),
            text: &self.text
        }
    }
}
