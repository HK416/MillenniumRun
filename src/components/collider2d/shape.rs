use super::Collider2d;



/// #### 한국어 </br>
/// 원 모양의 충돌체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// It is circle-shaped collider. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Default for Circle {
    #[inline]
    fn default() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0, 
            radius: 0.0 
        }
    }
}

impl Collider2d<(f32, f32)> for Circle {
    fn test(&self, other: &(f32, f32)) -> bool {
        let x_sq = (self.x - other.0) * (self.x - other.0);
        let y_sq = (self.y - other.1) * (self.y - other.1);
        let dist = f32::sqrt(x_sq + y_sq);
        return self.radius >= dist;
    }
}

impl Collider2d<Circle> for Circle {
    fn test(&self, other: &Circle) -> bool {
        let x_sq = (self.x - other.x) * (self.x - other.x);
        let y_sq = (self.y - other.y) * (self.y - other.y);
        let dist = f32::sqrt(x_sq + y_sq);
        return self.radius + other.radius >= dist;
    }
}

impl Collider2d<AABB> for Circle {
    fn test(&self, other: &AABB) -> bool {
        let top = other.y + 0.5 * other.height;
        let left = other.x - 0.5 * other.width;
        let bottom = other.y - 0.5 * other.height;
        let right = other.x + 0.5 * other.width;

        let top_ex = top + self.radius;
        let left_ex = left - self.radius;
        let bottom_ex = bottom - self.radius;
        let right_ex = right + self.radius;

        if left <= self.x && self.x <= right
        && bottom_ex <= self.y && self.y <= top_ex {
            return true;
        }

        if left_ex <= self.x && self.x <= right_ex
        && bottom <= self.y && self.y <= top {
            return true;
        }

        let x_sq = (self.x - left) * (self.x - left);
        let y_sq = (self.y - top) * (self.y - top);
        let dist = f32::sqrt(x_sq + y_sq);
        if self.radius >= dist {
            return true;
        }

        let x_sq = (self.x - right) * (self.x - right);
        let y_sq = (self.y - top) * (self.y - top);
        let dist = f32::sqrt(x_sq + y_sq);
        if self.radius >= dist {
            return true;
        }

        let x_sq = (self.x - left) * (self.x - left);
        let y_sq = (self.y - bottom) * (self.y - bottom);
        let dist = f32::sqrt(x_sq + y_sq);
        if self.radius >= dist {
            return true;
        }

        let x_sq = (self.x - right) * (self.x - right);
        let y_sq = (self.y - bottom) * (self.y - bottom);
        let dist = f32::sqrt(x_sq + y_sq);
        if self.radius >= dist {
            return true;
        }

        return false;
    }
}

impl Collider2d<OBB> for Circle {
    #[inline]
    fn test(&self, other: &OBB) -> bool {
        other.test(self)
    }
}



/// #### 한국어 </br>
/// 축에 정렬된 직사각형 모양의 충돌체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// It is a rectangular collider aligned on an axis. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for AABB {
    #[inline]
    fn default() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0, 
            width: 0.0, 
            height: 0.0 
        }
    }
}

impl Collider2d<(f32, f32)> for AABB {
    fn test(&self, other: &(f32, f32)) -> bool {
        let top = self.y + 0.5 * self.height;
        let left = self.x - 0.5 * self.width;
        let bottom = self.y - 0.5 * self.height;
        let right = self.x + 0.5 * self.width;
        return left <= other.0 && other.0 <= right
        && bottom <= other.1 && other.1 <= top;
    }
}

impl Collider2d<Circle> for AABB {
    #[inline]
    fn test(&self, other: &Circle) -> bool {
        other.test(self)
    }
}

impl Collider2d<AABB> for AABB {
    fn test(&self, other: &AABB) -> bool {
        let top_l = self.y + 0.5 * self.height;
        let left_l = self.x - 0.5 * self.width;
        let bottom_l = self.y - 0.5 * self.height;
        let right_l = self.x + 0.5 * self.width;

        let top_r = other.y + 0.5 * other.height;
        let left_r = other.x - 0.5 * other.width;
        let bottom_r = other.y - 0.5 * other.height;
        let right_r = other.x + 0.5 * other.width;

        return left_l <= right_r && right_l >= left_r
        && bottom_l <= top_r && top_l >= bottom_r;
    }
}

