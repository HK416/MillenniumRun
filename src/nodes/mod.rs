pub mod first_time;
pub mod intro;
pub mod setup;
pub mod title;

pub mod consts {
    pub const PIXEL_PER_METER: f32 = 50.0 / 1.0;
    pub const METER_PER_PIXEL: f32 = 1.0 / PIXEL_PER_METER;
}

pub mod path {
    pub const SETTINGS_PATH: &'static str = "user.settings";

    // Fonts ------------------------------------------------------------------
    pub const NEXON_LV2_GOTHIC_BOLD_PATH: &'static str = "fonts/nexon_lv2_gothic_bold.ttf";
    pub const NEXON_LV2_GOTHIC_MEDIUM_PATH: &'static str = "fonts/nexon_lv2_gothic_medium.ttf";
    pub const NEXON_LV2_GOTHIC_PATH: &'static str = "fonts/nexon_lv2_gothic.ttf";

    // Scripts ----------------------------------------------------------------
    pub const KOR_SCRIPTS_PATH: &'static str = "scripts/kor.ron";

    // Shaders ----------------------------------------------------------------
    pub const UI_SHADER_PATH: &'static str = "shaders/ui.wgsl";
    pub const UI_TEXT_SHADER_PATH: &'static str = "shaders/text.wgsl";
    pub const SPRITE_SHADER_PATH: &'static str = "shaders/sprite.wgsl";

    // Textures ---------------------------------------------------------------
    pub const LOGO_TEXTURE_PATH: &'static str = "textures/ui/logo.dds";
    pub const BUTTON_MEDIUM_TEXTURE_PATH: &'static str = "textures/ui/button_medium.dds";
    pub const BUTTON_WIDE_TEXTURE_PATH: &'static str = "textures/ui/button_wide.dds";
    pub const TITLE_BUTTON_START_TEXTURE_PATH: &'static str = "textures/ui/title_button_start.dds";
    pub const TITLE_BUTTON_SETTING_TEXTURE_PATH: &'static str = "textures/ui/title_button_setting.dds";
    pub const TITLE_BUTTON_EXIT_TEXTURE_PATH: &'static str = "textures/ui/title_button_exit.dds";
    pub const TITLE_BUTTON_RETURN_TEXTURE_PATH: &'static str = "textures/ui/title_button_return.dds";
    pub const WINDOW_RATIO_4_3_TEXTURE_PATH: &'static str = "textures/ui/window_ratio_4_3.dds";

    pub const BACKGROUND_TEXTURE_PATH: &'static str = "textures/background.dds";
    pub const ARIS_STANDING_TEXTURE_PATH: &'static str = "textures/aris_standing.dds";
    pub const MOMOI_STANDING_TEXTURE_PATH: &'static str = "textures/momoi_standing.dds";
    pub const MIDORI_STANDING_TEXTURE_PATH: &'static str = "textures/midori_standing.dds";
    pub const YUZU_STANDING_TEXTURE_PATH: &'static str = "textures/yuzu_standing.dds";

    // Sounds -----------------------------------------------------------------
    pub const THEME64_SOUND_PATH: &'static str = "sounds/bgm/theme64.ogg";

    pub const CLICK_SOUND_PATH: &'static str = "sounds/ui/click.ogg";
    pub const CANCEL_SOUND_PATH: &'static str = "sounds/ui/cancel.ogg";
    
    pub const ARIS_TITLE_SOUND_PATH: &'static str = "sounds/aris/aris_title.ogg";

    pub const MOMOI_TITLE_SOUND_PATH: &'static str = "sounds/momoi/momoi_title.ogg";

    pub const MIDORI_TITLE_SOUND_PATH: &'static str = "sounds/midori/midori_title.ogg";

    pub const YUZU_TITLE_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_title.ogg";
}
