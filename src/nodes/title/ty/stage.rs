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
        brush::UiBrush,
        objects::{UiObject, UiObjectBuilder},
    }
};

const WND_ANCHOR_TOP: f32 = 0.5 + 0.2666666667;
const WND_ANCHOR_LEFT: f32 = 0.3 - 0.2;
const WND_ANCHOR_BOTTOM: f32 = 0.5 - 0.2666666667;
const WND_ANCHOR_RIGHT: f32 = 0.3 + 0.2;

const BTN_ANCHOR_TOP: f32 = 0.32 + 0.04444444444;
const BTN_ANCHOR_LEFT: f32 = 0.3 - 0.15;
const BTN_ANCHOR_BOTTOM: f32 = 0.32 - 0.04444444444;
const BTN_ANCHOR_RIGHT: f32 = 0.3 + 0.15;

const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);
const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);



/// #### 한국어 </br>
/// 스테이지 윈도우의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for stage window. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageWindowTags {
    Window = 0,
    Enter = 1,
}


#[derive(Debug, Clone)]
pub struct StageWindowDesc<'a> {
    pub text: Vec<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 스테이지 윈도우 요소의 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// A collection of exit window elements. </br>
/// 
#[derive(Debug)]
pub struct StageWindow(Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>);

#[allow(dead_code)]
impl StageWindow {
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        descs: HashMap<StageWindowTags, StageWindowDesc<'a>>
    ) -> Self {
        // (한국어) 메시지 박스 윈도우를 생성합니다.
        // (English Translation) Create a message box window.
        let desc = descs.get(&StageWindowTags::Window).expect("Descriptor not found!");
        let anchor = Anchor::new(WND_ANCHOR_TOP, WND_ANCHOR_LEFT, WND_ANCHOR_BOTTOM, WND_ANCHOR_RIGHT);
        let window = (
            Arc::new(UiObjectBuilder::new(
                Some("MessageBoxWindow"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![]
        );


        // (한국어) `입장` 버튼을 생성합니다.
        // (English Translation) Create a `enter` button.
        let desc = descs.get(&StageWindowTags::Enter).expect("Descriptor not found!");
        let anchor = Anchor::new(BTN_ANCHOR_TOP, BTN_ANCHOR_LEFT, BTN_ANCHOR_BOTTOM, BTN_ANCHOR_RIGHT);
        let enter = (
            Arc::new(UiObjectBuilder::new(
                Some("EnterButton"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device)),
            vec![
                Arc::new(Section2dBuilder::new(
                    Some("EnterButton"), 
                    desc.text[0], 
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
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
            enter,
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
    pub fn pickable(&self) -> Vec<(StageWindowTags, (Arc<UiObject>, Vec<Arc<Section2d>>))> {
        vec![
            (StageWindowTags::Enter, self.0.get(StageWindowTags::Enter as usize).unwrap().clone()),
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
