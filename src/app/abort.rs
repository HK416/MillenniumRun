/// #### 한국어
/// `PanicErr`를 반환하는 [`std::result::Result`]의 래퍼 타입 입니다.
/// 
/// #### English (Translation)
/// A wrapper type for [`std::result::Result`] that returns `PanicErr`.
/// 
pub type AppResult<T> = std::result::Result<T, PanicErr>;


#[macro_export]
macro_rules! panic_err {
    ($summary:expr, $($message:tt)*) => {
        PanicErr::new(
            file!(), 
            line!(), 
            column!(),
            $summary,
            format_args!($($message)*).to_string()
        )
    };
}


/// #### 한국어
/// 프로그램 실행 중 프로그램 실행을 중단시켜야할 중대한 에러가 발생할 경우
/// 오류에 대한 사용자에게 전달하고자 하는 메시지를 담고 있는 구조체 입니다.
/// 
/// #### English (Translation)
/// This is structure containing the message to be delivered to the user
/// abort the error when a serious error that should stop the program execution
/// occurs during program execution.
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicErr {
    file: String,
    line: u32,
    column: u32,
    summary: String,
    message: String,
}

impl PanicErr {
    /// #### 한국어
    /// 새로운 `PanicErr`를 생성합니다.
    /// 이 함수를 직접 호출하는 대신 `panic_err` 매크로를 호출하여 생성해야 합니다.
    /// 
    /// #### English (Translation)
    /// Creates a new `PanicErr`.
    /// Instead of calling the function directly, it must be created by calling the `panic_err` macro.
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

    /// #### 한국어
    /// 오류 메시지 내용을 반환합니다.
    /// 디버그 모드일 경우 디버깅을 위한 메시지가 추가됩니다.
    /// 
    /// #### English (Translation)
    /// Returns the content of the error message.
    /// When in debug mode, messages for debugging are added.
    /// 
    #[inline]
    #[allow(unreachable_code)]
    pub fn display(&self) -> String {
        #[cfg(debug_assertions)]
        return format!("{} ({}::{}::{})", &self.message, &self.file, &self.line, &self.column);
        return self.message.clone()
    }

    /// #### 한국어
    /// 메시지 창을 띄워 오류 메시지를 출력한 뒤, 프로그램 실행을 중단시킵니다.
    /// 이 함수는 되도록 메인 스레드에서 호출해야 합니다.
    /// 그렇지 않을 경우 일부 플랫폼에서 메시지 창을 띄우지 않을 수 있습니다.
    /// 
    /// #### English (Translation)
    /// Displays an error message by popping up a message window, then stops program execution.
    /// This function should preferably be called from the main thread.
    /// Otherwise, the message window may not pop up on some platforms.
    /// 
    pub fn abort(self) -> ! {
        use std::process::abort;
        use native_dialog::{MessageDialog, MessageType};

        log::error!("{} :: {}", &self.summary, &self.display());
        unsafe {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title(&self.summary)
                .set_text(&self.display())
                .show_alert()
                .unwrap_unchecked() 
        };

        abort()
    }
}
