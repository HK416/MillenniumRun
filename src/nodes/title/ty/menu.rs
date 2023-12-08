use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use ab_glyph::Font;
use glam::{Vec4, Vec3};

use crate::components::{
    ui::{
        UserInterface,
        anchor::Anchor,
        margin::Margin,
        brush::UiBrush,
        objects::{UiObject, UiObjectBuilder},
    }, 
    text::{
        Section,
        brush::TextBrush,
        section::d2::{Section2d, Section2dBuilder},
    },
};


const ANCHOR_TOP: f32 = 0.4;
const ANCHOR_LEFT: f32 = 0.5;
const ANCHOR_BOTTOM: f32 = 0.4;
const ANCHOR_RIGHT: f32 = 0.5;

const BTN_TOP: i32 = 32;
const BTN_LEFT: i32 = -192;
const BTN_BOTTOM: i32 = -32;
const BTN_RIGHT: i32 = 192;
const BTN_GAP: i32 = 74;

const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);



/// #### 한국어 </br>
/// 메뉴 버튼의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for menu buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuButtonTags {
    StartButton,
    SettingButton,
    ExitButton,
}


#[derive(Debug, Clone)]
pub struct MenuButtonDesc<'a> {
    pub text: Vec<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 메뉴 버튼모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a menu button collection. </br>
#[derive(Debug)]
pub struct MenuButtons(Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>);

#[allow(dead_code)]
impl MenuButtons {
    /// #### 한국어 </br>
    /// 메뉴 버튼들을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create menu buttons. </br>
    /// 
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        descs: HashMap<MenuButtonTags, MenuButtonDesc<'a>>,
    ) -> Self {
        // (한국어) `시작` 버튼을 생성합니다.
        // (English Translation) Create a `start` button.
        let desc = descs.get(&MenuButtonTags::StartButton).expect("Descriptor not found!");
        let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
        let margin = Margin::new(BTN_TOP + 1 * BTN_GAP, BTN_LEFT, BTN_BOTTOM + 1 * BTN_GAP, BTN_RIGHT);
        let start_button = (
            Arc::new(UiObjectBuilder::new(
                Some("StartButton"), 
                tex_sampler, 
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("StartButton"), 
                    desc.text[0],
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_margin(margin)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue))
            ],
        );


        // (한국어) `게임 설정` 버튼을 생성합니다.
        // (English Translation) Create a `setting` button.
        let desc = descs.get(&MenuButtonTags::SettingButton).expect("Descriptor not found!");
        let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
        let margin = Margin::new(BTN_TOP + 0 * BTN_GAP, BTN_LEFT, BTN_BOTTOM + 0 * BTN_GAP, BTN_RIGHT);
        let setting_button = (
            Arc::new(UiObjectBuilder::new(
                Some("SettingButton"), 
                &tex_sampler, 
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("SettingButton"), 
                    desc.text[0],
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_margin(margin)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)),
            ],
        );


        // (한국어) `게임 종료` 버튼을 생성합니다.
        // (English Translation) Create a `exit` button.
        let desc = descs.get(&MenuButtonTags::ExitButton).expect("Descriptor not found!");
        let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
        let margin = Margin::new(BTN_TOP - 1 * BTN_GAP, BTN_LEFT, BTN_BOTTOM - 1 * BTN_GAP, BTN_RIGHT);
        let exit_button = (
            Arc::new(UiObjectBuilder::new(
                Some("ExitButton"), 
                &tex_sampler, 
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("ExitButton"), 
                    desc.text[0],
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_margin(margin)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)),
            ],
        );


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Self(vec![
            start_button,
            setting_button,
            exit_button,
        ])
    } 

    #[inline]
    pub fn get(&self, index: usize) -> Option<&(Arc<UiObject>, Vec<Arc<Section2d>>)> {
        self.0.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut (Arc<UiObject>, Vec<Arc<Section2d>>)> {
        self.0.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, (Arc<UiObject>, Vec<Arc<Section2d>>)> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, (Arc<UiObject>, Vec<Arc<Section2d>>)> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn pickable(&self) -> Vec<(MenuButtonTags, (Arc<UiObject>, Vec<Arc<Section2d>>))> {
        vec![
            (
                MenuButtonTags::StartButton, 
                self.0.get(MenuButtonTags::StartButton as usize).unwrap().clone()
            ),
            (
                MenuButtonTags::SettingButton, 
                self.0.get(MenuButtonTags::SettingButton as usize).unwrap().clone()
            ),
            (
                MenuButtonTags::ExitButton, 
                self.0.get(MenuButtonTags::ExitButton as usize).unwrap().clone()
            ),
        ]
    }

    #[inline]
    pub fn interfaces(&self) -> Vec<&dyn UserInterface> {
        self.0.iter()
        .map(|(ui, _)| ui.as_ref() as &dyn UserInterface)
        .collect()
    }

    #[inline]
    pub fn sections(&self) -> Vec<&dyn Section> {
        self.0.iter()
        .map(|(_, texts)| texts)
        .flatten()
        .map(|it| it.as_ref() as &dyn Section)
        .collect()
    }

    #[inline]
    pub fn draw<'pass>(
        &'pass self, 
        ui_brush: &'pass UiBrush,
        text_brush: &'pass TextBrush,
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        ui_brush.draw(rpass, self.interfaces().into_iter());
        text_brush.draw_2d(rpass, self.sections().into_iter());
        
    }
}
