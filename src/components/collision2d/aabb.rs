use std::fmt;



/// #### 한국어 </br>
/// 2차원 축에 정렬된 충돌 박스 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a collision box structure aligned on a two-dimensional axis. </br>
/// 
#[derive(Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl BoundingBox {
    #[inline]
    pub fn new(p0: (f32, f32), p1: (f32, f32)) -> Self {
        Self { 
            min: (
                if p0.0 <= p1.0 { p0.0 } else { p1.0 },
                if p0.1 <= p1.1 { p0.1 } else { p1.1 },
            ), 
            max: (
                if p0.0 <= p1.0 { p1.0 } else { p0.0 },
                if p0.1 <= p1.1 { p1.1 } else { p0.1 },
            )
        }
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        other.min.0 <= self.max.0 && self.min.0 <= other.max.0
        && other.min.1 <= self.max.1 && self.min.1 <= other.max.1
    }

    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        self.min.0 <= other.min.0 && other.max.0 <= self.max.0
        && self.min.1 <= other.min.1 && other.max.1 <= self.max.1
    }
}



impl Default for BoundingBox {
    #[inline]
    fn default() -> Self {
        Self { 
            min: (0.0, 0.0),
            max: (0.0, 0.0),
        }
    }
}

impl fmt::Debug for BoundingBox {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}
