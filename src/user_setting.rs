use serde::{Serialize, Deserialize};
use crate::{
    locale::Locale,
    resolution::{Resolution, ScreenMode},
};


/// ### 한국어 
/// 현재 사용자의 어플리케이션 설정입니다. </br>
/// 어플리케이션이 윈도우를 표시하기 전에 어플리케이션 사용자 설정을 읽고, 어플리케이션 사용자 설정에 맞춰 윈도우를 생성합니다. </br>
/// 만약 어플리케이션 사용자 설정이 없는 경우(처음 시작하는 경우) 기본 어플리케이션 사용자 설정을 사용합니다. </br>
/// 
/// ### English (machine translation)
/// Current user's application settings. </br>
/// Before an application displays a window, it reads the application user settings and creates a window according to the application user settings. </br>
/// If there is no application user setting (when starting for the first time), the default application user setting is used. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppUserSetting {
    pub locale: Locale,
    pub resolution: Resolution,
    pub screen_mode: ScreenMode,
}

impl AppUserSetting {
    pub const ASSETS_PATH: &'static str = "user.settings";
}
