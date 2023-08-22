use super::{PanicErr, KeyCode, MouseButton};



/// #### 한국어
/// 어플리케이션의 이벤트 루프에게 전달되는 명령의 종류들 입니다.
/// 어플리케이션 이벤트 루프는 전달된 명령을 처리합니다.
/// 
/// #### English (Translation)
/// These are the types of commands passed to the application's event loop.
/// The application event loop processes passed commands.
/// 
#[derive(Debug, Clone)]
pub enum AppCmd {
    /// #### 한국어
    /// 어플리케이션 이벤트 루프에게 프로그램을 중단시킬것을 명령합니다.
    /// 
    /// #### English (Translation)
    /// order to the application event loop to abort the program.
    /// 
    PanicError(PanicErr),

    /// #### 한국어
    /// 어플리케이션 이벤트 루프에게 프로그램을 종료시킬것을 명령합니다.
    /// 
    /// #### English (Translation)
    /// Order to the application event loop to terminate the program.
    Terminate,
}


/// #### 한국어
/// 어플리케이션 이벤트 루프에서 게임 로직 루프에 전달되는 이벤트의 종류들 입니다.
/// 
/// #### English (Translation)
/// These are the types of events passed from the application event loop to the game logic loop.
/// 
#[derive(Debug, Clone)]
pub enum GameLogicEvent {
    /// #### 한국어
    /// 이벤트가 시작됨을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that an event has started.
    /// 
    NextMainEvents,

    /// #### 한국어
    /// 모든 이벤트가 처리되었음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event indicates that all events have been processed.
    /// 
    MainEventsCleared,

    /// #### 한국어
    /// 어플리케이션이 일시정지되었음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the application is paused.
    /// 
    ApplicationPaused,

    /// #### 한국어
    /// 어플리케이션이 재개하였음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the application has resumed.
    /// 
    ApplicationResumed,

    /// #### 한국어
    /// 어플리케이션이 곧 종료됨을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the application is about to terminate.
    /// 
    ApplicationTerminate,

    /// #### 한국어
    /// 어플리케이션 윈도우의 크기 또는 배율이 변경되었음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the size or magnification of the application window has changed.
    /// 
    WindowResized,

    /// #### 한국어
    /// 어플리케이션 윈도우의 위치가 이동되었음을 알리는 이벤트 입니다.
    /// 이동된 윈도우의 가로와 세로의 픽셀 좌표 정보를 포함합니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the location of the application window has moved.
    /// Contains horizontal and vertical pixel coordinate information of the moved window.
    /// 
    WindowMoved { x: i32, y: i32 },

    /// #### 한국어
    /// 키보드의 키가 눌렸음을 알리는 이벤트 입니다. 눌린 키의 식별 코드를 포함합니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that a key on the keyboard has been pressed.
    /// Contains the identification code of the pressed key.
    /// 
    KeyPressed(KeyCode),

    /// #### 한국어
    /// 키보드의 키가 떼어졌음을 알리는 이벤트 입니다. 떼어진 키의 식별 코드를 포함합니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that a key on the keyboard has been released.
    /// Contains the identification code of the released key.
    /// 
    KeyReleased(KeyCode),

    /// #### 한국어
    /// 마우스의 커서가 이동되었음을 알리는 이벤트 입니다.
    /// 윈도우의 왼쪽 위 모서리를 기준으로 픽셀 단위로 조정됩니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the mouse cursor has moved.
    /// (x,y) coords in pixels relative to the top-left corner of the window. 
    /// 
    CursorMoved { x: f32, y: f32 },

    /// #### 한국어
    /// 마우스 휠이 조작되었음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the mouse wheel has been operated.
    /// 
    MouseWheel { horizontal: f32, vertical: f32 },

    /// #### 한국어
    /// 마우스 버튼이 눌렸음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the mouse button has been pressed.
    /// 
    MousePressed(MouseButton),
    
    /// #### 한국어
    /// 마우스 버튼이 떼어졌음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the mouse button has been released.
    MouseReleased(MouseButton),
}



/// #### 한국어
/// 어플리케이션 이벤트 루프에서 게임 렌더 루프에 전달되는 이벤트의 종류들 입니다.
/// 
/// #### English (Translation)
/// These are the types of events passed from the application event loop to the game render loop.
/// 
#[derive(Debug, Clone)]
pub enum GameRenderEvent {
    /// #### 한국어
    /// 어플리케이션이 곧 종료됨을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the application is about to terminate.
    /// 
    ApplicationTerminate,

    /// #### 한국어
    /// 어플리케이션 윈도우의 크기 또는 배율이 변경되었음을 알리는 이벤트 입니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that the size or magnification of the application window has changed.
    /// 
    WindowResized,
}
