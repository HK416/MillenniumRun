use std::fmt;

use glam::{
    Mat4, 
    Mat3,
    Vec3, 
    Vec4, 
    Quat, 
    Vec4Swizzles, 
};



/// #### 한국어 </br>
/// 월드 좌표계에 존재하는 게임 오브젝트의 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the interface for game object that exists in the world coordinate system. </br>
/// 
pub trait GameObject : fmt::Debug {
    /// #### 한국어 </br>
    /// 게임 오브젝트의 변환 행렬을 빌려옵니다. (reference) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the game object's transformation matrix. (reference) </br>
    /// 
    fn ref_transform(&self) -> &Mat4;

    /// #### 한국어 </br>
    /// 게임 오브젝트의 변환 행렬을 빌려옵니다. (mutable) </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the game object's transformation matrix. (mutable) </br>
    /// 
    fn mut_transform(&mut self) -> &mut Mat4;

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 위치를 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the position of the game object in world coordinates. </br>
    /// 
    #[inline]
    fn set_position(&mut self, position: Vec3) {
        let mat = self.mut_transform();
        mat.w_axis.x = position.x;
        mat.w_axis.y = position.y;
        mat.w_axis.z = position.z;
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 위치를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the position of the game object in world coordinates. </br>
    /// 
    #[inline]
    fn get_position(&self) -> Vec3 {
        let mat = self.ref_transform();
        return mat.w_axis.xyz();
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 회전을 설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Sets the rotation of the game object in world coordinates. </br>
    /// 
    #[inline]
    fn set_rotation(&mut self, rotation: Quat) {
        let mat = self.mut_transform();
        let rot = Mat3::from_quat(rotation.normalize());
        mat.x_axis.x = rot.x_axis.x;
        mat.x_axis.y = rot.x_axis.y;
        mat.x_axis.z = rot.x_axis.z;

        mat.y_axis.x = rot.y_axis.x;
        mat.y_axis.y = rot.y_axis.y;
        mat.y_axis.z = rot.y_axis.z;

        mat.z_axis.x = rot.z_axis.x;
        mat.z_axis.y = rot.z_axis.y;
        mat.z_axis.z = rot.z_axis.z;        
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 회전을 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the rotation of the game object in world coordinates. </br>
    /// 
    #[inline]
    fn get_rotation(&self) -> Quat {
        let mat = self.ref_transform();
        return Quat::from_mat4(mat).normalize();
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 오른쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the right vector of game object in world coordinates. </br>
    /// 
    #[inline]
    fn get_right_vec(&self) -> Vec3 {
        let mat = self.ref_transform();
        return mat.x_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 위쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the up vector of game object in world coordinates. </br>
    /// 
    #[inline]
    fn get_up_vec(&self) -> Vec3 {
        let mat = self.ref_transform();
        return mat.y_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 게임 오브젝트의 월드 좌표상의 앞쪽 벡터를 가져옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Gets the look vector of game object in world coordinates. </br>
    /// 
    #[inline]
    fn get_look_vec(&self) -> Vec3 {
        let mat = self.ref_transform();
        return mat.z_axis.xyz().normalize();
    }

    /// #### 한국어 </br>
    /// 주어진 거리만큼 게임 오브젝트의 방향으로 게임 오브젝트를 이동시킵니다.
    /// 
    /// #### English (Translation) </br>
    /// Moves the game object in the direction of the game object by a given distance. </br>
    /// 
    #[inline]
    fn translate_local(&mut self, distance: Vec3) {
        let right = self.get_right_vec();
        let up = self.get_up_vec();
        let look = self.get_look_vec();

        let distance = right * distance.x + up * distance.y + look * distance.z;
        self.translate_world(distance);
    }

    /// #### 한국어 </br>
    /// 주어진 거리만큼 월드 좌표계 축 방향으로 게임 오브젝트를 이동시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Moves the game object along the world coordinate system axis by a given distance. </br>
    /// 
    #[inline]
    fn translate_world(&mut self, distance: Vec3) {
        let mat = self.mut_transform();
        mat.w_axis.x += distance.x;
        mat.w_axis.y += distance.y;
        mat.w_axis.z += distance.z;
    }

    /// #### 한국어 </br>
    /// 주어진 회전만큼 게임 오브젝트를 회전시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Rotates the game object by a given rotation. </br>
    /// 
    #[inline]
    fn rotate(&mut self, rotation: Quat) {
        let mat = self.mut_transform();
        let rot = Mat4::from_quat(rotation.normalize());
        *mat = rot.mul_mat4(&mat);
    }

    /// #### 한국어 </br>
    /// 게임오브젝트가 월드 좌표에서 주어진 위치를 바라보도록 합니다. </br>
    /// <b>주어진 위치와 게임 오브젝트의 위치가 같으면 애플리케이션 실행을 중단시킵니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Makes the game object face the given position in world coordinate.
    /// <b>Aborts application execution if the given position is the same as the game object's position.</b></br>
    /// 
    #[inline]
    fn look_at_point(&mut self, point: Vec3) {
        let position = self.get_position();
        let up = self.get_up_vec();
        let look = (point - position).try_normalize().expect("A given point must not equal a position.");
        let right = up.cross(look).normalize();
        let up = look.cross(right).normalize();

        let mat = self.mut_transform();
        mat.x_axis.x = right.x;
        mat.x_axis.y = right.y;
        mat.x_axis.z = right.z;

        mat.y_axis.x = up.x;
        mat.y_axis.y = up.y;
        mat.y_axis.z = up.z;

        mat.z_axis.x = look.x;
        mat.z_axis.y = look.y;
        mat.z_axis.z = look.z;
    }
} 



/// #### 한국어 </br>
/// 월드 좌표계에 존재하는 게임 카메라 오브젝트의 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the interface for the game camera object that exists in the world coordinates system. </br>
/// 
pub trait GameCameraObject : GameObject {
    /// #### 한국어 </br>
    /// 카메라 오브젝트의 카메라 변환 행렬을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the camera transformation matrix of the camera object. </br>
    /// 
    #[inline]
    fn camera_transform(&self) -> Mat4 {
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

    /// #### 한국어 </br>
    /// 카메라 오브젝트의 투영 변환 행렬을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the projection transformation of the camera object. </br>
    /// 
    fn projection_transform(&self) -> Mat4;
}
