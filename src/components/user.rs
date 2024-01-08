use serde::{Serialize, Deserialize};
use winit::{
    window::Window,
    dpi::{
        PhysicalPosition, 
        PhysicalSize, 
        LogicalSize,
    },
};

use crate::{
    game_err,
    components::{sound::Volume, control::Control},
    assets::interface::{AssetDecoder, AssetEncoder},
    system::error::{AppResult, GameError},
};



/// #### 한국어 </br>
/// 애플리케이션 언어 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of application languages. </br>
/// 
#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    #[default]
    Unknown,
    Korean,
}



/// #### 한국어 </br>
/// 애플리케이션 화면 모드 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of application screen modes. </br>
/// 
#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenMode {
    #[default]
    Windowed,
    FullScreen,
}



/// #### 한국어 </br>
/// 애플리케이션 윈도우의 해상도 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of resolutions for application window. </br>
/// 
#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resolution {
    #[default]
    W800H600,
    W1024H768,
    W1152H864,
    W1280H960,
    W1400H1050,
    W1600H1200,
}

impl Resolution {
    #[inline]
    pub fn downgrade(self) -> Option<Self> {
        match self {
            Self::W800H600 => None,
            Self::W1024H768 => Some(Self::W800H600),
            Self::W1152H864 => Some(Self::W1024H768),
            Self::W1280H960 => Some(Self::W1152H864),
            Self::W1400H1050 => Some(Self::W1280H960),
            Self::W1600H1200 => Some(Self::W1400H1050),
        }
    }
}

impl Into<LogicalSize<u32>> for Resolution {
    #[inline]
    fn into(self) -> LogicalSize<u32> {
        match self {
            Self::W800H600 => (800, 600),
            Self::W1024H768 => (1024, 768),
            Self::W1152H864 => (1152, 864),
            Self::W1280H960 => (1280, 960),
            Self::W1400H1050 => (1400, 1050),
            Self::W1600H1200 => (1600, 1200),
        }.into()
    }
}



/// #### 한국어 </br>
/// 애플리케이션 설정을 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains application settings. </br>
/// 
#[repr(C)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Settings {
    pub control: Control, 
    pub language: Language,
    pub screen_mode: ScreenMode,
    pub resolution: Resolution,
    pub background_volume: Volume,
    pub effect_volume: Volume,
    pub voice_volume: Volume,
}

impl Default for Settings {
    #[inline]
    fn default() -> Self {
        Self { 
            control: Control::default(), 
            language: Language::default(), 
            screen_mode: ScreenMode::default(), 
            resolution: Resolution::default(), 
            background_volume: Volume::new(80),
            effect_volume: Volume::new(70),
            voice_volume: Volume::new(70),
        }
    }
}



/// #### 한국어 </br>
/// 애플리케이션 설정의 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder for application settings. </br>
/// 
#[derive(Debug)]
pub struct SettingsDecoder;

impl AssetDecoder for SettingsDecoder {
    type Output = Settings;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        Ok(ron::de::from_bytes(buf)
            .map_err(|err| game_err!(
                "Failed to load asset file",
                "The asset file failed to load for the following reasons: {}",
                err.to_string()
            ))?
        )
    }
}



/// #### 한국어 </br>
/// 애플리케이션 설정의 인코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a encoder for application settings. </br>
/// 
#[derive(Debug)]
pub struct SettingsEncoder;

impl AssetEncoder for SettingsEncoder {
    type Input = Settings;

    #[inline]
    fn encode(&self, val: &Self::Input) -> AppResult<Vec<u8>> {
        use ron::ser::PrettyConfig;
        
        let config = PrettyConfig::new()
            .separate_tuple_members(true)
            .enumerate_arrays(true)
            .struct_names(true);

        Ok(ron::ser::to_string_pretty(val, config)
            .map_err(|err| game_err!(
                "Failed to store asset file",
                "The asset file failed to store for the following reasons: {}",
                err.to_string()
            ))?
            .bytes()
            .collect()
        )
    }
}



/// #### 한국어 </br>
/// 애플리케이션 윈도우 크기를 설정합니다. </br>
/// <b>애플리케이션 윈도우 크기 조정에 실패한 경우 `GameError`를 반환합니다.</b></br>
/// 
/// #### English (Translation) </br>
/// Sets the application window size. </br>
/// <b>Returns `GameError` if application window resizing fails.</b></br>
/// 
#[inline]
pub fn set_window_size(window: &Window, resolution: Resolution) -> AppResult<Resolution> {
    let monitor = window.current_monitor()
        .ok_or_else(|| game_err!(
            "Application window resize failed", 
            "Unable to get information about the monitor where the current application window is located."
        ))?;
    
    let logical_size: LogicalSize<u32> = resolution.into();
    let physical_size: PhysicalSize<u32> = logical_size.to_physical(window.scale_factor());
    if physical_size.width <= monitor.size().width 
    && physical_size.height <= monitor.size().height {
        if window.request_inner_size(physical_size).is_some() {
            Err(game_err!(
                "Application window resize failed",
                "The application window cannot be resized."
            ))
        } else {
            // (한국어) 애플리케이션 윈도우를 화면 중앙에 위치시킵니다.
            // (English Translation) Centers the application window on the screen.
            let monitor = window.current_monitor().unwrap();
            let center_x = monitor.position().x + (monitor.size().width / 2) as i32;
            let center_y = monitor.position().y + (monitor.size().height / 2) as i32;
            window.set_outer_position(PhysicalPosition::new(
                center_x - (physical_size.width / 2) as i32,
                center_y - (physical_size.height / 2) as i32
            ));
            
            Ok(resolution)
        }
    } else {
        if let Some(resolution) = resolution.downgrade() {
            set_window_size(window, resolution)
        } else {
            Err(game_err!(
                "Application window resize failed",
                "The application window cannot be resized."
            ))
        }
    }
}


/// #### 한국어 </br>
/// 애플리케이션 윈도우 화면 모드를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets the application window screen mode. </br>
/// 
#[inline]
pub fn set_screen_mode(window: &Window, screen_mode: ScreenMode) {
    match screen_mode {
        ScreenMode::Windowed => window.set_fullscreen(None),
        ScreenMode::FullScreen => {
            #[cfg(target_os = "macos")] {
                use winit::platform::macos::WindowExtMacOS;
                window.set_simple_fullscreen(true);
            }
            #[cfg(not(target_os = "macos"))] {
                use winit::window::Fullscreen;
                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            }
        },
    }
}
