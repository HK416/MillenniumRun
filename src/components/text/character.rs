use glam::{Mat4, Vec4, Vec2};
use bytemuck::{Pod, Zeroable};



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



/// #### 한국어 </br>
/// 문자를 렌더링 할 때 필요한 글리프 정보를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains glyph information needed when rendering characters. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct GlyphData {
    pub transform: Mat4,
    pub color: Vec4,
    pub size: Vec2,
}

impl Default for GlyphData {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Mat4::IDENTITY, 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 글리프 데이터 버퍼를 가지고 있는 텍스트의 한 문자입니다. </br>
/// 
/// #### English (Translation) </br>
/// A character of text that holds a glyph data buffer. </br>
/// 
#[derive(Debug)]
pub enum Character {
    Control(char),
    Char(char, GlyphData, wgpu::Buffer),
}
