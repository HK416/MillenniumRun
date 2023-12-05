//! #### 한국어 </br>
//! 모든 게임 장면에서 사용하는 에셋의 목록입니다. </br>
//! 
//! #### English (Translation) </br>
//! List of assets used in all game scene. </br>
//! 

use crate::nodes::path;

pub const ASSETS: [&'static str; 7] = [
    path::FONT_BLOD_PATH,
    path::FONT_MEDIUM_PATH,
    path::SETTINGS_PATH,
    path::SPRITE_SHADER_PATH,
    path::TEXT2D_SHADER_PATH,
    path::TEXT3D_SHADER_PATH,
    path::UI_SHADER_PATH,
];
