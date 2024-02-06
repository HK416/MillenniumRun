use ab_glyph::FontArc;
use glam::{Vec4, Vec3};

use crate::{
    components::{
        text::{TextBrush, Text, TextBuilder},
        ui::{UiBrush, UiObject, UiObjectBuilder},
        anchor::Anchor,
        margin::Margin,
        script::{Script, ScriptTags},
    },
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 종료 메시지 박스 버튼의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of exit message box buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExitMessageBox {
    Background = 0,
    Yes = 1,
    No = 2,
}

impl From<usize> for ExitMessageBox {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Background,
            1 => Self::Yes,
            2 => Self::No,
            _ => panic!("index out of range!")
        }
    }
}


/// #### 한국어 </br>
/// 종료 메시지 박스를 생성하는데 사용되는 텍스처 뷰 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A set of texture views used to create the exit message box. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub(super) struct ExitMsgBoxTextureViews<'a> {
    pub window_texture_view: &'a wgpu::TextureView,
    pub yes_btn_texture_view: &'a wgpu::TextureView,
    pub no_btn_texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 종료 메시지 박스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a exit message box. </br>
/// 
pub(super) fn create_exit_message_box<'a>(
    font: &'a FontArc, 
    device: &'a wgpu::Device, 
    queue: &'a wgpu::Queue, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: ExitMsgBoxTextureViews<'a>,
    script: &'a Script, 
    ui_brush: &'a UiBrush, 
    text_brush: &'a TextBrush, 
) -> AppResult<Vec<(UiObject, Text)>> {
    const ANCHOR_TOP: f32 = 0.5;
    const ANCHOR_LEFT: f32 = 0.5;
    const ANCHOR_BOTTOM: f32 = 0.5;
    const ANCHOR_RIGHT: f32 = 0.5;

    const WND_WIDTH: i32 = 400;
    const WND_HEIGHT: i32 = WND_WIDTH / 4 * 3;
    const WND_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.75);
    const WND_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

    const BTN_WIDTH: i32 = 150;
    const BTN_HEIGHT: i32 = BTN_WIDTH / 3;
    const BTN_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);

    const YES_BTN_COLOR: Vec4 = Vec4::new(255.0 / 255.0, 103.0 / 255.0, 105.0 / 255.0, 1.0);
    const NO_BTN_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

    const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);
    const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

    
    // (한국어) 종료 메시지 박스의 윈도우 배경을 생성합니다. 
    // (English Translation) Creates a window background for the exit message box. 
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let wnd_margin = Margin::new(WND_HEIGHT / 2, -WND_WIDTH / 2, -WND_HEIGHT / 2, WND_WIDTH / 2);
    let text_margin = Margin::new(WND_HEIGHT / 5, -WND_WIDTH / 2, 0, WND_WIDTH / 2);
    let background = (
        UiObjectBuilder::new(
            Some("ExitMessageBoxBackground"),
            tex_sampler,
            texture_views.window_texture_view,
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(wnd_margin)
        .with_color(WND_COLOR)
        .with_global_translation(WND_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("ExitMessageBoxBackground"),
            font,
            script.get(ScriptTags::GameExitReconfirmMessage)?,
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(text_margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue),
    );


    // (한국어) `예` 버튼을 생성합니다.
    // (English Translation) Create a `Yes` Button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(
        BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        -BTN_WIDTH / 2 - WND_WIDTH / 5,
        -BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        BTN_WIDTH / 2 - WND_WIDTH / 5
    );
    let yes_button = (
        UiObjectBuilder::new(
            Some("YesButton"),
            tex_sampler,
            texture_views.yes_btn_texture_view,
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(YES_BTN_COLOR)
        .with_global_translation(BTN_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("YesButton"),
            font,
            script.get(ScriptTags::GameExitOkayButton)?,
            text_brush,
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue),
    );


    // (한국어) `아니오` 버튼을 생성합니다.
    // (English Translation) Create a `No` Button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(
        BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        -BTN_WIDTH / 2 + WND_WIDTH / 5,
        -BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        BTN_WIDTH / 2 + WND_WIDTH / 5
    );
    let no_button = (
        UiObjectBuilder::new(
            Some("NoButton"),
            tex_sampler,
            texture_views.no_btn_texture_view,
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(NO_BTN_COLOR)
        .with_global_translation(BTN_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("NoButton"),
            font,
            script.get(ScriptTags::GameExitCancelButton)?,
            text_brush,
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
        background, 
        yes_button, 
        no_button, 
    ]);
}



/// #### 한국어 </br>
/// 설정 윈도우의 요소 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of setting window elements. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SettingWindow {
    Background = 0,
    Store = 1,
    Exit = 2,
}

impl From<usize> for SettingWindow {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Background, 
            1 => Self::Store, 
            2 => Self::Exit, 
            _ => panic!("index out of range!") 
        }
    }
}



