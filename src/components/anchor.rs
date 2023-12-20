use bytemuck::{Pod, Zeroable};



//--------------------------------*
// Coordinates System             |
//                                |
//   (0.0, 0.0)     (1.0, 0.0)   |
//          *----------*          |
//          |          |          |
//          |          |          |
//          *----------*          |
//   (0.0, 1.0)    (1.0, 1.0)  | 
//--------------------------------*
/// #### 한국어 </br>
/// 기준점을 표현하는 자료형입니다. </br>
/// 각 요소는 0.0에서 1.0사이의 값을 가집니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a data type that represents a pivot point. </br>
/// Each element has a value between 0.0 and 1.0. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Anchor {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

#[allow(dead_code)]
impl Anchor {
    /// # Panic </br>
    /// #### 한국어 <br>
    /// 다음과 같은 경우 애플리케이션 실행을 중단시킵니다. </br>
    /// <b>- 주어진 값의 범위가 `0.0 ~ 1.0`이 아닌 경우.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// The application will stop running in the following cases: </br>
    /// <b>- If the given value range is not `0.0 ~ 1.0`.</b></br>
    /// 
    #[inline]
    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Self {
        assert!(is_contains(top), "The given \'top\' must be a value between 0.0 and 1.0.");
        assert!(is_contains(left), "The given \'left\' must be a value between 0.0 and 1.0.");
        assert!(is_contains(bottom), "The given \'bottom\' must be a value between 0.0 and 1.0.");
        assert!(is_contains(right), "The given \'right\' must be a value between 0.0 and 1.0.");
        Self { top, left, bottom, right }
    }

    /// # Panic </br>
    /// #### 한국어 <br>
    /// 다음과 같은 경우 애플리케이션 실행을 중단시킵니다. </br>
    /// <b>- 주어진 값의 범위가 `0.0 ~ 1.0`이 아닌 경우.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// The application will stop running in the following cases: </br>
    /// <b>- If the given value range is not `0.0 ~ 1.0`.</b></br>
    /// 
    #[inline]
    pub fn set_top(&mut self, val: f32) {
        assert!(is_contains(val), "The given \'top\' must be a value between 0.0 and 1.0.");
        self.top = val;
    }

    /// # Panic </br>
    /// #### 한국어 <br>
    /// 다음과 같은 경우 애플리케이션 실행을 중단시킵니다. </br>
    /// <b>- 주어진 값의 범위가 `0.0 ~ 1.0`이 아닌 경우.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// The application will stop running in the following cases: </br>
    /// <b>- If the given value range is not `0.0 ~ 1.0`.</b></br>
    /// 
    #[inline]
    pub fn set_left(&mut self, val: f32) {
        assert!(is_contains(val), "The given \'left\' must be a value between 0.0 and 1.0.");
        self.left = val;
    }

    /// # Panic </br>
    /// #### 한국어 <br>
    /// 다음과 같은 경우 애플리케이션 실행을 중단시킵니다. </br>
    /// <b>- 주어진 값의 범위가 `0.0 ~ 1.0`이 아닌 경우.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// The application will stop running in the following cases: </br>
    /// <b>- If the given value range is not `0.0 ~ 1.0`.</b></br>
    /// 
    #[inline]
    pub fn set_bottom(&mut self, val: f32) {
        assert!(is_contains(val), "The given \'bottom\' must be a value between 0.0 and 1.0.");
        self.bottom = val;
    }

    /// # Panic </br>
    /// #### 한국어 <br>
    /// 다음과 같은 경우 애플리케이션 실행을 중단시킵니다. </br>
    /// <b>- 주어진 값의 범위가 `0.0 ~ 1.0`이 아닌 경우.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// The application will stop running in the following cases: </br>
    /// <b>- If the given value range is not `0.0 ~ 1.0`.</b></br>
    /// 
    #[inline]
    pub fn set_right(&mut self, val: f32) {
        assert!(is_contains(val), "The given \'right\' must be a value between 0.0 and 1.0.");
        self.right = val;
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.top
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.left
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.right
    }
}

impl Default for Anchor {
    #[inline]
    fn default() -> Self {
        Self { 
            top: 0.5,
            left: 0.5,
            bottom: 0.5,
            right: 0.5,
        }
    }
}



/// #### 한국어 </br>
/// 주어진 값이 `0.0 ~ 1.0`사이의 값인지 확인합니다. </br>
/// 
/// #### English (Translation) </br>
/// Checks whether the given value is `0.0 ~ 1.0`.
/// 
#[inline]
fn is_contains(value: f32) -> bool {
    (0.0..=1.0).contains(&value)
}
