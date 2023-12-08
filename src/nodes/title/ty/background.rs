use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use glam::{Vec4, Vec3, Vec2};

use crate::{
    components::sprite::{
        Sprite, 
        brush::SpriteBrush,
        objects::{SpriteObject, SpriteBuilder},
    },
    nodes::consts::PIXEL_PER_METER,
};



/// #### 한국어 </br>
/// 배경의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for background. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundTags {
    Background = 0,
    Cabinet = 1,
    Sofa = 2,
}


#[derive(Debug, Clone)]
pub struct BackgroundDesc<'a> {
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 배경 모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a background collection. </br>
/// 
#[derive(Debug)]
pub struct Backgrounds(Vec<Arc<SpriteObject>>);

#[allow(dead_code)]
impl Backgrounds {
    pub fn new<'a>(
        device: &wgpu::Device,
        tex_sampler: &wgpu::Sampler,
        sprite_brush: &SpriteBrush,
        descs: HashMap<BackgroundTags, BackgroundDesc<'a>>,
    ) -> Self {
        const SPRITE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);

        // (한국어) `배경`을 생성합니다.
        // (English Translation) Create a `background`.
        const BACKGROUND_POSITION: Vec3 = Vec3::new(0.0 * PIXEL_PER_METER, 0.0 * PIXEL_PER_METER, -10.0 * PIXEL_PER_METER);
        const BACKGROUND_SIZE: Vec2 = Vec2::new(18.0 * PIXEL_PER_METER, 18.0 * PIXEL_PER_METER);
        let desc = descs.get(&BackgroundTags::Background).expect("Descriptor not found!");
        let background = Arc::new(SpriteBuilder::new(
            Some("Background"),
            tex_sampler,
            desc.texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(BACKGROUND_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(BACKGROUND_POSITION)
        .build(device));


        // (한국어) `캐비넷`을 생성합니다.
        // (English Translation) Create a `cabinet`.
        const CABINET_POSITION: Vec3 = Vec3::new(4.0 * PIXEL_PER_METER, 4.0 * PIXEL_PER_METER, -5.0 * PIXEL_PER_METER);
        const CABINET_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 4.0 * PIXEL_PER_METER);
        let desc = descs.get(&BackgroundTags::Cabinet).expect("Descriptor not found!");
        let cabinet = Arc::new(SpriteBuilder::new(
            Some("Cabinet"),
            tex_sampler,
            desc.texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(CABINET_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(CABINET_POSITION)
        .build(device));


        // (한국어) `소파`를 생성합니다.
        // (English Translation) Create a `sofa`.
        const SOFA_POSITION: Vec3 = Vec3::new(0.0 * PIXEL_PER_METER, 2.5 * PIXEL_PER_METER, -3.0 * PIXEL_PER_METER);
        const SOFA_SIZE: Vec2 = Vec2::new(4.5 * PIXEL_PER_METER, 2.25 * PIXEL_PER_METER);
        let desc = descs.get(&BackgroundTags::Sofa).expect("Descriptor not found!");
        let sofa = Arc::new(SpriteBuilder::new(
            Some("Sofa"),
            tex_sampler,
            desc.texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(SOFA_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(SOFA_POSITION)
        .build(device));


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Self(vec![
            background,
            cabinet,
            sofa,
        ])
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&Arc<SpriteObject>> {
        self.0.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Arc<SpriteObject>> {
        self.0.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Arc<SpriteObject>> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, Arc<SpriteObject>> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn sprites(&self) -> Vec<&dyn Sprite> {
        self.0.iter()
        .map(|it| it.as_ref() as &dyn Sprite)
        .collect()
    }

    #[inline]
    pub fn draw<'pass>(
        &'pass self,
        sprite_brush: &'pass SpriteBrush,
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        sprite_brush.draw_texture_blended(rpass, self.sprites().into_iter())
    }
}
