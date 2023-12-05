use bytemuck::{Pod, Zeroable};



/// #### 한국어 </br>
/// 각 기준점으로 부터 상대 위치를 나타냅니다. </br>
/// 
/// #### English (Translation) </br>
/// Indicates the relative position from each pivot point. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Margin {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl Margin {
    #[inline]
    pub const fn new(top: i32, left: i32, bottom: i32, right: i32) -> Self {
        Self { top, left, bottom, right }
    }

    #[inline]
    pub fn set_top(&mut self, val: i32) {
        self.top = val;
    }

    #[inline]
    pub fn set_left(&mut self, val: i32) {
        self.left = val;
    }

    #[inline]
    pub fn set_bottom(&mut self, val: i32) {
        self.bottom = val;
    }

    #[inline]
    pub fn set_right(&mut self, val: i32) {
        self.right = val;
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.top
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.left
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.right
    }
}

impl Default for Margin {
    #[inline]
    fn default() -> Self {
        Self {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        }
    }
}
