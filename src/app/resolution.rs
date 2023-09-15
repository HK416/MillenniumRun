use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use winit::{
    window::Window,
    dpi::LogicalSize, 
};



lazy_static! {
    /// #### 한국어 </br>
    /// 각 해상도의 크기 목록 입니다. </br>
    /// 
    /// #### English (machine translation) </br>
    /// Size list for each resolution. </br>
    /// 
    static ref SIZE: HashMap<Resolution, LogicalSize<u32>> = HashMap::from([
        (Resolution::W640H360, (640, 360).into()),
        (Resolution::W960H540, (960, 540).into()),
        (Resolution::W1280H720, (1280, 720).into()),
        (Resolution::W1440H810, (1440, 810).into()),
        (Resolution::W1600H900, (1600, 900).into()),
        (Resolution::W1920H1080, (1920, 1080).into()),
    ]);
}

/// #### 한국어 </br>
/// 어플리케이션에서 사용가능한 16:9비율의 해상도 목록입니다. </br>
/// 
/// #### English (machine translation) </br>
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

impl AsRef<LogicalSize<u32>> for Resolution {
    #[inline]
    fn as_ref(&self) -> &LogicalSize<u32> {
        log::info!("resultion: {:?}", self);
        SIZE.get(self).expect("Unable to get window size for given resolution. Please add the window size for the given resolution")
    }
}



/// #### 한국어 </br>
/// 윈도우 화면 모드 목록입니다. </br>
/// 
/// #### English (machine translation) </br>
/// List of window screen modes. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenMode {
    #[default]
    Windowed,
    FullScreen,
}

impl ScreenMode {
    #[inline]
    pub fn is_fullscreen(&self) -> bool {
        match self {
            Self::Windowed => false,
            Self::FullScreen => true,
        }
    }
}



/// #### 한국어 </br>
/// 윈도우의 화면 모드를 설정합니다. </br>
/// 전체 화면 모드로 설정할 수 없는 경우 창 모드로 설정됩니다. </br>
/// 
/// #### English (Translation)
/// Set the screen mode of the window. </br>
/// If full screen mode cannot be set, it will be set to windowed mode. </br>
/// 
pub fn set_screen_mode(window: &Window, mode: &ScreenMode) {
    log::info!("screen mode: {:?}", mode);
    if mode.is_fullscreen() {
        #[cfg(target_os = "macos")] {
            use winit::platform::macos::WindowExtMacOS;
            window.set_simple_fullscreen(true);
            return;
        }
        #[cfg(not(target_os = "macos"))] {
            use winit::window::Fullscreen;
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            return;
        }
    }
    window.set_fullscreen(None);
}
