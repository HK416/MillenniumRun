use std::mem::size_of;

use glam::{Mat4, Vec3, Vec4, Quat};
use bytemuck::{Pod, Zeroable};



/// #### 한국어 </br>
/// 텍스트 렌더링에 사용되는 버텍스 버퍼를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This builder creates a vertex buffer used for text rendering. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexInputBuilder {
    pub width: f32,
    pub height: f32,
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
    pub color: Vec4,
}

impl VertexInputBuilder {
    #[inline]
    pub fn build(self) -> VertexInput {
        VertexInput { 
            width: self.width, 
            height: self.height, 
            color: self.color, 
            transform: Mat4::from_scale_rotation_translation(
                self.scale, 
                self.rotation, 
                self.position
            ),
            ..Default::default()
        }
    }
}

impl Default for VertexInputBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            width: 0.0, 
            height: 0.0, 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            rotation: Quat::IDENTITY, 
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 텍스트 렌더링에 사용되는 버텍스 버퍼 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a vertex buffer structure used for text rendering. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct VertexInput {
    pub width: f32,
    pub height: f32,
    pub padding: [u8; size_of::<f32>() * 2],
    pub color: Vec4,
    pub transform: Mat4,
}

impl Default for VertexInput {
    #[inline]
    fn default() -> Self {
        Self { 
            width: 0.0, 
            height: 0.0, 
            padding: [0; size_of::<f32>() * 2],
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            transform: Mat4::IDENTITY, 
        }
    }
}
