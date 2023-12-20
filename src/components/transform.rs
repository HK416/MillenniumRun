use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Mat3, Quat, Vec4, Vec3, Vec4Swizzles};



/// #### 한국어 </br>
/// 월드 좌표계상의 변환 행렬입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the transformation matrix in the world coordinate system. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Transform {
    inner: Mat4,
}

#[allow(dead_code)]
impl Transform {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 위치를 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the position in world coordinates. </br>
    /// 
    #[inline]
    pub fn set_position(&mut self, position: Vec3) {
        self.inner.w_axis.x = position.x;
        self.inner.w_axis.y = position.y;
        self.inner.w_axis.z = position.z;
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 위치를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the position in world coordinates. </br>
    /// 
    #[inline]
    pub fn get_position(&self) -> Vec3 {
        return self.inner.w_axis.xyz()
    }


    /// #### 한국어 </br>
    /// 월드 좌표상의 회전을 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the rotation in world coordinates. </br>
    /// 
    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat) {
        let mat = Mat3::from_quat(rotation.normalize());
        self.inner.x_axis.x = mat.x_axis.x;
        self.inner.x_axis.y = mat.x_axis.y;
        self.inner.x_axis.z = mat.x_axis.z;

        self.inner.y_axis.x = mat.y_axis.x;
        self.inner.y_axis.y = mat.y_axis.y;
        self.inner.y_axis.z = mat.y_axis.z;

        self.inner.z_axis.x = mat.z_axis.x;
        self.inner.z_axis.y = mat.z_axis.y;
        self.inner.z_axis.z = mat.z_axis.z;        
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 회전을 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the rotation in world coordinates. </br>
    /// 
    #[inline]
    pub fn get_rotation(&self) -> Quat {
        return Quat::from_mat4(&self.inner).normalize();
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 오른쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the right vector in world coordinates. </br>
    /// 
    #[inline]
    pub fn get_right_vec(&self) -> Vec3 {
        return self.inner.x_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 위쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the up vector in world coordinates. </br>
    /// 
    #[inline]
    fn get_up_vec(&self) -> Vec3 {
        return self.inner.y_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 월드 좌표상의 앞쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the look vector in world coordinates. </br>
    /// 
    #[inline]
    pub fn get_look_vec(&self) -> Vec3 {
        return self.inner.z_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 주어진 거리만큼 로컬 축을 따라 위치를 이동시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Moves the position along the local axis by a given distance. </br>
    /// 
    #[inline]
    pub fn translate_local(&mut self, distance: Vec3) {
        let right = self.get_right_vec();
        let up = self.get_up_vec();
        let look = self.get_look_vec();

        let distance = right * distance.x + up * distance.y + look * distance.z;
        self.translate_world(distance);
    }

    /// #### 한국어 </br>
    /// 주어진 거리만큼 월드 좌표계 축을 따라 위치를 이동시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Moves the position along the world axis by a given distance. </br>
    /// 
    #[inline]
    pub fn translate_world(&mut self, distance: Vec3) {
        self.inner.w_axis.x += distance.x;
        self.inner.w_axis.y += distance.y;
        self.inner.w_axis.z += distance.z;
    }

    /// #### 한국어 </br>
    /// 주어진 회전만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates by a given rotation. </br>
    /// 
    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        let mat = Mat4::from_quat(rotation.normalize());
        self.inner = mat.mul_mat4(&self.inner);
    }

    /// #### 한국어 </br>
    /// 주어진 위치를 로컬 z축이 향하도록 합니다. </br>
    /// <b>주어진 위치와 변환 행렬의 위치가 같으면 애플리케이션 실행을 중단시킵니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Orients the local z-axis toward a given location. </br>
    /// <b>Aborts application execution if the given position is the same as the position in the transformation matrix.</b></br>
    /// 
    #[inline]
    pub fn look_at_point(&mut self, point: Vec3) {
        let position = self.get_position();
        let up = self.get_up_vec();
        let look = (point - position).try_normalize().expect("A given point must not equal a position.");
        let right = up.cross(look).normalize();
        let up = look.cross(right).normalize();

        self.inner.x_axis.x = right.x;
        self.inner.x_axis.y = right.y;
        self.inner.x_axis.z = right.z;

        self.inner.y_axis.x = up.x;
        self.inner.y_axis.y = up.y;
        self.inner.y_axis.z = up.z;

        self.inner.z_axis.x = look.x;
        self.inner.z_axis.y = look.y;
        self.inner.z_axis.z = look.z;
    }

    /// #### 한국어 </br>
    /// 카메라 변환 행렬을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the camera transformation matrix. </br>
    /// 
    #[inline]
    pub fn camera_transform(&self) -> Mat4 {
        let position = self.get_position();
        let right = self.get_right_vec();
        let up = self.get_up_vec();
        let look = self.get_look_vec();
        
        return Mat4 { 
            x_axis: Vec4 { x: right.x, y: up.x, z: look.x, w: 0.0 }, 
            y_axis: Vec4 { x: right.y, y: up.y, z: look.y, w: 0.0 }, 
            z_axis: Vec4 { x: right.z, y: up.z, z: look.z, w: 0.0 }, 
            w_axis: Vec4 { 
                x: -position.dot(right), 
                y: -position.dot(up), 
                z: -position.dot(look), 
                w: 1.0 
            } 
        };
    }
}

impl From<Mat4> for Transform {
    #[inline]
    fn from(value: Mat4) -> Self {
        Transform { inner: value }
    }
}

impl Into<Mat4> for Transform {
    #[inline]
    fn into(self) -> Mat4 {
        self.inner
    }
}

impl AsRef<Mat4> for Transform {
    #[inline]
    fn as_ref(&self) -> &Mat4 {
        &self.inner
    }
}

impl AsMut<Mat4> for Transform {
    fn as_mut(&mut self) -> &mut Mat4 {
        &mut self.inner
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self { 
            inner: Mat4::IDENTITY 
        }
    }
}



/// #### 한국어 </br>
/// 원근 투영 변환 행렬을 생성하는 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data to generate a perspective projection transformation matrix. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perspective {
    pub fov_y: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}



/// #### 한국어 </br>
/// 정사영 투영 변환 행렬을 생성하는 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data to generate an orthographic projection transformation matrix. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orthographic {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub near: f32,
    pub far: f32,
}



/// #### 한국어 </br>
/// 투영 행렬을 생성하는데 필요한 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the data needed to generate the projection matrix. </br>
///  
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Perspective(Perspective),
    Orthographic(Orthographic),
}

impl Projection {
    #[inline]
    pub const fn new_ortho(top: f32, left: f32, bottom: f32, right: f32, near: f32, far: f32) -> Self {
        Self::Orthographic(Orthographic { top, left, bottom, right, near, far })
    }

    /// #### 한국어 </br>
    /// 투영 변환 행렬을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the projection transformation. </br>
    /// 
    pub fn projection_transform(self) -> Mat4 {
        match self {
            Self::Perspective(it) => {
                Mat4::perspective_rh(it.fov_y, it.aspect_ratio, it.near, it.far)
            },
            Self::Orthographic(it) => {
                Mat4::orthographic_rh(it.left, it.right, it.bottom, it.top, it.near, it.far)
            }
        }
    }
}
