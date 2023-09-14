use std::fmt;
use std::result::Result;



/// #### 한국어 </br>
/// `PanicMsg`를 반환하는 [`Result`](std::result::Result)의 래퍼 타입 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A wrapper type for [`Result`](std::result::Result) that returns `PanicMsg`. </br>
/// 
pub type AppResult<T> = Result<T, PanicMsg>;


#[macro_export]
macro_rules! panic_msg {
    ($summary:expr, $($message:tt)*) => {
        PanicMsg::new(
            file!(), 
            line!(), 
            column!(),
            $summary,
            format_args!($($message)*).to_string()
        )
    };
}


/// #### 한국어 </br>
/// 프로그램을 종료시켜야 하는 런타임 에러가 발생한 경우 사용자에게 보여줄 메시지를 저장하는 자료형 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a data type that stores a message to be displayed to the user </br>
/// when a runtime error that requires the program to be terminated occurs. </br>
/// 
#[derive(Clone, PartialEq, Eq)]
pub struct PanicMsg {
    file: String,
    line: u32,
    column: u32,
    summary: String,
    message: String,
}

impl PanicMsg {
    /// #### 한국어 </br>
    /// 새로운 `PanicMsg`를 생성합니다. </br>
    /// <b>이 함수를 직접 호출하는 대신 `panic_msg` 매크로를 호출하여 생성해야 합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a new `PanicMsg`. </br>
    /// <b>Instead of calling the function directly, it must be created by calling the `panic_msg` macro.</b></br>
    /// 
    #[inline]
    pub fn new<F, S, M>(file: F, line: u32, column: u32, summary: S, message: M) -> Self 
    where F: Into<String>, S: Into<String>, M: Into<String> {
        Self {
            file: file.into(),
            line,
            column,
            summary: summary.into(),
            message: message.into(),
        }
    }

    /// #### 한국어 </br>
    /// 화면에 창을 띄워 메시지를 표시한 합니다. 그 후에 프로그램 실행을 중단합니다. </br>
    /// <b>메모:이 함수는 이벤트 루프에서만 호출되어야 합니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// A window should be displayed on the screen to display a message. </br>
    /// After that, the program stops running. </br>
    /// <b>Note: This function must only be called from the event loop.</b></br>
    /// 
    pub(super) fn abort(self) -> ! {
        use std::process::abort;
        use native_dialog::{MessageDialog, MessageType};

        log::error!("{:?}", &self);
        unsafe {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title(&self.summary)
                .set_text(&self.message)
                .show_alert()
                .unwrap_unchecked() 
        };

        abort()
    }
}

impl fmt::Debug for PanicMsg {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] - \"{}\" <{}:{}:{}>", &self.summary, &self.message, &self.file, &self.line, &self.column)
    }
}

impl fmt::Display for PanicMsg {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] - \"{}\"", &self.summary, &self.message)
    }
}
