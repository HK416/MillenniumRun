pub mod first_time;
pub mod intro;
pub mod setup;
pub mod title;
pub mod in_game;

pub mod consts {
    pub const PIXEL_PER_METER: f32 = 50.0 / 1.0;
}

pub mod path {
    pub const SAVE_PATH: &'static str = "user.sav";
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
    pub const TILE_SPRITE_SHADER_PATH: &'static str = "shaders/tile.wgsl";

    pub const BULLET_SHADER_PATH: &'static str = "shaders/bullet.wgsl";

    // Textures ---------------------------------------------------------------
    pub const LOGO_TEXTURE_PATH: &'static str = "textures/sys/logo.dds";
    pub const DUMMY_TEXTURE_PATH: &'static str = "textures/sys/dummy.dds";

    pub const STAR_TEXTURE_PATH: &'static str = "textures/ui/star.dds";
    pub const HEART_TEXTURE_PATH: &'static str = "textures/ui/heart.dds";
    pub const FINISH_TEXTURE_PATH: &'static str = "textures/ui/finish.dds";
    pub const BUTTON_MEDIUM_TEXTURE_PATH: &'static str = "textures/ui/button_medium.dds";
    pub const BUTTON_WIDE_TEXTURE_PATH: &'static str = "textures/ui/button_wide.dds";
    pub const BUTTON_ETC_TEXTURE_PATH: &'static str = "textures/ui/button_etc.dds";
    pub const BUTTON_RETURN_TEXTURE_PATH: &'static str = "textures/ui/button_return.dds";
    pub const TITLE_BUTTON_START_TEXTURE_PATH: &'static str = "textures/ui/title_button_start.dds";
    pub const TITLE_BUTTON_SETTING_TEXTURE_PATH: &'static str = "textures/ui/title_button_setting.dds";
    pub const TITLE_BUTTON_EXIT_TEXTURE_PATH: &'static str = "textures/ui/title_button_exit.dds";
    pub const WINDOW_RATIO_4_3_TEXTURE_PATH: &'static str = "textures/ui/window_ratio_4_3.dds";
    pub const WINDOW_RATIO_8_1_TEXTURE_PATH: &'static str = "textures/ui/window_ratio_8_1.dds";

    pub const TITLE_BACKGROUND_TEXTURE_PATH: &'static str = "textures/bg/title_background.dds";
    pub const INGAME_BACKGROUND_TEXTURE_PATH: &'static str = "textures/bg/ingame_background.dds";

    pub const DEF_IMG_TEXTURE_PATH: &'static str = "textures/img/default.dds";
    pub const ARIS_IMG_TEXTURE_PATH: &'static str = "textures/img/aris.dds";
    pub const MOMOI_IMG_TEXTURE_PATH: &'static str = "textures/img/momoi.dds";
    pub const MIDORI_IMG_TEXTURE_PATH: &'static str = "textures/img/midori.dds";
    pub const YUZU_IMG_TEXTURE_PATH: &'static str = "textures/img/yuzu.dds";
    pub const YUUKA_IMG_TEXTURE_PATH: &'static str = "textures/img/yuuka.dds";

    pub const ARIS_STANDING_TEXTURE_PATH: &'static str = "textures/character/aris_standing.dds";
    pub const ARIS_PLAYER_TEXTURE_PATH: &'static str = "textures/character/aris_player.dds";

    pub const MOMOI_STANDING_TEXTURE_PATH: &'static str = "textures/character/momoi_standing.dds";
    pub const MOMOI_PLAYER_TEXTURE_PATH: &'static str = "textures/character/momoi_player.dds";

    pub const MIDORI_STANDING_TEXTURE_PATH: &'static str = "textures/character/midori_standing.dds";
    pub const MIDORI_PLAYER_TEXTURE_PATH: &'static str = "textures/character/midori_player.dds";

    pub const YUZU_STANDING_TEXTURE_PATH: &'static str = "textures/character/yuzu_standing.dds";
    pub const YUZU_PLAYER_TEXTURE_PATH: &'static str = "textures/character/yuzu_player.dds";

    pub const YUUKA_ENEMY_TEXTURE_PATH: &'static str = "textures/enemy/yuuka.dds";
    pub const YUUKA_BULLET_TEXTURE_PATH: &'static str = "textures/enemy/yuuka_bullet.dds";

    // Sounds -----------------------------------------------------------------
    pub const CLICK_SOUND_PATH: &'static str = "sounds/effect/click.ogg";
    pub const CANCEL_SOUND_PATH: &'static str = "sounds/effect/cancel.ogg";
    pub const START_SOUND_PATH: &'static str = "sounds/effect/start.ogg";
    pub const PAUSE_SOUND_PATH: &'static str = "sounds/effect/pause.ogg";
    pub const FINISH_SOUND_PATH: &'static str = "sounds/effect/finish.ogg";
    pub const BULLET_FIRE_SOUND_PATH: &'static str = "sounds/effect/bullet_fire.ogg";

