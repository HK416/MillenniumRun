pub mod shape;



/// #### 한국어 </br>
/// 충돌체가 다른 객체와 충돌하는지 확인하는 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an interface 
/// that checks whether a collider collides with another object. </br>
/// 
pub trait Collider2d<T> : std::fmt::Debug {
    /// #### 한국어 </br>
    /// 두 객체가 서로 충돌하는지 테스트 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Tests whether two object collide with each other. </br>
    /// 
    fn test(&self, other: &T) -> bool;
}
