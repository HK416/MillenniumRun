#![allow(dead_code)]

pub mod f32 {
    /// #### 한국어 </br>
    /// - 주어진 값들은 `0`보다 크거나 같아야 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// - The given values must be greater than or equal to `0`.
    /// 
    #[inline]
    pub fn linear(val: f32, max: f32) -> f32 {
        debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
        return (val / max).clamp(0.0, 1.0);
    }

    /// #### 한국어 </br>
    /// - 주어진 값들은 `0`보다 크거나 같아야 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// - The given values must be greater than or equal to `0`.
    /// 
    #[inline]
    pub fn smooth_step(val: f32, max: f32) -> f32 {
        debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
        let t = (val / max).clamp(0.0, 1.0); 
        return 3.0 * t * t - 2.0 * t * t * t;
    }
}

pub mod f64 {
    /// #### 한국어 </br>
    /// - 주어진 값들은 `0`보다 크거나 같아야 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// - The given values must be greater than or equal to `0`.
    /// 
    #[inline]
    pub fn linear(val: f64, max: f64) -> f64 {
        debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
        return (val / max).clamp(0.0, 1.0);
    }

    /// #### 한국어 </br>
    /// - 주어진 값들은 `0`보다 크거나 같아야 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// - The given values must be greater than or equal to `0`.
    /// 
    #[inline]
    pub fn smooth_step(val: f64, max: f64) -> f64 {
        debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
        let t = (val / max).clamp(0.0, 1.0); 
        return 3.0 * t * t - 2.0 * t * t * t;
    }
}
