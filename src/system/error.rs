use std::fmt;
use std::result::Result;



#[macro_export]
macro_rules! game_err {
    ($summary:expr, $($message:tt)*) => {
        GameError::new(
            file!(),
            line!(),
            column!(),
            $summary,
            format_args!($($message)*).to_string()
        )
    };
}


/// #### 한국어 </br>
/// [`Result`](std::result::Result)의 래퍼 타입 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A wrapper type for [`Result`](std::result::Result). </br>
/// 
pub type AppResult<T> = Result<T, GameError>;


/// #### 한국어 </br>
/// 애플리케이션 실행 중 발생한 오류 메시지를 담고있습니다. </br>
/// 
/// #### English (Translation)
/// Contains error messages that occurred while running the application. </br>
/// 
#[derive(Clone, PartialEq, Eq)]
pub struct GameError {
    file: String,
    line: u32,
    column: u32,
    summary: String,
    message: String,
}

impl GameError {
    #[inline]
    pub fn new<F, S, M>(file: F, line: u32, column: u32, summary: S, message: M) -> Self 
    where F: Into<String>, S: Into<String>, M: Into<String> {
        Self { 
            file: file.into(), 
            line, 
            column, 
            summary: summary.into(), 
            message: message.into() 
        }
    }
}

impl fmt::Debug for GameError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameError")
            .field("file", &self.file)
            .field("line", &self.line)
            .field("column", &self.line)
            .field("summary", &self.summary)
            .field("message", &self.message)
            .finish()
    }
}

impl ToString for GameError {
    #[inline]
    fn to_string(&self) -> String {
        format!("<{}> \"{}\"", self.summary, self.message)
    }
}



/// #### 한국어 </br>
/// 화면에 에러 메시지를 표시합니다. </br>
/// 사용자가 확인 버튼을 누르면 애플리케이션 실행이 중단됩니다. </br>
/// <b>주의: 이 함수는 메인 스레드에서 호출되어야 합니다.</b></br>
/// 
/// #### English (Translation) </br>
/// Displays an error message on the screen. </br>
/// When the user clicks the OK button, the application aborts running. </br>
/// <b>Caution: This function must be called from the main thread.</b></br>
/// 
#[inline]
pub fn popup_err_msg_and_abort(err: GameError) -> ! {
    use std::process::abort;
    use native_dialog::{
        MessageDialog,
        MessageType,
    };

    log::error!("{:?}", err);
    unsafe {
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title(&err.summary)
            .set_text(&err.message)
            .show_alert()
            .unwrap_unchecked()
    };

    abort()
}
