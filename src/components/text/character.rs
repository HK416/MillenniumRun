use std::sync::{Mutex, MutexGuard};

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
/// 문자를 렌더링 할 때 필요한 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data needed when rendering characters. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct CharacterData {
    pub transform: Mat4,
    pub color: Vec4,
    pub size: Vec2,
}

impl Default for CharacterData {
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
/// 그리기 가능한 문자의 데이터 버퍼를 포함하고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains a data buffer of drawable characters.
/// 
#[derive(Debug)]
pub struct DrawableChar {
    pub data: Mutex<CharacterData>,
    pub(super) buffer: wgpu::Buffer,
}

impl DrawableChar {
    /// #### 한국어 </br>
    /// 문자 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the character data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update_character<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, CharacterData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&*guard));
    }

    /// #### 한국어 </br>
    /// 문자를 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws text on the screen. </br>
    /// 
    #[inline]
    pub fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
        rpass.draw(0..4, 0..1);
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
    Drawable(char, DrawableChar),
}