    pub const THEME18_SOUND_PATH: &'static str = "sounds/bgm/theme18.ogg";
    pub const THEME19_SOUND_PATH: &'static str = "sounds/bgm/theme19.ogg";
    pub const THEME23_SOUND_PATH: &'static str = "sounds/bgm/theme23.ogg";
    pub const THEME27_SOUND_PATH: &'static str = "sounds/bgm/theme27.ogg";
    pub const THEME30_SOUND_PATH: &'static str = "sounds/bgm/theme30.ogg";
    pub const THEME64_SOUND_PATH: &'static str = "sounds/bgm/theme64.ogg";

    pub const ARIS_TITLE_SOUND_PATH: &'static str = "sounds/aris/aris_title.ogg";
    pub const ARIS_STAGE_START_SOUND_PATH: &'static str = "sounds/aris/aris_stage_start.ogg";
    pub const ARIS_SMILE_0_SOUND_PATH: &'static str = "sounds/aris/aris_smile_0.ogg";
    pub const ARIS_SMILE_1_SOUND_PATH: &'static str = "sounds/aris/aris_smile_1.ogg";
    pub const ARIS_DAMAGE_0_SOUND_PATH: &'static str = "sounds/aris/aris_damage_0.ogg";
    pub const ARIS_DAMAGE_1_SOUND_PATH: &'static str = "sounds/aris/aris_damage_1.ogg";
    pub const ARIS_DAMAGE_2_SOUND_PATH: &'static str = "sounds/aris/aris_damage_2.ogg";

    pub const MOMOI_TITLE_SOUND_PATH: &'static str = "sounds/momoi/momoi_title.ogg";
    pub const MOMOI_STAGE_START_SOUND_PATH: &'static str = "sounds/momoi/momoi_stage_start.ogg";
    pub const MOMOI_SMILE_0_SOUND_PATH: &'static str = "sounds/momoi/momoi_smile_0.ogg";
    pub const MOMOI_SMILE_1_SOUND_PATH: &'static str = "sounds/momoi/momoi_smile_1.ogg";
    pub const MOMOI_DAMAGE_0_SOUND_PATH: &'static str = "sounds/momoi/momoi_damage_0.ogg";
    pub const MOMOI_DAMAGE_1_SOUND_PATH: &'static str = "sounds/momoi/momoi_damage_1.ogg";
    pub const MOMOI_DAMAGE_2_SOUND_PATH: &'static str = "sounds/momoi/momoi_damage_2.ogg";

    pub const MIDORI_TITLE_SOUND_PATH: &'static str = "sounds/midori/midori_title.ogg";
    pub const MIDORI_STAGE_START_SOUND_PATH: &'static str = "sounds/midori/midori_stage_start.ogg";
    pub const MIDORI_SMILE_0_SOUND_PATH: &'static str = "sounds/midori/midori_smile_0.ogg";
    pub const MIDORI_SMILE_1_SOUND_PATH: &'static str = "sounds/midori/midori_smile_1.ogg";
    pub const MIDORI_DAMAGE_0_SOUND_PATH: &'static str = "sounds/midori/midori_damage_0.ogg";
    pub const MIDORI_DAMAGE_1_SOUND_PATH: &'static str = "sounds/midori/midori_damage_1.ogg";
    pub const MIDORI_DAMAGE_2_SOUND_PATH: &'static str = "sounds/midori/midori_damage_2.ogg";

    pub const YUZU_TITLE_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_title.ogg";
    pub const YUZU_STAGE_START_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_stage_start.ogg";
    pub const YUZU_SMILE_0_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_smile_0.ogg";
    pub const YUZU_SMILE_1_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_smile_1.ogg";
    pub const YUZU_DAMAGE_0_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_damage_0.ogg";
    pub const YUZU_DAMAGE_1_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_damage_1.ogg";
    pub const YUZU_DAMAGE_2_SOUND_PATH: &'static str = "sounds/yuzu/yuzu_damage_2.ogg";

    pub const YUUKA_TITLE_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_title.ogg";
    pub const YUUKA_ATTACK0_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_attack_0.ogg";
    pub const YUUKA_ATTACK1_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_attack_1.ogg";
    pub const YUUKA_ATTACK2_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_attack_2.ogg";
    pub const YUUKA_ATTACK3_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_attack_3.ogg";
    pub const YUUKA_VICTORY_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_victory.ogg";
    pub const YUUKA_DEFEAT_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_defeat.ogg";
    pub const YUUKA_HIDDEN_SOUND_PATH: &'static str = "sounds/yuuka/yuuka_hidden.ogg";
}
