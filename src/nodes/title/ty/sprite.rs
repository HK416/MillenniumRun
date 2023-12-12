use std::sync::Arc;
use std::slice::{Iter, IterMut};
use std::collections::HashMap;

use glam::{Mat4, Vec4, Vec3, Vec2};

use crate::{
    components::{
        collider2d::{
            Collider2d,
            shape::AABB,
        },
        sprite::{
            Sprite,
            brush::SpriteBrush,
            objects::{
                InstanceData, 
                SpriteObject, 
                SpriteBuilder
            },
        },
    },
    nodes::consts::PIXEL_PER_METER,
};



/// #### 한국어 </br>
/// 스프라이트의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for sprites. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpriteButtonTags {
    Yuzu = 0,
    Aris = 1,
    Momoi = 2,
    Midori = 3,
}


#[derive(Debug, Clone)]
pub struct SpriteButtonDesc<'a> {
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 스프라이트 버튼 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a sprite button collection. </br>
/// 
#[derive(Debug)]
pub struct SpriteButtons(Vec<Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>>);

#[allow(dead_code)]
impl SpriteButtons {
    /// #### 한국어 </br>
    /// 스프라이트 버튼들을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create sprite buttons. </br>
    /// 
    pub fn new<'a>(
        device: &wgpu::Device,
        tex_sampler: &wgpu::Sampler,
        sprite_brush: &SpriteBrush,
        descs: HashMap<SpriteButtonTags, SpriteButtonDesc<'a>>,
    ) -> Self {
        const SPRITE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

        // (한국어) `Yuzu` 스프라이트를 생성합니다.
        // (English Translation) Create a `Yuzu` sprite.
        const YUZU_POSITION: Vec3 = Vec3::new(3.2 * PIXEL_PER_METER, 3.7 * PIXEL_PER_METER, -4.0 * PIXEL_PER_METER);
        const YUZU_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 2.0 * PIXEL_PER_METER);
        let desc = descs.get(&SpriteButtonTags::Yuzu).expect("Descriptor not found!");
        let instances = [InstanceData { transform: Mat4::from_translation(YUZU_POSITION).into() }];
        let yuzu_button = Arc::new((
            SpriteBuilder::new(
                Some("Yuzu"),
                tex_sampler,
                desc.texture_view,
                sprite_brush.ref_buffer_layout(),
                sprite_brush.ref_texture_layout()
            )
            .with_size(YUZU_SIZE)
            .with_color(SPRITE_COLOR)
            .build(device, instances),
            Box::new(AABB {
                x: YUZU_POSITION.x,
                y: YUZU_POSITION.y,
                width: YUZU_SIZE.x,
                height: YUZU_SIZE.y
            }) as _
        ));


        // (한국어) `Aris` 스프라이트를 생성합니다.
        // (English Translation) Create a `Aris` sprite.
        const ARIS_POSITION: Vec3 = Vec3::new(0.0 * PIXEL_PER_METER, 2.7 * PIXEL_PER_METER, -2.0 * PIXEL_PER_METER);
        const ARIS_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 2.0 * PIXEL_PER_METER);
        let desc = descs.get(&SpriteButtonTags::Aris).expect("Descriptor not found!");
        let instances = [InstanceData { transform: Mat4::from_translation(ARIS_POSITION).into() }];
        let aris_button = Arc::new((
            SpriteBuilder::new(
                Some("Aris"), 
                tex_sampler, 
                desc.texture_view,
                sprite_brush.ref_buffer_layout(),
                sprite_brush.ref_texture_layout()
            )
            .with_size(ARIS_SIZE)
            .with_color(SPRITE_COLOR)
            .build(device, instances),
            Box::new(AABB {
                x: ARIS_POSITION.x,
                y: ARIS_POSITION.y,
                width: ARIS_SIZE.x,
                height: ARIS_SIZE.y
            }) as _
        ));

        
        // (한국어) `Momoi` 스프라이트를 생성합니다.
        // (English Translation) Create a `Momoi` sprite.
        const MOMOI_POSITION: Vec3 = Vec3::new(-3.0 * PIXEL_PER_METER, 1.5 * PIXEL_PER_METER, -1.0 * PIXEL_PER_METER);
        const MOMOI_SIZE: Vec2 = Vec2::new(2.3 * PIXEL_PER_METER, 2.3 * PIXEL_PER_METER);
        let desc = descs.get(&SpriteButtonTags::Momoi).expect("Descriptor not found!");
        let instances = [InstanceData { transform: Mat4::from_translation(MOMOI_POSITION).into() }];
        let momoi_button = Arc::new((
            SpriteBuilder::new(
                Some("Momoi"), 
                tex_sampler, 
                desc.texture_view,
                sprite_brush.ref_buffer_layout(),
                sprite_brush.ref_texture_layout()
            )
            .with_size(MOMOI_SIZE)
            .with_color(SPRITE_COLOR)
            .build(device, instances),
            Box::new(AABB {
                x: MOMOI_POSITION.x,
                y: MOMOI_POSITION.y,
                width: MOMOI_SIZE.x,
                height: MOMOI_SIZE.y
            }) as _
        ));


        // (한국어) `Midori` 스프라이트를 생성합니다.
        // (English Translation) Create a `Midori` sprite.
        const MIDORI_POSITION: Vec3 = Vec3::new(3.0 * PIXEL_PER_METER, 1.5 * PIXEL_PER_METER, -1.0 * PIXEL_PER_METER);
        const MIDORI_SIZE: Vec2 = Vec2::new(2.3 * PIXEL_PER_METER, 2.3 * PIXEL_PER_METER);
        let desc = descs.get(&SpriteButtonTags::Midori).expect("Descriptor not found!");
        let instances = [InstanceData { transform: Mat4::from_translation(MIDORI_POSITION).into() }];
        let midori_button = Arc::new((
            SpriteBuilder::new(
                Some("Midori"), 
                tex_sampler, 
                desc.texture_view,
                sprite_brush.ref_buffer_layout(),
                sprite_brush.ref_texture_layout()
            )
            .with_size(MIDORI_SIZE)
            .with_color(SPRITE_COLOR)
            .build(device, instances),
            Box::new(AABB {
                x: MIDORI_POSITION.x,
                y: MIDORI_POSITION.y,
                width: MIDORI_SIZE.x,
                height: MIDORI_SIZE.y
            }) as _
        ));


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Self(vec![
            yuzu_button,
            aris_button,
            momoi_button,
            midori_button,
        ])
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>> {
        self.0.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>> {
        self.0.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn pickable<'a>(&'a self) -> Vec<(SpriteButtonTags, Arc<(SpriteObject, Box<dyn Collider2d<(f32, f32)>>)>)> {
        vec![
            (SpriteButtonTags::Yuzu, self.0.get(SpriteButtonTags::Yuzu as usize).unwrap().clone()),
            (SpriteButtonTags::Aris, self.0.get(SpriteButtonTags::Aris as usize).unwrap().clone()),
            (SpriteButtonTags::Momoi, self.0.get(SpriteButtonTags::Momoi as usize).unwrap().clone()),
            (SpriteButtonTags::Midori, self.0.get(SpriteButtonTags::Midori as usize).unwrap().clone()),
        ]
    }

    #[inline]
    pub fn sprites(&self) -> Vec<&dyn Sprite> {
        self.0.iter()
        .map(|it| &it.0 as &dyn Sprite)
        .collect()
    }

    #[inline]
    pub fn draw<'pass>(
        &'pass self,
        sprite_brush: &'pass SpriteBrush,
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        sprite_brush.draw_textured(rpass, self.sprites().into_iter())
    }
}
