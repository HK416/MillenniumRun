use std::fmt;

use glam::{
    Mat4, Affine3A, Quat, Vec3, 
    EulerRot, Vec4Swizzles,
};

use crate::app::{
    abort::AppResult, 
    message::GameLogicEvent,
};



/// #### 한국어 </br>
/// 장면을 구성하는 모든 물체에 대한 기본 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the primary interface for all objects that make up a scene. </br>
/// 
pub trait GameObject : fmt::Debug {
    /// #### 한국어 </br>
    /// `GameObject`의 회전과 위치를 주어진 3차원 월드 좌표상의 회전과 위치로 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the rotation and position of the `GameObject` to the given rotation and position in 3D world coordinates. </br>
    /// 
    #[inline]
    fn set_rotation_translation(&mut self, rotation: Quat, translation: Vec3) {
        assert!(rotation.is_normalized(), "A given rotation must be normalized.");
        *self.mut_transform() = Mat4::from_rotation_translation(rotation, translation);
    }
    
    /// #### 한국어 </br>
    /// 3차원 월드 좌표상 `GameObject`의 위치를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the position of a `GameObject` in 3D world coordinates. </br>
    /// 
    #[inline]
    fn get_translation(&self) -> Vec3 {
        self.ref_transform().w_axis.xyz()
    }
    
    /// #### 한국어 </br>
    /// `GameObject`의 위치를 주어진 3차원 월드 좌표상의 위치로 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the position of `GameObject` to the given 3D world coordinates. </br>
    /// 
    #[inline]
    fn set_translation(&mut self, translation: Vec3) {
        let rotation: Quat = self.get_rotation();
        self.set_rotation_translation(rotation, translation)
    }
    
    /// #### 한국어 </br>
    /// `GameObject`의 위치를 주어진 3차원 월드 좌표상의 거리 만큼 이동시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Move the position of `GameObject` by the given distance in 3D world coordinates. </br>
    /// 
    #[inline]
    fn translate_world(&mut self, distance: Vec3) {
        *self.mut_transform() = (
            Affine3A::from_translation(distance) 
            * Affine3A::from_mat4(self.ref_transform().clone())
        ).into();
    }

    /// #### 한국어 </br>
    /// `GameObject`의 위치를 주어진 3차원 로컬 좌표상의 거리 만큼 이동시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Move the position of `GameObject` by the given distance in 3D local coordinates. </br>
    /// 
    #[inline]
    fn translate_local(&mut self, distance: Vec3) {
        let distance = self.get_right_norm() * distance.x
            + self.get_up_norm() * distance.y
            + self.get_look_norm() * distance.z;
        self.translate_world(distance)
    } 
    
    /// #### 한국어 </br>
    /// 3차원 월드 좌표상 `GameObject`의 회전을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the rotation of the `GameObject` in 3D world coordinates. </br>
    /// 
    #[inline]
    fn get_rotation(&self) -> Quat {
        Quat::from_mat4(self.ref_transform()).normalize()
    }

    /// #### 한국어 </br>
    /// 3차원 월드 좌표상 `GameObject`의 정규화된 오른쪽 벡터를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the normalized right vector of the `GameObject` in 3D world coordinates. </br>
    /// 
    #[inline]
    fn get_right_norm(&self) -> Vec3 {
        self.ref_transform().x_axis.xyz().normalize()
    }

    /// #### 한국어 </br>
    /// 3차원 월드 좌표상 `GameObject`의 정규화된 위쪽 벡터를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the normalized up vector of the `GameObject` in 3D world coordinates. </br>
    /// 
    #[inline]
    fn get_up_norm(&self) -> Vec3 {
        self.ref_transform().y_axis.xyz().normalize()
    }

