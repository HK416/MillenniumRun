use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use winit::{window::Fullscreen, event_loop::EventLoop, dpi::LogicalSize};


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
    Borderless,
    FullScreen,
}

impl ScreenMode {
    /// ### 한국어
    /// 윈도우 화면 모드를 `winit` 크레이트의 형식으로 반환합니다.
    /// 
    /// ### English (machine translation)
    /// Returns the window screen mode in the `winit` crate format.
    /// 
    #[inline]
    pub fn as_fullscreen<T: 'static>(
        &self, 
        event_loop: &EventLoop<T>
    ) -> Option<Fullscreen> {
        match self {
            ScreenMode::Windowed => None,
            ScreenMode::Borderless => Some(Fullscreen::Borderless(None)),
            ScreenMode::FullScreen => {
                let monitor = event_loop
                    .available_monitors()
                    .next()?;
                Some(Fullscreen::Exclusive(monitor.video_modes().next()?))
            },
        }
    } 
}