/// #### 한국어 </br>
/// 스테이지 윈도우의 요소 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of stage window elements. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageWindow {
    Background = 0,
    Enter = 1,
}

impl From<usize> for StageWindow {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Background,
            1 => Self::Enter,
            _ => panic!("index out of range!")
        }
    }
}


/// #### 한국어 </br>
/// 스테이지 윈도우를 생성하는데 사용되는 텍스처 뷰 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A set of texture views used to create the stage window. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub(super) struct StageWindowTextureView<'a> {
    pub window_texture_view: &'a wgpu::TextureView, 
    pub enter_btn_texture_view: &'a wgpu::TextureView, 
}


/// #### 한국어 </br>
/// 스테이지 윈도우를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a stage window. </br>
/// 
pub(super) fn create_stage_window<'a>(
    font: &'a FontArc, 
    device: &'a wgpu::Device, 
    queue: &'a wgpu::Queue, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: StageWindowTextureView<'a>, 
    script: &'a Script, 
    ui_brush: &'a UiBrush, 
    text_brush: &'a TextBrush
) -> AppResult<(UiObject, (UiObject, Text))> {
    let anchor = Anchor::new(
        1.0 - 0.01, 
        0.5 - 0.25, 
        1.0 - 0.51, 
        0.5 + 0.25
    );
    let margin = Margin::new(0, 0, 0, 0);
    let ui_color = Vec4::new(1.0, 1.0, 1.0, 0.0);
    let ui_translation = Vec3::new(0.0, 0.0, 0.75);
    let background = UiObjectBuilder::new(
        Some("StageWindow"),
        tex_sampler,
        texture_views.window_texture_view,
        ui_brush
    )
    .with_anchor(anchor)
    .with_margin(margin)
    .with_color(ui_color)
    .with_global_translation(ui_translation)
    .build(device);


    let anchor = Anchor::new(
        1.0 - 0.48 + 0.09375, 
        0.5 - 0.2, 
        1.0 - 0.48, 
        0.5 + 0.2
    );
    let margin = Margin::new(0, 0, 0, 0);
    let ui_color = Vec4::new(1.0, 1.0, 1.0, 0.0);
    let ui_translation = Vec3::new(0.0, 0.0, 0.5);
    let text_color = Vec4::new(0.0, 0.0, 0.0, 0.0);
    let text_translation = Vec3::new(0.0, 0.0, 0.25);
    let enter_button = (
        UiObjectBuilder::new(
            Some("ReturnButton"),
            tex_sampler,
            texture_views.enter_btn_texture_view,
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(ui_color)
        .with_global_translation(ui_translation)
        .build(device),
        TextBuilder::new(
            Some("ReturnButton"),
            font, 
            script.get(ScriptTags::TitleStageEnterButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(text_color)
        .with_translation(text_translation)
        .build(device, queue)
    );

    return Ok((
        background, 
        enter_button, 
    ));
}

/// #### 한국어 </br>
/// 설정 창의 배경 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a background interface for the settings window. </br>
/// 
pub(super) fn create_setting_windows(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    window_texture_view: &wgpu::TextureView, 
    sub_window_texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> Vec<UiObject> {
    let background = UiObjectBuilder::new(
        Some("SettingBackground"), 
        tex_sampler, 
        window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(300, -400, -300, 400))
    .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.9))
    .build(device);

    let item0 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(204, -368, 108, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    let item1 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(76, -368, -20, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    let item2 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-52, -368, -204, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    return vec![
        background, 
        item0, 
        item1, 
        item2
    ];
}

/// #### 한국어 </br>
/// 설정 창의 타이틀 텍스트들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates title texts for the settings window. </br>
/// 
pub(super) fn create_setting_window_titles(
    nexon_lv2_gothic_medium: &FontArc, 
    nexon_lv2_gothic_bold: &FontArc, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> AppResult<Vec<Text>> {
    let main_title = TextBuilder::new(
        Some("SettingTitle"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(292, -368, 244, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item0_title = TextBuilder::new(
        Some("SettingItem0Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingLanguageOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(236, -368, 204, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item0_sub_title = TextBuilder::new(
        Some("SettingItem0SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingLanguageOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(204, -368, 172, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item1_title = TextBuilder::new(
        Some("SettingItem1Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingResolutionOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(108, -368, 76, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item1_sub_title = TextBuilder::new(
        Some("SettingItem1SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingResolutionOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(76, -368, 44, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item2_title = TextBuilder::new(
        Some("SettingItem2Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingVolumeOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-20, -368, -52, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item2_sub_title = TextBuilder::new(
        Some("SettingItem2SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingVolumeOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-52, -368, -84, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    return Ok(vec![
        main_title, 
        item0_title, 
        item0_sub_title, 
        item1_title, 
        item1_sub_title, 
        item2_title, 
        item2_sub_title, 
    ]);
}