    /// #### 한국어 </br>
    /// 3차원 월드 좌표상 `GameObject`의 정규화된 앞쪽 벡터를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the normalized forward vector of the `GameObject` in 3D world coordinates. </br>
    /// 
    #[inline]
    fn get_look_norm(&self) -> Vec3 {
        self.ref_transform().z_axis.xyz().normalize()
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 주어진 3차원 월드 좌표상의 회전으로 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the rotation of `GameObject` to the given rotation in 3D world coordinates. </br>
    /// 
    #[inline]
    fn set_rotation(&mut self, rotation: Quat) {
        let rotation: Quat = rotation.normalize();
        let translation: Vec3 = self.get_translation();
        self.set_rotation_translation(rotation, translation)
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 주어진 3차원 월드 좌표상의 회전만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by the given rotation in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation(&mut self, rotation: Quat) {
        *self.mut_transform() = (
            Affine3A::from_quat(rotation.normalize())
            * Affine3A::from_mat4(self.ref_transform().clone())
        ).into();
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 주어진 3차원 월드 좌표상의 축과 각도(라디안)만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by an angle(in radians) with the given axis in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation_axis_angle(&mut self, axis: Vec3, angle: f32) {
        self.rotation(Quat::from_axis_angle(axis.normalize(), angle))
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 3차원 월드 좌표의 축과 주어진 각도(라디안)만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by the given angle(in radians) with an axis in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation_euler(&mut self, euler: EulerRot, a: f32, b: f32, c: f32) {
        self.rotation(Quat::from_euler(euler, a, b, c))
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 3차원 월드 좌표의 x축과 주어진 각도(라디안)만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by the given angle(in radians) with the x-axis in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation_x(&mut self, angle: f32) {
        self.rotation(Quat::from_rotation_x(angle))
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 3차원 월드 좌표의 y축과 주어진 각도(라디안)만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by the given angle(in radians) with the y-axis in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation_y(&mut self, angle: f32) {
        self.rotation(Quat::from_rotation_y(angle))
    }

    /// #### 한국어 </br>
    /// `GameObject`의 회전을 3차원 월드 좌표의 z축과 주어진 각도(라디안)만큼 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the rotation of `GameObject` by the given angle(in radians) with the z-axis in 3D world coordinates. </br>
    /// 
    #[inline]
    fn rotation_z(&mut self, angle: f32) {
        self.rotation(Quat::from_rotation_z(angle))
    }

    /// #### 한국어 </br>
    /// 주어진 3차원 월드 좌표상의 위치를 바라도록 `GameObject`의 회전을 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the rotation of the `GameObject` to look at the given position in 3D world coordintates. </br>
    /// 
    fn look_at_point(&mut self, point: Vec3) {
        use std::f32::EPSILON;
        let distance: Vec3 = point - self.get_translation();
        assert!(!(distance.length_squared() <= EPSILON), "A given point must not equal a position.");

        let up = self.get_up_norm();
        let look = distance.normalize();
        let right = up.cross(look).normalize();
        let up = look.cross(right).normalize();

        self.mut_transform().x_axis = (right, 0.0).into();
        self.mut_transform().y_axis = (up, 0.0).into();
        self.mut_transform().z_axis = (look, 0.0).into();
    }

    /// #### 한국어 </br>
    /// `GameObject`의 월드 좌표 변환 행렬을 대여합니다. (reference ver) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the world coordinate transformation matrix of `GameObject`. (reference ver) </br>
    /// 
    fn ref_transform(&self) -> &Mat4;

    /// #### 한국어 </br>
    /// `GameObject`의 월드 좌표 변환 행렬을 대여합니다. (mutable ver) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the world coordinate transformation matrix of `GameObject`. (mutable ver) </br>
    /// 
    fn mut_transform(&mut self) -> &mut Mat4;

    /// #### 한국어 </br>
    /// 윈도우 이벤트를 처리하는 함수입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This function handles window events. </br>
    /// 
    #[allow(unused_variables)]
    fn handle_events(&mut self, event: &GameLogicEvent) -> AppResult<()> { Ok(()) }

    /// #### 한국어 </br>
    /// 경과된 시간만큼 `GameObject`를 갱신하는 함수입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This function updates `GameObject` as much as the elapsed time. </br>
    /// 
    #[allow(unused_variables)]
    fn update(&mut self, time: f64) -> AppResult<()> { Ok(()) }
}



#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct TestObject(Mat4);

    impl GameObject for TestObject { 
        fn ref_transform(&self) -> &Mat4 {
            &self.0
        }

        fn mut_transform(&mut self) -> &mut Mat4 {
            &mut self.0
        }
    }

    #[test]
    fn test_rotation_translation() {
        use std::f32::EPSILON;

        let mut obj: Box<dyn GameObject> = Box::new(TestObject(
            Mat4::from_translation((1.0, 0.0, 0.0).into())
        ));

        // (1.0, 0.0, 0.0) -> (6.0, 0.0, 0.0)
        const RES_0: Vec3 = Vec3 { x: 6.0, y: 0.0, z: 0.0 };
        obj.translate_world((5.0, 0.0, 0.0).into());
        assert!((RES_0 - obj.get_translation()).length_squared() <= EPSILON);
        
        // (6.0, 0.0, 0.0) -> (0.0, 6.0, 0.0)
        const RES_1: Vec3 = Vec3 { x: 0.0, y: 6.0, z: 0.0 };
        obj.rotation_axis_angle((0.0, 0.0, 1.0).into(), 90f32.to_radians());
        assert!((RES_1 - obj.get_translation()).length_squared() <= EPSILON);
        
        // (0.0, 6.0, 0.0) -> (0.0, 9.0, 0.0)
        const RES_2: Vec3 = Vec3 { x: 0.0, y: 9.0, z: 0.0 };
        obj.translate_local((3.0, 0.0, 0.0).into());
        assert!((RES_2 - obj.get_translation()).length_squared() <= EPSILON);

        // (0.0, 9.0, 0.0) -> (9.0, 0.0, 0.0)
        const RES_3: Vec3 = Vec3 { x: 9.0, y: 0.0, z: 0.0 };
        obj.rotation_axis_angle((0.0, 0.0, 1.0).into(), -90f32.to_radians());
        assert!((RES_3 - obj.get_translation()).length_squared() <= EPSILON);
    }
}
