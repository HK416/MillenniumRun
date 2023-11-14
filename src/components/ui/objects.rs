/// #### 한국어 </br>
/// 유저 인터페이스 오브젝트의 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the interface of the user interface object. </br>
/// 
pub trait UiObject : std::fmt::Debug {
    /// #### 한국어 </br>
    /// 렌더 패스에 쉐이더 변수를 바인딩 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Bind shader variables to the render pass. </br>
    /// 
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>);

    /// #### 한국어 </br>
    /// 유저 인터페이스를 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws user interface to the screen. </br>
    /// 
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>);
}


/// #### 한국어 </br>
/// 버튼 오브젝트의 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the interface of button object. </br>
/// 
pub trait UiButtonObject : UiObject {
    /// #### (한국어) </br>
    /// 마우스 버튼이 눌렸을때 호출되는 함수입니다. </br>
    /// 광선과 버튼 영역이 교차할 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This function is called when the mouse button is pressed. </br>
    /// Returns `true` if the ray intersects the button area. </br>
    /// 
    fn mouse_pressed(&self, x: f32, y: f32, ortho: &glam::Mat4) -> bool;

    /// #### (한국어) </br>
    /// 마우스 버튼이 떼어졌을때 호출되는 함수입니다. </br>
    /// 광선과 버튼 영역이 교차할 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This function is called when the mouse button is released. </br>
    /// Returns `true` if the ray intersects the button area. </br>
    /// 
    fn mouse_released(&self, x: f32, y: f32, ortho: &glam::Mat4) -> bool;
}
