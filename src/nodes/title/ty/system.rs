use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use ab_glyph::Font;

use crate::components::{
    text::{
        Section,
        brush::TextBrush,
        section::d2::Section2d,
    },
    ui::{
        UserInterface,
        anchor::Anchor,
        margin::Margin,
        brush::UiBrush,
        objects::{UiObject, UiObjectBuilder},
    },
};



/// #### 한국어 </br>
/// 시스템 버튼의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for system buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemButtonTags {
    ReturnButton = 0,
}


#[derive(Debug, Clone)]
pub struct SystemButtonDesc<'a> {
    pub text: Vec<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 시스템 버튼 모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a system button collection. </br>
/// 
#[derive(Debug)]
pub struct SystemButtons(Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>);

#[allow(dead_code)]
impl SystemButtons {
    #[allow(unused_variables)]
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        descs: HashMap<SystemButtonTags, SystemButtonDesc<'a>>,
    ) -> Self {
        // (한국어) `되돌아가기 버튼`을 생성합니다.
        // (English Translation) Create a `return button`.
        let desc = descs.get(&SystemButtonTags::ReturnButton).expect("Descriptor not found!");
        let anchor = Anchor::new(1.0, 0.0, 1.0, 0.0);
        let margin = Margin::new(-16, 16, -80, 80);
        let return_button = (
            Arc::new(UiObjectBuilder::new(
                Some("ReturnButton"),
                tex_sampler,
                desc.texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_margin(margin)
            .with_color((1.0, 1.0, 1.0, 1.0).into())
            .with_translation((0.0, 0.0, 0.5).into())
            .build(device)),
            vec![],
        );

        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Self(vec![
            return_button,
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
    pub fn pickable(&self) -> Vec<(SystemButtonTags, (Arc<UiObject>, Vec<Arc<Section2d>>))> {
        vec![
            (
                SystemButtonTags::ReturnButton, 
                self.0.get(SystemButtonTags::ReturnButton as usize).unwrap().clone()
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
