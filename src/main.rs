mod assets;
mod framework;
mod locale;
mod timer;
mod resolution;
mod user_setting;

use async_std::task;
use crate::framework::Framework;


/// #### 한국어
/// 프로그램의 진입점 입니다.
/// `Windows`, `Linux`, `macOS` 이 세 가지 운영체제만 어플리케이션이 동작합니다.
/// 
/// #### English (Translation)
/// It is the entry point of the program.
/// The application works only on these three operating systems: `Windows`, `Linux`, and `macOS`.
/// 
fn main() {
    env_logger::init();
    log::info!("❖ Application Launching. ❖");

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))] {
        task::block_on(Framework::new()).run()
    }

    #[allow(unreachable_code)] {
        panic!("❗️❗️❗️ This platform is not supported. ❗️❗️❗️")
    }
}
