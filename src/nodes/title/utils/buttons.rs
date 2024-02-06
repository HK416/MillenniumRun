use std::collections::HashMap;

use ab_glyph::FontArc;
use glam::{Vec4, Vec3};

use crate::{
    components::{
        text::{TextBrush, Text, TextBuilder},
        ui::{UiBrush, UiObject, UiObjectBuilder},
        anchor::Anchor, margin::Margin,
        script::{Script, ScriptTags},
        user::{Settings, Language, Resolution},
    },
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 메뉴 버튼의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of menu buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuButtons {
    Start = 0,
    Setting = 1,
    Exit = 2,
}

impl From<usize> for MenuButtons {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Start,
            1 => Self::Setting,
            2 => Self::Exit,
            _ => panic!("index out of range!")
        }
    }
}


/// #### 한국어 </br>
/// 메뉴 버튼들을 생성하는데 사용되는 텍스처 뷰 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A set of texture views used to create the menu buttons. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub(super) struct MenuButtonTextureViews<'a> {
    pub start_btn_texture_view: &'a wgpu::TextureView, 
    pub setting_btn_texture_view: &'a wgpu::TextureView, 
    pub exit_btn_texture_view: &'a wgpu::TextureView, 
}


/// #### 한국어 </br>
/// 메뉴 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a menu buttons. </br>
/// 
pub(super) fn create_menu_buttons<'a>(
    font: &'a FontArc, 
    device: &'a wgpu::Device, 
    queue: &'a wgpu::Queue, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: MenuButtonTextureViews<'a>, 
    script: &'a Script, 
    ui_brush: &'a UiBrush, 
    text_brush: &'a TextBrush
) -> AppResult<Vec<(UiObject, Text)>> {
    const ANCHOR_TOP: f32 = 0.4;
    const ANCHOR_LEFT: f32 = 0.5;
    const ANCHOR_BOTTOM: f32 = 0.4;
    const ANCHOR_RIGHT: f32 = 0.5;

    const WIDTH: i32 = 512;
    const HEIGHT: i32 = WIDTH / 8 + (WIDTH / 8 / 4);
    const GAP: i32 = HEIGHT + HEIGHT / 4;

    const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
    const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    
    const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);
    const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

    
    // (한국어) `시작` 버튼을 생성합니다.
    // (English Translation) Create a `start` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(1 * GAP + HEIGHT / 2, -WIDTH / 2, 1 * GAP - HEIGHT / 2, WIDTH / 2);
    let start_button = (UiObjectBuilder::new(
            Some("StartButton"),
            tex_sampler,
            texture_views.start_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_global_translation(UI_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("StartButton"),
            font,
            script.get(ScriptTags::TitleStartButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue),
    );


    // (한국어) `설정` 버튼을 생성합니다.
    // (English Translation) Create a `setting` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(0 * GAP + HEIGHT / 2, -WIDTH / 2, 0 * GAP - HEIGHT / 2, WIDTH / 2);
    let setting_button = (
        UiObjectBuilder::new(
            Some("SettingButton"),
            tex_sampler,
            texture_views.setting_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_global_translation(UI_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("SettingButton"),
            font,
            script.get(ScriptTags::TitleSettingButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue),
    );


    // (한국어) `종료` 버튼을 생성합니다.
    // (English Translation) Create a `exit` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(-1 * GAP + HEIGHT / 2, -WIDTH / 2, -1 * GAP - HEIGHT / 2, WIDTH / 2);
    let exit_button = (
        UiObjectBuilder::new(
            Some("ExitButton"),
            tex_sampler,
            texture_views.exit_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_global_translation(UI_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("ExitButton"),
            font,
            script.get(ScriptTags::TitleExitButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue),
    );
    
    //-------------------------------------------------------------------------*
    // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
    // (English Translation) Caution: Do not change the order.                 |
    //-------------------------------------------------------------------------*
    return Ok(vec![
        start_button,
        setting_button,
        exit_button,
    ]);
}



/// #### 한국어 </br>
/// 시스템 버튼의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of system buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemButtons {
    Return = 0,
}

impl From<usize> for SystemButtons {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Return,
            _ => panic!("index out of range!")
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub(super) struct SystemButtonTextureViews<'a> {
    pub return_btn_texture_view: &'a wgpu::TextureView, 
}


/// #### 한국어 </br>
/// 시스템 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create system buttons. </br>
/// 
#[allow(unused_variables)]
pub(super) fn create_system_buttons<'a>(
    device: &'a wgpu::Device, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: SystemButtonTextureViews<'a>, 
    ui_brush: &'a UiBrush, 
) -> UiObject {
    
    // (한국어) `되돌아가기` 버튼을 생성합니다.
    // (English Translation) Create a `Return` button.
    let anchor = Anchor::new(1.0, 0.0, 1.0, 0.0);
    let margin = Margin::new(-16, 16, -96, 96);
    let color = Vec4::new(1.0, 1.0, 1.0, 0.0);
    let translation = Vec3::new(0.0, 0.0, 0.5);
    UiObjectBuilder::new(
        Some("ReturnButton"),
        tex_sampler,
        texture_views.return_btn_texture_view, 
        ui_brush
    )
    .with_anchor(anchor)
    .with_margin(margin)
    .with_color(color)
    .with_global_translation(translation)
    .build(device)
}

/// #### 한국어 </br>
/// 설정 창의 언어 선택 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create language selection buttons in the setting window. </br>
/// 
pub(super) fn create_setting_languages(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> HashMap<Language, (UiObject, Text)> {
    const TOP: i32 = 164;
    const LEFT: i32 = -344;
    const HEIGHT: i32 = 36;
    const WIDTH: i32 = HEIGHT * 3;
    const GAP: i32 = 8;

    let mut left = LEFT;
    let mut languages = HashMap::new();
    const LANGUAGES: [(Language, &'static str); 1] = [
        (Language::Korean, "한국어"), 
    ];

    for (language, text) in LANGUAGES {
        languages.insert(
            language, 
            (
                UiObjectBuilder::new(
                    Some(&format!("{}_Button", text)), 
                    tex_sampler, 
                    texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDTH))
                .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
                .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
                .build(device), 
                TextBuilder::new(
                    Some(&format!("{}_ButtonText", text)), 
                    font, 
                    text, 
                    text_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDTH))
                .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.4))
                .build(device, queue)
            )
        );

        left += GAP + WIDTH;
    }

    return languages;
}

/// #### 한국어 </br>
/// 설정 창의 해상도 선택 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create resolution selection buttons in the setting window. </br>
/// 
pub(super) fn create_setting_resolutions(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> HashMap<Resolution, (UiObject, Text)> {
    const TOP: i32 = 36;
    const LEFT: i32 = -344;
    const HEIGHT: i32 = 36;
    const WIDHT: i32 = HEIGHT * 3; 
    const GAP: i32 = 8;

    let mut left = LEFT;
    let mut resolutions = HashMap::new();
    const RESOLUTIONS: [(Resolution, &'static str); 6] = [
        (Resolution::W800H600, "800x600"),
        (Resolution::W1024H768, "1024x768"), 
        (Resolution::W1152H864, "1152x864"), 
        (Resolution::W1280H960, "1280x960"), 
        (Resolution::W1400H1050, "1400x1050"), 
        (Resolution::W1600H1200, "1600x1200"), 
    ];

    for (resolution, text) in RESOLUTIONS {
        resolutions.insert(
            resolution, 
            (
                UiObjectBuilder::new(
                    Some(&format!("{}_Button", text)), 
                    tex_sampler, 
                    texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDHT))
                .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
                .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
                .build(device),
                TextBuilder::new(
                    Some(&format!("{}_ButtonText", text)), 
                    font, 
                    text, 
                    text_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDHT))
                .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.4))
                .build(device, queue)
            )
        );

        left += GAP + WIDHT;
    }

    return resolutions;
}

