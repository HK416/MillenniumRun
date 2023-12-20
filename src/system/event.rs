/// #### 한국어 </br>
/// 애플리케이션 이벤트 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of application events. </br>
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    /// #### 한국어 </br>
    /// 애플리케이션을 종료 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Quit the application. </br>
    /// 
    Terminate,
}
