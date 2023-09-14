use std::sync::mpsc::Sender;

use crossbeam::queue::SegQueue;
use winit::event::{
    MouseButton, 
    VirtualKeyCode
};

use crate::{
    panic_msg, 
    app::{
        abort::{PanicMsg, AppResult},
        running_flag::RunningFlag,
    }
};


/// #### 한국어 </br>
/// 각 스레드에서 어플리케이션 이벤트 루프(메인 스레드)에 보내는 명령 대기열 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the command queue that each thread sends to the application event loop (main thread). </br>
/// 
static mut CMD_QUEUE: SegQueue<AppCommand> = SegQueue::new();



/// #### 한국어 </br>
/// 어플리케이션 이벤트 루프(메인 스레드)에 명령어를 보내는 채널 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a channel that sends commands to the application event loop (main thread). </br>
/// 
#[derive(Debug)]
pub struct AppCommandChannel;

impl AppCommandChannel {
    /// #### 한국어 </br>
    /// 명령 대기열에 있는 오래된 명령어를 가져옵니다. </br>
    /// 만약 명령 대기열이 비어있는 경우 `None`을 반환합니다. </br>
    /// <b>메모: 이 함수는 이벤트 루프에서만 호출되어야 합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Fetch old commands from the command queue. </br>
    /// If the command queue is empty, it returns `None`. </br>
    /// <b>Note: This function must only be called from the event loop.</b></br>
    /// 
    #[inline]
    pub(super) fn recv() -> Option<AppCommand> {
        unsafe { CMD_QUEUE.pop() }
    }

    /// #### 한국어 </br>
    /// 명령 대기열에 명령어를 추가합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Add a command to the command queue. </br>
    /// 
    #[inline]
    pub fn send(command: AppCommand) {
        unsafe { CMD_QUEUE.push(command) }
    }
}



/// #### 한국어 </br>
/// 어플리케이션의 이벤트 루프에게 전달되는 명령의 종류들 입니다. </br>
/// 어플리케이션 이벤트 루프는 전달된 명령을 처리합니다. </br>
/// 
/// #### English (Translation) </br>
/// These are the types of commands passed to the application's event loop. </br>
/// The application event loop processes passed commands. </br>
/// 
#[derive(Debug, Clone)]
pub enum AppCommand {
    /// #### 한국어 </br>
    /// 어플리케이션 이벤트 루프에게 프로그램을 중단시킬것을 명령합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// order to the application event loop to abort the program. </br>
    /// 
    Panic(PanicMsg),

    /// #### 한국어 </br>
    /// 어플리케이션 이벤트 루프에게 프로그램을 종료시킬것을 명령합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Order to the application event loop to terminate the program. </br>
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
    NextMainEvents(f64),

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
    KeyPressed(VirtualKeyCode),

    /// #### 한국어
    /// 키보드의 키가 떼어졌음을 알리는 이벤트 입니다. 떼어진 키의 식별 코드를 포함합니다.
    /// 
    /// #### English (Translation)
    /// This event notifies that a key on the keyboard has been released.
    /// Contains the identification code of the released key.
    /// 
    KeyReleased(VirtualKeyCode),

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



/// #### 한국어 </br>
/// 현재 스레드에서 다른 스레드로 메시지를 전달하는 함수 입니다. </br>
/// 함수를 실행하는 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a function that passes a message from the current thread to anther thread. </br>
/// If an error occurs while executing the function, it returns `PanicMsg`. </br>
/// 
pub fn send<T>(msg: T, sender: &Sender<T>) -> AppResult<()> {
    if RunningFlag::is_running() {
        sender.send(msg).map_err(|e| panic_msg!(
            "Message send failed",
            "Message send failed for the following reasons: {}",
            e.to_string()
        ))?;
    }
    Ok(())
}



/// #### 한국어 </br>
/// `AppResult`의 결과를 처리하는 함수입니다. </br>
/// `PanicMsg`가 발생한 경우 메인 스레드로 `PanicMsg`를 전달하고 현재 스레드를 종료시킵니다. </br>
/// 
/// #### English (Translation)
/// This is a function that processes the result of `AppResult`. </br>
/// If a `PanicMsg` occurs, it passes `PanicMsg` to the main thread 
/// and terminates the current thread. </br>
/// 
#[inline]
pub fn success<T>(result: AppResult<T>) -> T {
    match result {
        Ok(val) => val,
        Err(msg) => {
            RunningFlag::set_exit();
            AppCommandChannel::send(AppCommand::Panic(msg.clone()));
            panic!("{}", msg);
        }
    }
}