/// #### 한국어 </br>
/// 돌아가기 버튼을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a return button. </br>
/// 
#[inline]
pub(super) fn create_setting_return_button(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<(UiObject, Text)> {
    return Ok((
        UiObjectBuilder::new(
            Some("SettingReturnButton"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-220, 224, -268, 368))
        .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
        .build(device), 
        TextBuilder::new(
            Some("SettingReturnButtonText"), 
            font, 
            script.get(ScriptTags::SettingReturnButton)?, 
            text_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-220, 224, -268, 368))
        .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
        .with_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_translation(Vec3::new(0.0, 0.0, 0.4))
        .build(device, queue)
    ))
}

/// #### 한국어 </br>
/// 사용자가 설정 할 수 있는 음향 옵션 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of sound options that the user can set. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeOptions {
    Background, 
    Effect, 
    Voice, 
}

pub const SETTING_VOLUME_RANGE_MAX: i32 = 272;
pub const SETTING_VOLUME_RANGE_MIN: i32 = -240;
pub const VOLUME_BAR_WIDTH: i32 = 8;


/// #### 한국어 </br>
/// 설정 창 볼륨 조절 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a settings window volume control interface. </br>
/// 
pub(super) fn create_setting_volume_background(
    font: &FontArc, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<HashMap<VolumeOptions, (UiObject, Text)>> {
    let mut backgrounds = HashMap::new();
    backgrounds.insert(
        VolumeOptions::Background, 
        (
            UiObjectBuilder::new(
                Some("BackgroundVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-96, SETTING_VOLUME_RANGE_MIN, -104, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("BackgroundVolumeText"), 
                font, 
                script.get(ScriptTags::BackgroundVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-84, -368, -116, -240))
            .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
            .with_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.4))
            .build(device, queue)
        )
    );

    backgrounds.insert(
        VolumeOptions::Effect, 
        (
            UiObjectBuilder::new(
                Some("EffectVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-128, SETTING_VOLUME_RANGE_MIN, -136, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("EffectVolumeText"), 
                font, 
                script.get(ScriptTags::EffectVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-116, -368, -148, -240))
            .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
            .with_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.4))
            .build(device, queue)
        )
    );

    backgrounds.insert(
        VolumeOptions::Voice, 
        (
            UiObjectBuilder::new(
                Some("VoiceVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-160, SETTING_VOLUME_RANGE_MIN, -168, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("VoiceVolumeText"), 
                font, 
                script.get(ScriptTags::VoiceVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-148, -368, -180, -240))
            .build(device, queue)
        )
    );

    return Ok(backgrounds);
}

/// #### 한국어 </br>
/// 설정 창의 볼륨 조절 막대기를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a volume control bar in the setting window. </br>
/// 
pub(super) fn create_setting_volume_bar(
    settings: &Settings, 
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> HashMap<VolumeOptions, UiObject> {
    const RANGE: i32 = SETTING_VOLUME_RANGE_MAX - SETTING_VOLUME_RANGE_MIN;
    let mut bar = HashMap::new();

    let delta = RANGE as f32 * settings.background_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Background, 
        UiObjectBuilder::new(
            Some("BackgroundVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-90, pos - VOLUME_BAR_WIDTH / 2, -110, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    let delta = RANGE as f32 * settings.effect_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Effect, 
        UiObjectBuilder::new(
            Some("EffectVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-122, pos - VOLUME_BAR_WIDTH / 2, -142, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    let delta = RANGE as f32 * settings.voice_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Voice, 
        UiObjectBuilder::new(
            Some("VoiceVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-154, pos - VOLUME_BAR_WIDTH / 2, -174, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    return bar;
}
