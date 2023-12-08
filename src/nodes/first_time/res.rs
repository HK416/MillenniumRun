//! #### 한국어 </br>
//! `FirstTimeSetup` 게임 장면에서 사용하는 에셋의 목록입니다. </br>
//! 
//! #### English (Translation) </br>
//! List of assets used in `FirstTimeSetup` game scene. </br>
//! 

use crate::nodes::path;

pub const ASSETS: [&'static str; 2] = [
    path::sys::CLICK_SOUND_PATH,
    path::sys::BUTTON_WIDE_TEXTURE_PATH,
];
