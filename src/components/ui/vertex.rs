use std::mem::size_of;

use glam::{Vec4, Vec3};
use bytemuck::{Pod, Zeroable};

use crate::components::{
    anchor::Anchor, 
    margin::Margin
};



/// #### 한국어 </br>
/// 유저 인터페이스 렌더링에 사용되는 버텍스 버퍼 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a vertex buffer structure used for user interface rendering. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct VertexInput {
    pub anchor: Anchor,
    pub margin: Margin,
    pub color: Vec4,
    pub scale: Vec3,
    pub depth: f32,
}

impl Default for VertexInput {
    #[inline]
    fn default() -> Self {
        Self { 
            anchor: Anchor::default(), 
            margin: Margin::default(),
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            depth: 0.0,
        }
    }
}
