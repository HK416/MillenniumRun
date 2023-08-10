use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use winit::{dpi::LogicalSize, window::{Fullscreen, Window}};

#[cfg(target_os = "macos")] 
use winit::platform::macos::WindowExtMacOS;

#[allow(unused_imports)]
use crate::framework::panic_msg;



/// ### 한국어
/// 어플리케이션에서 사용가능한 16:9비율의 해상도 목록입니다. </br>
/// 
/// ### English (machine translation)
/// A list of 16:9 aspect ratio resolutions available in the application. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Resolution {
    W640H360,
    W960H540,
    #[default]
    W1280H720,
    W1440H810,
    W1600H900,
    W1920H1080,
}

impl Resolution {
    /// ### 한국어
    /// 해상도를 가로와 세로의 크기로 반환합니다.
    /// 
    /// ### English (machine translation)
    /// Returns the resolution in horizontal and vertical dimensions.
    /// 
    #[inline]
    pub fn as_logical_size(&self) -> LogicalSize<u32> {
        debug_assert!(R_SIZE.get(&self).is_some(), "Resolution size does not exist. Please add this size. (resolution: {:?})", &self);
        unsafe { *R_SIZE.get(&self).unwrap_unchecked() }
    }
}


lazy_static! {
    /// ### 한국어 
    /// 각 해상도의 크기 목록 입니다. </br>
    /// 
    /// ### English (machine translation) 
    /// Size list for each resolution. </br>
    /// 
    static ref R_SIZE: HashMap<Resolution, LogicalSize<u32>> = HashMap::from([
        (Resolution::W640H360, LogicalSize::new(640, 360)),
        (Resolution::W960H540, LogicalSize::new(960, 540)),
        (Resolution::W1280H720, LogicalSize::new(1280, 720)),
        (Resolution::W1440H810, LogicalSize::new(1440, 810)),
        (Resolution::W1600H900, LogicalSize::new(1600, 900)),
        (Resolution::W1920H1080, LogicalSize::new(1920, 1080)),
    ]);
}


/// ### 한국어 
/// 윈도우 화면 모드 목록입니다. </br>
/// 
/// ### English (machine translation) 
/// List of window screen modes. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenMode {
    #[default]
    Windowed,
    FullScreen,
}

/// #### 한국어
/// 윈도우를 전체화면 모드로 설정합니다.
/// `screen_mode`가 `ScreenMode::FullScreen`일 때 `Windows`, `Linux`의 경우 현재 모니터 정보를 가져와 비디오 모드를 설정합니다. 
/// 반면에 `macOS`의 경우 전용 함수를 사용합니다.
/// 
/// #### English (Translation)
/// Sets the window to full screen mode. 
/// When screen_mode is ScreenMode::FullScreen, in case of `Windows` or `Linux`, get the current monitor information and set the video mode. 
/// On the other hand, for `macOS`, it uses a dedicated function.
/// 
/// <br>
/// 
/// # Panics
/// #### 한국어
/// `screen_mode`가 `ScreenMode::FullScreen`일 때 `Windows`와 `Linux`에서 
/// 현재 모니터 정보나 비디오 모드를 가져올 수 없는 경우 오류 메시지를 화면에 띄우고 프로그램 실행을 중단합니다.
/// 
/// #### English (Translation)
/// If `screen_mode` is `ScreenMode::FullScreen` and Windows and Linux cannot get the current monitor information or video mode, 
/// display an error message and stop running the program.
/// 
pub fn set_fullscreen(
    window: &Window,
    screen_mode: &ScreenMode,
) {
    match screen_mode {
        ScreenMode::Windowed => window.set_fullscreen(None),
        ScreenMode::FullScreen => {
            #[cfg(target_os = "macos")] {
                window.set_simple_fullscreen(true);
            }
            #[cfg(not(target_os = "macos"))] {
                let monitor = match window.current_monitor() {
                    Some(monitor) => monitor,
                    None => panic_msg("Window system error", "Could not find the monitor where the window is currently located!"),
                };
                let video_mode = match monitor.video_modes().next() {
                    Some(mode) => mode,
                    None => panic_msg("Window system error", "The monitor's video mode could not be found!"),
                };
                window.set_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
            }
        }
    }
}
