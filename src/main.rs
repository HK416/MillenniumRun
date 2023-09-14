mod app;
mod assets;
mod game;
mod logic;
mod render;

use crate::app::main_loop::run;



/// #### 한국어 </br>
/// 프로그램의 진입점 입니다. </br>
/// `Windows`, `Linux`, `macOS` 이 세 가지 운영체제만 어플리케이션이 동작합니다. <br>
/// 
/// #### English (Translation) </br>
/// It is the entry point of the program. </br>
/// The application works only on these three operating systems: `Windows`, `Linux`, and `macOS`. </br>
/// 
fn main() {
    env_logger::init();
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))] {
        run();
    }

    #[allow(unreachable_code)] {
        panic!("❗️❗️❗️ This platform is not supported. ❗️❗️❗️")
    }
}
