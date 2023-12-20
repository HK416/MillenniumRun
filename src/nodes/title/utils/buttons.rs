use std::sync::Arc;

use ab_glyph::Font;
use glam::{Vec4, Vec3};

use crate::{
    components::{
        text2d::{
            brush::Text2dBrush, 
            section::{Section2d, Section2dBuilder},
        },
        ui::{
            brush::UiBrush, 
            objects::{UiObject, UiObjectBuilder},
        },
        anchor::Anchor, 
        margin::Margin,
        script::{Script, ScriptTags},
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
pub(super) fn create_menu_buttons<'a, F: Font>(
    font: &'a F, 
    device: &'a wgpu::Device, 
    queue: &'a wgpu::Queue, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: MenuButtonTextureViews<'a>, 
    script: &'a Script, 
    ui_brush: &'a UiBrush, 
    text_brush: &'a Text2dBrush
) -> AppResult<Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>> {
    const ANCHOR_TOP: f32 = 0.4;
    const ANCHOR_LEFT: f32 = 0.5;
    const ANCHOR_BOTTOM: f32 = 0.4;
    const ANCHOR_RIGHT: f32 = 0.5;

    const WIDTH: i32 = 512;
    const HEIGHT: i32 = WIDTH / 8 + (WIDTH / 8 / 4);
    const GAP: i32 = HEIGHT + HEIGHT / 4;

    const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
    const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);
    
    const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);
    const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);

    
    // (한국어) `시작` 버튼을 생성합니다.
    // (English Translation) Create a `start` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(1 * GAP + HEIGHT / 2, -WIDTH / 2, 1 * GAP - HEIGHT / 2, WIDTH / 2);
    let start_button = (
        Arc::new(UiObjectBuilder::new(
            Some("StartButton"),
            tex_sampler,
            texture_views.start_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_translation(UI_TRANSLATION)
        .build(device)),
        vec![
            Arc::new(Section2dBuilder::new(
                Some("StartButton"),
                font,
                script.get(ScriptTags::StartMenuButton)?, 
                text_brush
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(TEXT_COLOR)
            .with_translation(TEXT_TRANSLATION)
            .build(device, queue)),
        ]
    );


    // (한국어) `설정` 버튼을 생성합니다.
    // (English Translation) Create a `setting` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(0 * GAP + HEIGHT / 2, -WIDTH / 2, 0 * GAP - HEIGHT / 2, WIDTH / 2);
    let setting_button = (
        Arc::new(UiObjectBuilder::new(
            Some("SettingButton"),
            tex_sampler,
            texture_views.setting_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_translation(UI_TRANSLATION)
        .build(device)),
        vec![
            Arc::new(Section2dBuilder::new(
                Some("SettingButton"),
                font,
                script.get(ScriptTags::SettingMenuButton)?, 
                text_brush
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(TEXT_COLOR)
            .with_translation(TEXT_TRANSLATION)
            .build(device, queue)),
        ]
    );


    // (한국어) `종료` 버튼을 생성합니다.
    // (English Translation) Create a `exit` button.
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(-1 * GAP + HEIGHT / 2, -WIDTH / 2, -1 * GAP - HEIGHT / 2, WIDTH / 2);
    let exit_button = (
        Arc::new(UiObjectBuilder::new(
            Some("ExitButton"),
            tex_sampler,
            texture_views.exit_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(UI_COLOR)
        .with_translation(UI_TRANSLATION)
        .build(device)),
        vec![
            Arc::new(Section2dBuilder::new(
                Some("ExitButton"),
                font,
                script.get(ScriptTags::ExitMenuButton)?, 
                text_brush
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(TEXT_COLOR)
            .with_translation(TEXT_TRANSLATION)
            .build(device, queue)),
        ]
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
pub(super) fn create_system_buttons<'a, F: Font>(
    font: &'a F, 
    device: &'a wgpu::Device, 
    queue: &'a wgpu::Queue, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: SystemButtonTextureViews<'a>, 
    script: &'a Script, 
    ui_brush: &'a UiBrush, 
    text_brush: &'a Text2dBrush
) -> AppResult<Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>> {
    
    // (한국어) `되돌아가기` 버튼을 생성합니다.
    // (English Translation) Create a `Return` button.
    let anchor = Anchor::new(1.0, 0.0, 1.0, 0.0);
    let margin = Margin::new(-16, 16, -96, 96);
    let color = Vec4::new(1.0, 1.0, 1.0, 0.0);
    let translation = Vec3::new(0.0, 0.0, 0.5);
    let return_button = (
        Arc::new(UiObjectBuilder::new(
            Some("ReturnButton"),
            tex_sampler,
            texture_views.return_btn_texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(color)
        .with_translation(translation)
        .build(device)),
        vec![],
    );


    //-------------------------------------------------------------------------*
    // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
    // (English Translation) Caution: Do not change the order.                 |
    //-------------------------------------------------------------------------*
    return Ok(vec![
        return_button, 
    ])
}
