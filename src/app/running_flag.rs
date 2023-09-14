use std::sync::atomic::{AtomicBool, Ordering};



/// #### 한국어 </br>
/// 현재 어플리케이션이 실행중인지 여부를 나타내는 플래그 변수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a flag variable that indicates whether the application is currently running. </br>
/// 
static mut RUNNING: AtomicBool = AtomicBool::new(true);



/// #### 한국어 </br>
/// 어플리케이션이 실행중인지 여부를 나타냅니다. </br>
/// 
/// #### English (Translation) </br>
/// Indicates whether the application is running. </br>
/// 
#[derive(Debug)]
pub struct RunningFlag;

impl RunningFlag {
    /// #### 한국어 </br>
    /// 어플리케이션 실행 여부를 `false`로 바꿉니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Change whether to run the application to `false`. </br>
    /// 
    #[inline]
    pub fn set_exit() {
        unsafe { RUNNING.store(false, Ordering::Release) }
    }

    /// #### 한국어 </br>
    /// 현재 어플리케이션이 실행중인지 확인합니다. </br>
    /// 어플리케이션이 실행중인 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Check whether the application is currently running. </br>
    /// Returns `true` if the application is running. </br>
    /// 
    #[inline]
    pub fn is_running() -> bool {
        unsafe { RUNNING.load(Ordering::Acquire) }
    }
}
