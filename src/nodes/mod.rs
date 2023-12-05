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
    pub const FONT_MEDIUM_PATH: &'static str = "fonts/nexon_lv2_gothic_medium.ttf";
    pub const FONT_BLOD_PATH: &'static str = "fonts/nexon_lv2_gothic_bold.ttf";

    pub const UI_SHADER_PATH: &'static str = "shaders/ui.wgsl";
    pub const TEXT2D_SHADER_PATH: &'static str = "shaders/text_2d.wgsl";
    pub const TEXT3D_SHADER_PATH: &'static str = "shaders/text_3d.wgsl";
    pub const SPRITE_SHADER_PATH: &'static str = "shaders/sprite.wgsl";

    pub mod sys {
        pub const BUTTON_BASE_TEXTURE_PATH: &'static str = "textures/sys/button_base.png";
        pub const BUTTON_BLUE_TEXTURE_PATH: &'static str = "textures/sys/button_blue.png";
        pub const BUTTON_RED_TEXTURE_PATH: &'static str = "textures/sys/button_red.png";
        pub const BUTTON_START_TEXTURE_PATH: &'static str = "textures/sys/button_start.png";
        pub const BUTTON_SETTING_TEXTURE_PATH: &'static str = "textures/sys/button_setting.png";
        pub const BUTTON_EXIT_TEXTURE_PATH: &'static str = "textures/sys/button_exit.png";
        pub const BUTTON_RETURN_TEXTURE_PATH: &'static str = "textures/sys/button_return.png";

        pub const WINDOW_TEXTURE_PATH: &'static str = "textures/sys/window.png";

        pub const CLICK_SOUND_PATH: &'static str = "sounds/sys/click.ogg";
        pub const CANCEL_SOUND_PATH: &'static str = "sounds/sys/cancel.ogg";
    }

    pub mod intro {
        pub const LOGO_TEXTURE_PATH: &'static str = "textures/intro/logo.png";

        pub const YUZU_SOUND_PATH: &'static str = "sounds/intro/yuzu.ogg";
        pub const ARIS_SOUND_PATH: &'static str = "sounds/intro/aris.ogg";
        pub const MOMOI_SOUND_PATH: &'static str = "sounds/intro/momoi.ogg";
        pub const MIDORI_SOUND_PATH: &'static str = "sounds/intro/midori.ogg";
    }

    pub mod title {
        pub const SOFA_TEXTURE_PATH: &'static str = "textures/title/sofa.png";
        pub const CABINET_TEXTURE_PATH: &'static str = "textures/title/cabinet.png";
        pub const BACKGROUND_PATH: &'static str = "textures/title/background.png";
        pub const YUZU_TEXTURE_PATH: &'static str = "textures/title/yuzu.png";
        pub const ARIS_TEXTURE_PATH: &'static str = "textures/title/aris.png";
        pub const MOMOI_TEXTURE_PATH: &'static str = "textures/title/momoi.png";
        pub const MIDORI_TEXTURE_PATH: &'static str = "textures/title/midori.png";

        pub const BGM_SOUND_PATH: &'static str = "sounds/title/theme64.ogg";
    }
}
