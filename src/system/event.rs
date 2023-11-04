use crate::system::error::GameError;



/// #### 한국어 </br>
/// 애플리케이션 이벤트 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of application events. </br>
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    /// #### 한국어 </br>
    /// 애플리케이션 실행 중 오류가 발생했음을 알립니다. </br>
    /// 오류 메시지를 담고 있습니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Notifies that an error occurred while running the application. </br>
    /// Contains an error message. </br>
    /// 
    GameError(GameError),

    /// #### 한국어 </br>
    /// 애플리케이션을 종료 합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Quit the application. </br>
    /// 
    Terminate,
}
