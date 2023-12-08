use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use ab_glyph::Font;
use glam::{Vec3, Vec4};

use crate::components::text::section::d2::Section2dBuilder;
use crate::components::ui::objects::UiObjectBuilder;
use crate::components::{
    text::{
        Section,
        brush::TextBrush,
        section::d2::Section2d, 
    }, 
    ui::{
        UserInterface,
        anchor::Anchor,
        brush::UiBrush,
        objects::UiObject, 
    },
};

const WND_ANCHOR_TOP: f32 = 0.5 + 0.4;
const WND_ANCHOR_LEFT: f32 = 0.5 - 0.3;
const WND_ANCHOR_BOTTOM: f32 = 0.5 - 0.4;
const WND_ANCHOR_RIGHT: f32 = 0.5 + 0.3;

const BTN_RATIO_WIDTH: f32 = 0.1;
const BTN_RATIO_HEIGHT: f32 = 0.08888888889;

const BTN_SAVE_CENTER_X: f32 = 0.5 + 0.12;
const BTN_SAVE_CENTER_Y: f32 = 0.5 - 0.3;

const BTN_EXIT_CENTER_X: f32 = 0.5 + 0.24;
const BTN_EXIT_CENTER_Y: f32 = 0.5 - 0.3;

const SCALE: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);



#[derive(Debug, Clone)]
pub struct SettingWindowDesc<'a> {
    pub texts: Vec<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 설정 윈도우의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for setting window. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SettingWindowTags {
    Window = 0,
    SaveButton = 1,
    ExitButton = 2,
}


/// #### 한국어 </br>
/// 설정 윈도우의 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// A collection of setting window. </br>
/// 
#[derive(Debug)]
pub struct SettingWindow(Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>);

#[allow(dead_code, unused_variables)]
impl SettingWindow {
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        descs: HashMap<SettingWindowTags, SettingWindowDesc<'a>>
    ) -> Self {
        // (한국어) 설정 윈도우를 생성합니다.
        // (English Translation) Create a settings window.
        let desc = descs
            .get(&SettingWindowTags::Window)
            .expect("The required variable could not be found.");
        let anchor = Anchor::new(WND_ANCHOR_TOP, WND_ANCHOR_LEFT, WND_ANCHOR_BOTTOM, WND_ANCHOR_RIGHT);
        let window = (
            Arc::new(UiObjectBuilder::new(
                Some("SettingWindow"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![

            ],
        );

        
        // (한국어) `저장` 버튼을 생성합니다.
        // (English Translation) Create a `Store` button.
        let desc = descs
            .get(&SettingWindowTags::SaveButton)
            .expect("The required variable could not be found.");
        let anchor = Anchor::new(
            BTN_SAVE_CENTER_Y + 0.5 * BTN_RATIO_HEIGHT,
            BTN_SAVE_CENTER_X - 0.5 * BTN_RATIO_WIDTH,
            BTN_SAVE_CENTER_Y - 0.5 * BTN_RATIO_HEIGHT,
            BTN_SAVE_CENTER_X + 0.5 * BTN_RATIO_WIDTH,
        );
        let change_and_exit_button = (
            Arc::new(UiObjectBuilder::new(
                Some("StoreButton"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("StoreButton"), 
                    desc.texts[0], 
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_scale(SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)),
            ]
        );

        // (한국어) `취소` 버튼을 생성합니다.
        // (English Translation) Create a `cancel` button.
        let desc = descs
            .get(&SettingWindowTags::ExitButton)
            .expect("The required variable could not be found.");
        let anchor = Anchor::new(
            BTN_EXIT_CENTER_Y + 0.5 * BTN_RATIO_HEIGHT,
            BTN_EXIT_CENTER_X - 0.5 * BTN_RATIO_WIDTH,
            BTN_EXIT_CENTER_Y - 0.5 * BTN_RATIO_HEIGHT,
            BTN_EXIT_CENTER_X + 0.5 * BTN_RATIO_WIDTH,
        );
        let exit_button = (
            Arc::new(UiObjectBuilder::new(
                Some("CancelButton"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("CancelButton"), 
                    desc.texts[0], 
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_scale(SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)),
            ]
        );

        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Self(vec![
            window,
            change_and_exit_button,
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

    pub fn pickable(&self) -> Vec<(SettingWindowTags, (Arc<UiObject>, Vec<Arc<Section2d>>))> {
        vec![
            (
                SettingWindowTags::SaveButton, 
                self.0.get(SettingWindowTags::SaveButton as usize).unwrap().clone()
            ),
            (
                SettingWindowTags::ExitButton, 
                self.0.get(SettingWindowTags::ExitButton as usize).unwrap().clone()
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
