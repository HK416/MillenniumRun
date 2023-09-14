use glam::Mat4;

use crate::logic::object::GameObject;



/// #### 한국어
/// 뷰포트 사각형 자료형입니다.
/// `wgpu`의 뷰포트 좌표계를 사용합니다.
/// 자세한 내용은 [`wgpu::RenderPass::set_viewport`](`wgpu::RenderPass`)를 참고하세요.
/// 
/// #### English (Translation)
/// The viewport rectangle data type.
/// Use the viewport coordinate system of `wgpu`.
/// See [`wgpu::RenderPass::set_viewport`](`wgpu::RenderPass`) for details.
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_z: f32,
    pub max_z: f32,
}


/// #### 한국어
/// 장면에 존재하는 카메라 물체에 대한 인터페이스 입니다.
/// 
/// #### English (Translation)
/// An interface to camera objects that exist in the scene.
/// 
pub trait GameCamera : GameObject {
    /// #### 한국어
    /// `GameCamera`의 뷰포트를 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the viewport for `GameCamera`.
    /// 
    fn viewport(&self) -> Option<Viewport>;

    /// #### 한국어
    /// `GameCamera`의 카메라 변환 행렬을 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the camera transformation matrix for `GameCamera`.
    /// 
    fn camera_matrix(&self) -> Mat4;
    
    /// #### 한국어
    /// `GameCamera`의 투영 변환 행렬을 반환합니다.
    /// 
    /// #### English (Translation)
    /// Returns the projection transformation matrix for `GameCamera`.
    ///
    fn projection_matrix(&self) -> Mat4;
}