impl Collider2d<OBB> for AABB {
    #[inline]
    fn test(&self, other: &OBB) -> bool {
        other.test(self)
    }
}


/// #### 한국어 </br>
/// 방향성이 있는 직사각형 모양의 충돌체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// It is a directional rectangular collider. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OBB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub radian: f32,
}

impl Default for OBB {
    #[inline]
    fn default() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0, 
            width: 0.0, 
            height: 0.0, 
            radian: 0.0 
        }
    }
}

impl Collider2d<(f32, f32)> for OBB {
    fn test(&self, other: &(f32, f32)) -> bool {
        // (한국어) 
        // 월드 좌표계상 x축, y축 점을 OBB의 로컬 좌표계로 변환한다.
        // 
        // (English Translation) 
        // Convert the x- and y-axis points in the world coordinate system 
        // to the OBB's local coordinate system.
        //
        let inv_quat = glam::Quat::from_rotation_z(self.radian).inverse();
        let point = inv_quat.mul_vec3((other.0 - self.x, other.1 - self.y, 0.0).into());

        let top = self.y + 0.5 * self.height;
        let left = self.x - 0.5 * self.width;
        let bottom = self.y - 0.5 * self.height;
        let right = self.x + 0.5 * self.width;
        return left <= point.x && point.x <= right
        && bottom <= point.y && point.y <= top;
    }
}

impl Collider2d<Circle> for OBB {
    fn test(&self, other: &Circle) -> bool {
        // (한국어) 
        // 월드 좌표계상 x축, y축 점을 OBB의 로컬 좌표계로 변환한다.
        // 
        // (English Translation) 
        // Convert the x- and y-axis points in the world coordinate system 
        // to the OBB's local coordinate system.
        //
        let inv_quat = glam::Quat::from_rotation_z(self.radian).inverse();
        let point = inv_quat.mul_vec3((other.x - self.x, other.y - self.y, 0.0).into());

        let top = self.y + 0.5 * self.height;
        let left = self.x - 0.5 * self.width;
        let bottom = self.y - 0.5 * self.height;
        let right = self.x + 0.5 * self.width;

        let top_ex = top + other.radius;
        let left_ex = left - other.radius;
        let bottom_ex = bottom - other.radius;
        let right_ex = right + other.radius;

        if left <= point.x && point.x <= right 
        && bottom_ex <= point.y && point.y <= top_ex {
            return true;
        }

        if left_ex <= point.x && point.x <= right_ex 
        && bottom <= point.y && point.y <= top {
            return true;
        }

        let x_sq = (point.x - left) * (point.x - left);
        let y_sq = (point.y - top) * (point.y - top);
        let dist = f32::sqrt(x_sq + y_sq);
        if other.radius >= dist {
            return true;
        }

        let x_sq = (point.x - right) * (point.x - right);
        let y_sq = (point.y - top) * (point.y - top);
        let dist = f32::sqrt(x_sq + y_sq);
        if other.radius >= dist {
            return true;
        }

        let x_sq = (point.x - left) * (point.x - left);
        let y_sq = (point.y - bottom) * (point.y - bottom);
        let dist = f32::sqrt(x_sq + y_sq);
        if other.radius >= dist {
            return true;
        }

        let x_sq = (point.x - right) * (point.x - right);
        let y_sq = (point.y - bottom) * (point.y - bottom);
        let dist = f32::sqrt(x_sq + y_sq);
        if other.radius >= dist {
            return true;
        }

        return false;
    }
}

