use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use ab_glyph::Font;
use glam::{Vec4, Vec3};

use crate::components::{
    text::{
        Section,
        brush::TextBrush,
        section::d2::{Section2d, Section2dBuilder},
    },
    ui::{
        UserInterface,
        anchor::Anchor,
        margin::Margin,
        brush::UiBrush,
        objects::{UiObject, UiObjectBuilder},
    },
};

const SCALE: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);



#[derive(Debug, Clone)]
pub struct ExitWindowDesc<'a> {
    pub text: Vec<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 종료 윈도우의 배경 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for exit window background. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExitWindowTags {
    Window = 0,
    Okay = 1,
    Cancel = 2,
}


/// #### 한국어 </br>
/// 종료 윈도우 배경의 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// A collection of exit window backgrounds. </br>
/// 
#[derive(Debug)]
pub struct ExitWindow(Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>);

#[allow(dead_code)]
impl ExitWindow {
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        descs: HashMap<ExitWindowTags, ExitWindowDesc<'a>>
    ) -> Self {
        // (한국어) 메시지 박스 윈도우를 생성합니다.
        // (English Translation) Create a message box window.
        let desc = descs
            .get(&ExitWindowTags::Window)
            .expect("The required variable could not be found.");
        let window = (
            Arc::new(UiObjectBuilder::new(
                Some("MessageBoxWindow"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(144, -192, -144, 192))
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("MessageBoxWindow"),
                    desc.text[0],
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(64, -160, 0, 160))
                .with_scale(SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue))
            ],
        );

        // (한국어) 메시지 박스 `확인` 버튼을 생성합니다.
        // (English Translation) Create a message box `okay` button.
        let desc = descs
            .get(&ExitWindowTags::Okay)
            .expect("The required variable could not be found.");
        let okay_button = (
            Arc::new(UiObjectBuilder::new(
                Some("OkayButton"),
                &tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-60, -128 -32, -64 -60, -32))
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("OkayButton"),
                    desc.text[0],
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(-60, -128 -32, -64 -60, -32))
                .with_scale(SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)),
            ],
        );

        // (한국어) 메시지 박스 `확인` 버튼을 생성합니다.
        // (English Translation) Create a message box `okay` button.
        let desc = descs
            .get(&ExitWindowTags::Cancel)
            .expect("Descriptor not found!");
        let cancel_button = (
            Arc::new(UiObjectBuilder::new(
                Some("CancelButton"),
                &tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-60, 32, -64 -60, 32 + 128))
            .with_scale(SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("CancelButton"),
                    desc.text[0],
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(-60, 32, -64 -60, 32 + 128))
                .with_scale(SCALE)
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
            window,
            okay_button,
            cancel_button,
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
    pub fn pickable(&self) -> Vec<(ExitWindowTags, (Arc<UiObject>, Vec<Arc<Section2d>>))> {
        vec![
            (ExitWindowTags::Okay, self.0.get(ExitWindowTags::Okay as usize).unwrap().clone()),
            (ExitWindowTags::Cancel, self.0.get(ExitWindowTags::Cancel as usize).unwrap().clone()),
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
