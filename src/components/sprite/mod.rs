pub mod brush;
pub mod objects;


/// #### 한국어 </br>
/// 스프라이트 오브젝트의 랜더링 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the rendering interface for the sprite object. </br>
/// 
pub trait Sprite: std::fmt::Debug {
    /// #### 한국어 </br>
    /// 렌더 패스에 쉐이더 변수를 바인딩 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Bind shader variables to the render pass. </br>
    /// 
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>);

    /// #### 한국어 </br>
    /// 스프라이트를 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws sprite to the screen. </br>
    /// 
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>);
}