impl Collider2d<AABB> for OBB {
    fn test(&self, other: &AABB) -> bool {
        let rotation = glam::Quat::from_rotation_z(self.radian);
        let a = [
            (-0.5 * self.width, 0.5 * self.height),
            (-0.5 * self.width, - 0.5 * self.height),
            (0.5 * self.width, - 0.5 * self.height),
            (0.5 * self.width, 0.5 * self.height),
        ].map(|p| {
            let v = rotation.mul_vec3((p.0, p.1, 0.0).into());
            (v.x + self.x, v.y + self.y)
        });

        let b = [
            (other.x - 0.5 * other.width, other.y + 0.5 * other.height),
            (other.x - 0.5 * other.width, other.y - 0.5 * other.height),
            (other.x + 0.5 * other.width, other.y - 0.5 * other.height),
            (other.x + 0.5 * other.width, other.y + 0.5 * other.height),
        ];

        gjk::intersect(a.iter(), b.iter())
    }
}

impl Collider2d<OBB> for OBB {
    fn test(&self, other: &OBB) -> bool {
        let rotation = glam::Quat::from_rotation_z(self.radian);
        let a = [
            (-0.5 * self.width, 0.5 * self.height),
            (-0.5 * self.width, -0.5 * self.height),
            (0.5 * self.width, -0.5 * self.height),
            (0.5 * self.width, 0.5 * self.height),
        ].map(|p| {
            let v = rotation.mul_vec3((p.0, p.1, 0.0).into());
            (v.x + self.x, v.y + self.y)
        });

        let rotation = glam::Quat::from_rotation_z(other.radian);
        let b = [
            (-0.5 * other.width, 0.5 * other.height),
            (-0.5 * other.width, -0.5 * other.height),
            (0.5 * other.width, -0.5 * other.height),
            (0.5 * other.width, 0.5 * other.height),
        ].map(|p| {
            let v = rotation.mul_vec3((p.0, p.1, 0.0).into());
            (v.x + other.x, v.y + other.y)
        });

        gjk::intersect(a.iter(), b.iter())
    }
}



mod gjk {
    use glam::Vec3;

    #[inline]
    fn support_point(points: &[Vec3], dir: Vec3) ->  Vec3{
        points.iter()
        .max_by(|lhs, rhs| lhs.dot(dir).total_cmp(&rhs.dot(dir)))
        .expect("The given points are empty!")
        .clone()
    }

    #[inline]
    fn minkowsk_support_point(a: &[Vec3], b: &[Vec3], dir: Vec3) -> Vec3 {
        support_point(a, dir) - support_point(b, -dir)
    }

    pub fn intersect<'a, A, B>(a: A, b: B) -> bool 
    where A: Iterator<Item = &'a (f32, f32)>, B: Iterator<Item = &'a (f32, f32)> {
        const DIR: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let a: Vec<_> = a.map(|v| Vec3::new(v.0, v.1, 0.0)).collect();
        let b: Vec<_> = b.map(|v| Vec3::new(v.0, v.1, 0.0)).collect();

        let vec_oa = minkowsk_support_point(&a, &b, DIR);
        let vec_ao = -vec_oa;
        let vec_ob = minkowsk_support_point(&a, &b, vec_ao);
        if vec_ob.dot(vec_ao) < 0.0 {
            return false;
        }

        let vec_ab = vec_ao + vec_ob;
        let dir = (vec_ab.cross(vec_ao)).cross(vec_ab);
        let vec_oc = minkowsk_support_point(&a, &b, dir);
        if vec_oc.dot(dir) < 0.0 {
            return false;
        }

        let vec_co = -vec_oc;
        let vec_ca = vec_oa - vec_oc;
        let vec_cb = vec_ob - vec_oc;
        if ((vec_ca.cross(vec_cb)).cross(vec_cb)).dot(vec_co) <= 0.0
        && ((vec_cb.cross(vec_ca)).cross(vec_ca)).dot(vec_co) <= 0.0 {
            return true;
        } else {
            return false;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aabb_obb_test() {
        let a = AABB { x: 1.0, y: 1.0, width: 2.0, height: 2.0 };
        let b = OBB { x: -1.0, y: 0.0, width: 1.999, height: 1.0, radian: 0f32.to_radians() };
        assert!(!b.test(&a));

        let b = OBB { x: -1.0, y: 0.0, width: 1.999, height: 1.0, radian: 3f32.to_radians() };
        assert!(b.test(&a));
        
        let b = OBB { x: 1.0, y: -1.0, width: 2.5, height: 1.0, radian: 90f32.to_radians() };
        assert!(b.test(&a));
    }
}