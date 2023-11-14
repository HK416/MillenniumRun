pub mod entry;
pub mod first_time;
pub mod setup;
pub mod test;

pub mod consts {
    use glam::{Mat4, Vec4};

    pub const PIXEL_PER_METER: f32 = 50.0 / 1.0;
    pub const METER_PER_PIXEL: f32 = 1.0 / PIXEL_PER_METER;
    
    pub const VIEW_TOP: f32 = 4.5 * PIXEL_PER_METER;
    pub const VIEW_LEFT: f32 = -8.0 * PIXEL_PER_METER;
    pub const VIEW_BOTTOM: f32 = -4.5 * PIXEL_PER_METER;
    pub const VIEW_RIGHT: f32 = 8.0 * PIXEL_PER_METER;
    pub const VIEW_NEAR: f32 = 0.0 * PIXEL_PER_METER;
    pub const VIEW_FAR: f32 = 1000.0 * PIXEL_PER_METER;
    
    pub const VIEW_ORTHO: Mat4 = Mat4 {
        x_axis: Vec4 { x: 2.0 / (VIEW_RIGHT - VIEW_LEFT), y: 0.0, z: 0.0, w: 0.0 },
        y_axis: Vec4 { x: 0.0, y: 2.0 / (VIEW_TOP - VIEW_BOTTOM), z: 0.0, w: 0.0 },
        z_axis: Vec4 { x: 0.0, y: 0.0, z: 1.0 / (VIEW_NEAR - VIEW_TOP), w: 0.0 },
        w_axis: Vec4 {
            x: -(VIEW_LEFT + VIEW_RIGHT) / (VIEW_RIGHT - VIEW_LEFT),
            y: -(VIEW_TOP + VIEW_BOTTOM) / (VIEW_TOP - VIEW_BOTTOM),
            z: VIEW_NEAR / (VIEW_NEAR - VIEW_FAR),
            w: 1.0,
        }
    };
}

pub mod path {
    pub const SETTINGS_PATH: &'static str = "user.settings";

    pub const FONT_PATH: &'static str = "fonts/pretendard.ttf";

    pub const FONT_SHADER_PATH: &'static str = "shaders/font.wgsl";
    pub const TEST_SHADER_PATH: &'static str = "shaders/test.wgsl";
    pub const UI_SHADER_PATH: &'static str = "shaders/ui.wgsl";

    pub const CLICK_SOUND_PATH: &'static str = "sounds/click_sound.wav";
    pub const YUZU_TITLE_SOUND_PATH: &'static str = "sounds/yuzu_title.wav";
    pub const ARIS_TITLE_SOUND_PATH: &'static str = "sounds/aris_title.wav";
    pub const MOMOI_TITLE_SOUND_PATH: &'static str = "sounds/momoi_title.wav";
    pub const MIDORI_TITLE_SOUND_PATH: &'static str = "sounds/midori_title.wav";

    pub const BUTTON_TEXTURE_PATH: &'static str = "textures/button.dds";
    pub const LOGO_TEXTURE_PATH: &'static str = "textures/logo.dds";
}
