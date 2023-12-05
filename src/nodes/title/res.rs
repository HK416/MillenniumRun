//! #### 한국어 </br>
//! `Title` 게임 장면에서 사용하는 에셋의 목록입니다. </br>
//! 
//! #### English (Translation) </br>
//! List of assets used in `Title` game scene. </br>
//! 

use crate::nodes::path;

pub const ASSETS: [&'static str; 17] = [
    path::sys::BUTTON_RETURN_TEXTURE_PATH,
    path::sys::BUTTON_START_TEXTURE_PATH,
    path::sys::BUTTON_SETTING_TEXTURE_PATH,
    path::sys::BUTTON_EXIT_TEXTURE_PATH,
    path::sys::BUTTON_BLUE_TEXTURE_PATH,
    path::sys::BUTTON_RED_TEXTURE_PATH,
    path::sys::CLICK_SOUND_PATH,
    path::sys::CANCEL_SOUND_PATH,
    path::sys::WINDOW_TEXTURE_PATH,
    path::title::BACKGROUND_PATH,
    path::title::CABINET_TEXTURE_PATH,
    path::title::SOFA_TEXTURE_PATH,
    path::title::YUZU_TEXTURE_PATH,
    path::title::ARIS_TEXTURE_PATH,
    path::title::MOMOI_TEXTURE_PATH,
    path::title::MIDORI_TEXTURE_PATH,
    path::title::BGM_SOUND_PATH,
];
