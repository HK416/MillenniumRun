//! #### 한국어 </br>
//! `Intro` 게임 장면에서 사용하는 에셋의 목록입니다. </br>
//! 
//! #### English (Translation) </br>
//! List of assets used in `Intro` game scene. </br>
//! 

use crate::nodes::path;

pub const ASSETS: [&'static str; 5] = [
    path::intro::LOGO_TEXTURE_PATH,
    path::intro::YUZU_SOUND_PATH,
    path::intro::ARIS_SOUND_PATH,
    path::intro::MOMOI_SOUND_PATH,
    path::intro::MIDORI_SOUND_PATH,
];
