use serde::{Deserialize, Serialize};

use crate::logic::object::GameObject;


/// #### 한국어 </br>
/// `ui`의 기준점의 위치를 저장합니다. </br>
/// 윈도우의 상단 왼쪽은 (0.0, 0.0), 하단 오른쪽은 (1.0, 1.0)인 좌표계를 사용합니다. </br>
/// 
/// #### English (Translation) </br>
/// Stores the position of the pivot point of the `ui`. </br>
/// The coordinate system is (0.0, 0.0) for the top left of the window
/// and (1.0, 1.0) for the bottom right of the window. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct UIAnchor {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

impl UIAnchor {
    /// #### 한국어 </br>
    /// 새로운 `ui` 기준점의 위치를 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a new `ui` pivot point location. </br>
    /// 
    /// <br>
    /// 
    /// # Panics </br>
    /// #### 한국어 </br>
    /// - 주어진 `top`이 `bottom`보다 클 경우. </br>
    /// - 주어진 `left`가 `right`보다 클 경우. </br>
    /// 
    /// #### English (Translation) </br>
    /// - If the given `top` is greater than or equal `bottom`. </br>
    /// - If the given `left` is greater than or equal `right`. </br>
    /// 
    #[inline]
    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Self {
        assert!(top <= bottom, "A given 'top' must less than or equal the given 'bottom'.");
        assert!(left <= right, "A given 'left' must less than or equal the given 'right'.");
        Self { top, left, bottom, right }
    }

    /// #### 한국어 </br>
    /// `top`을 설정합니다. `bottom`보다 클 경우 `bottom`의 값으로 보정됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set `top`. If it is greater than `bottom`, it is corrected to the value of `bottom`. </br>
    /// 
    #[inline]
    pub fn set_top(&mut self, top: f32) {
        self.top = self.bottom.min(top)
    }

    /// #### 한국어 </br>
    /// `left`을 설정합니다. `right`보다 클 경우 `right`의 값으로 보정됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set `left`. If it is greater than `right`, it is corrected to the value of `right`. </br>
    /// 
    #[inline]
    pub fn set_left(&mut self, left: f32) {
        self.left = self.right.min(left)
    }

    /// #### 한국어 </br>
    /// `bottom`을 설정합니다. `top`보다 작은 경우 `top`의 값으로 보정됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set `bottom`. If it is less than `top`, it is corrected to the value of `top`. </br>
    /// 
    #[inline]
    pub fn set_bottom(&mut self, bottom: f32) {
        self.bottom = self.top.max(bottom)
    }

    /// #### 한국어 </br>
    /// `right`을 설정합니다. `left`보다 작을 경우 `left`의 값으로 보정됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Set `right`. If it is less than `left`, it is corrected to the value of `left`. </br>
    /// 
    #[inline]
    pub fn set_right(&mut self, right: f32) {
        self.right = self.left.max(right)
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.top.clone()
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.left.clone()
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.bottom.clone()
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.right.clone()
    }
}

impl Default for UIAnchor {
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




/// #### 한국어
/// `UIAnchor`로 부터 상대적인 픽셀 위치를 저장합니다.
/// 하단 오른쪽으로 갈수록 값이 증가하는 좌표계를 사용합니다.
/// 
/// #### English (Translation)
/// Store the relative pixel position from `UIAnchor`.
/// Uses a coordinate system with values increasing toward the bottom right.
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct UIMargin {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl UIMargin {
    /// #### 한국어
    /// 새로운 `ui` 상대 픽셀 위치를 생성합니다.
    /// 
    /// #### English (Translation)
    /// Creates a new `ui` relative pixel position.
    /// 
    #[inline]
    pub fn new(top: i32, left: i32, bottom: i32, right: i32) -> Self {
        Self { top, left, bottom, right }
    }

    #[inline]
    pub fn set_top(&mut self, top: i32) {
        self.top = top
    }

    #[inline]
    pub fn set_left(&mut self, left: i32) {
        self.left = left
    }

    #[inline]
    pub fn set_bottom(&mut self, bottom: i32) {
        self.bottom = bottom
    }

    #[inline]
    pub fn set_right(&mut self, right: i32) {
        self.right = right
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.top.clone()
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.left.clone()
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.bottom.clone()
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.right.clone()
    }
}

impl Default for UIMargin {
    #[inline]
    fn default() -> Self {
        Self { 
            top: 0, 
            left: 0, 
            bottom: 0, 
            right: 0 
        }
    }
}



/// #### 한국어
/// `ui`를 구성하는 모든 객체에 대한 기본 인터페이스 입니다.
/// 
/// #### English (Translation)
/// This is the base interface for all objects that make up `ui`.
/// 
pub trait UIObject : GameObject {
    /// #### 한국어
    /// `UIObject`의 `UIAnchor`를 대여합니다. (reference ver)
    /// 
    /// #### English (Translation)
    /// Borrows the `UIAnchor` of `UIObject`. (reference ver)
    /// 
    fn ref_anchor(&self) -> &UIAnchor;

    /// #### 한국어
    /// `UIObject`의 `UIAnchor`를 대여합니다. (mutable ver)
    /// 
    /// #### English (Translation)
    /// Borrows the `UIAnchor` of `UIObject`. (mutable ver)
    /// 
    fn mut_anchor(&mut self) -> &mut UIAnchor;

    /// #### 한국어
    /// `UIObject`의 `UIMargin`를 대여합니다. (reference ver)
    /// 
    /// #### English (Translation)
    /// Borrows the `UIMargin` of `UIObject`. (reference ver)
    /// 
    fn ref_margin(&self) -> &UIMargin;

    /// #### 한국어
    /// `UIObject`의 `UIMargin`를 대여합니다. (mutable ver)
    /// 
    /// #### English (Translation)
    /// Borrows the `UIMargin` of `UIObject`. (mutable ver)
    /// 
    fn mut_margin(&mut self) -> &mut UIMargin;
}
