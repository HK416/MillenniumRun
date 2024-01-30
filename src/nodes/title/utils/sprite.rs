use glam::{Vec4, Vec3, Vec2};

use crate::{
    components::{
        collider2d::shape::AABB,
        sprite::{Sprite, SpriteBrush, Instance},
    },
    nodes::consts::PIXEL_PER_METER,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 배경 스프라이트를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a background sprite. </br>
/// 
pub(super) fn create_background_sprite(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    sprite_brush: &SpriteBrush
) -> AppResult<Sprite> {
    Ok(Sprite::new(
        device,
        tex_sampler, 
        texture_view, 
        sprite_brush, 
        [Instance {
            size: Vec2 { x: 8.0, y: 8.0 } * PIXEL_PER_METER,
            ..Default::default()
        }]
    ))
}



const SCALE: f32 = 1.8;
const SPRITE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

const ARIS_X: f32 = -2.8 * PIXEL_PER_METER;
const ARIS_Y: f32 = 0.5 * 1.52 * PIXEL_PER_METER;
const ARIS_HEIGHT: f32 = 1.52 * PIXEL_PER_METER;
const ARIS_WIDTH: f32 = ARIS_HEIGHT * 1024.0 / 1412.0;
const ARIS_TRANSLATION: Vec3 = Vec3::new(ARIS_X, ARIS_Y, 0.0);
const ARIS_SIZE: Vec2 = Vec2::new(ARIS_WIDTH * SCALE, ARIS_HEIGHT * SCALE);

const MOMOI_X: f32 = -1.4 * PIXEL_PER_METER;
const MOMOI_Y: f32 = 0.5 * 1.43 * PIXEL_PER_METER;
const MOMOI_HEIGHT: f32 = 1.43 * PIXEL_PER_METER;
const MOMOI_WIDTH: f32 = MOMOI_HEIGHT * 1024.0 / 1184.0;
const MOMOI_TRANSLATION: Vec3 = Vec3::new(MOMOI_X, MOMOI_Y, 0.0);
const MOMOI_SIZE: Vec2 = Vec2::new(MOMOI_WIDTH * SCALE, MOMOI_HEIGHT * SCALE);

const MIDORI_X: f32 = 0.725 * PIXEL_PER_METER;
const MIDORI_Y: f32 = 0.5 * 1.43 * PIXEL_PER_METER;
const MIDORI_HEIGHT: f32 = 1.43 * PIXEL_PER_METER;
const MIDORI_WIDTH: f32 = MIDORI_HEIGHT * 1024.0 / 1356.0;
const MIDORI_TRANSLATION: Vec3 = Vec3::new(MIDORI_X, MIDORI_Y, 0.0);
const MIDORI_SIZE: Vec2 = Vec2::new(MIDORI_WIDTH * SCALE, MIDORI_HEIGHT * SCALE);

const YUZU_X: f32 = 2.2 * PIXEL_PER_METER;
const YUZU_Y: f32 = 0.5 * 1.5 * PIXEL_PER_METER;
const YUZU_HEIGHT: f32 = 1.5 * PIXEL_PER_METER;
const YUZU_WIDTH: f32 = YUZU_HEIGHT * 1024.0 / 1861.0;
const YUZU_TRANSLATION: Vec3 = Vec3::new(YUZU_X, YUZU_Y, 0.0);
const YUZU_SIZE: Vec2 = Vec2::new(YUZU_WIDTH * SCALE, YUZU_HEIGHT * SCALE);

const STAGE_VIEW_WIDTH: f32 = 2.0 * PIXEL_PER_METER;
const STAGE_VIEW_HEIGHT: f32 = STAGE_VIEW_WIDTH * 3.0 / 4.0;

pub const STAGE_ARIS_TOP: f32 = ARIS_Y + 0.5 * ARIS_HEIGHT * SCALE + 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_ARIS_LEFT: f32 = ARIS_X - 0.5 * STAGE_VIEW_WIDTH;
pub const STAGE_ARIS_BOTTOM: f32 = ARIS_Y + 0.5 * ARIS_HEIGHT * SCALE - 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_ARIS_RIGHT: f32 = ARIS_X + 0.5 * STAGE_VIEW_WIDTH;

pub const STAGE_MOMOI_TOP: f32 = MOMOI_Y + 0.5 * MOMOI_HEIGHT * SCALE + 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_MOMOI_LEFT: f32 = MOMOI_X - 0.5 * STAGE_VIEW_WIDTH;
pub const STAGE_MOMOI_BOTTOM: f32 = MOMOI_Y + 0.5 * MOMOI_HEIGHT * SCALE - 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_MOMOI_RIGHT: f32 = MOMOI_X + 0.5 * STAGE_VIEW_WIDTH;

pub const STAGE_MIDORI_TOP: f32 = MIDORI_Y + 0.5 * MIDORI_HEIGHT * SCALE + 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_MIDORI_LEFT: f32 = MIDORI_X - 0.5 * STAGE_VIEW_WIDTH;
pub const STAGE_MIDORI_BOTTOM: f32 = MIDORI_Y + 0.5 * MIDORI_HEIGHT * SCALE - 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_MIDORI_RIGHT: f32 = MIDORI_X + 0.5 * STAGE_VIEW_WIDTH;

pub const STAGE_YUZU_TOP: f32 = YUZU_Y + 0.5 * YUZU_HEIGHT * SCALE + 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_YUZU_LEFT: f32 = YUZU_X - 0.5 * STAGE_VIEW_WIDTH;
pub const STAGE_YUZU_BOTTOM: f32 = YUZU_Y + 0.5 * YUZU_HEIGHT * SCALE - 0.5 * STAGE_VIEW_HEIGHT;
pub const STAGE_YUZU_RIGHT: f32 = YUZU_X + 0.5 * STAGE_VIEW_WIDTH;



/// #### 한국어 </br>
/// 캐릭터 스프라이트를 생성하는데 사용되는 텍스처 뷰 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A set of texture views used to create the charactor sprites. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub(super) struct CharactorSpriteTextureViews<'a> {
    pub aris_texture_view: &'a wgpu::TextureView,
    pub momoi_texture_view: &'a wgpu::TextureView,
    pub midori_texture_view: &'a wgpu::TextureView,
    pub yuzu_texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 캐릭터 스프라이트를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a charactor sprite. </br>
/// 
pub(super) fn create_character_sprites<'a>(
    device: &'a wgpu::Device, 
    tex_sampler: &'a wgpu::Sampler, 
    texture_views: CharactorSpriteTextureViews<'a>, 
    sprite_brush: &'a SpriteBrush, 
) -> AppResult<Vec<(Sprite, AABB)>> {
    // (한국어) `Aris` 스프라이트를 생성합니다. 
    // (English Translation) Create the `Aris` sprite.
    let aris = (
        Sprite::new(
            device,
            tex_sampler, 
            texture_views.aris_texture_view,
            sprite_brush, 
            [Instance {
                translation: ARIS_TRANSLATION, 
                color: SPRITE_COLOR,
                size: ARIS_SIZE,
                ..Default::default()
            }]
        ),
        AABB {
            x: ARIS_X + 0.0,
            y: ARIS_Y + 0.0,
            width: ARIS_WIDTH * SCALE * 0.3,
            height: ARIS_HEIGHT * SCALE * 1.0,
        }
    );


    // (한국어) `Momoi` 스프라이트를 생성합니다. 
    // (English Translation) Create the `Momoi` sprite.
    let momoi = (
        Sprite::new(
            device,
            tex_sampler, 
            texture_views.momoi_texture_view, 
            sprite_brush, 
            [Instance {
                translation: MOMOI_TRANSLATION, 
                color: SPRITE_COLOR,
                size: MOMOI_SIZE,
                ..Default::default()
            }]
        ),
        AABB {
            x: MOMOI_X + 0.25 * MOMOI_WIDTH,
            y: MOMOI_Y + 0.0,
            width: MOMOI_WIDTH * SCALE * 0.5,
            height: MOMOI_HEIGHT * SCALE * 1.0,
        }
    );


    // (한국어) `Midori` 스프라이트를 생성합니다. 
    // (English Translation) Create the `Midori` sprite.
    let midori = (
        Sprite::new(
            device,
            tex_sampler, 
            texture_views.midori_texture_view, 
            sprite_brush, 
            [Instance {
                translation: MIDORI_TRANSLATION, 
                color: SPRITE_COLOR,
                size: MIDORI_SIZE,
                ..Default::default()
            }]
        ),
        AABB {
            x: MIDORI_X - 0.25 * MIDORI_WIDTH,
            y: MIDORI_Y + 0.0,
            width: MIDORI_WIDTH * SCALE * 0.5,
            height: MIDORI_HEIGHT * SCALE * 1.0,
        }
    );


    // (한국어) `Midori` 스프라이트를 생성합니다. 
    // (English Translation) Create the `Midori` sprite.
    let yuzu = (
        Sprite::new(
            device,
            tex_sampler, 
            texture_views.yuzu_texture_view, 
            sprite_brush, 
            [Instance {
                translation: YUZU_TRANSLATION, 
                color: SPRITE_COLOR,
                size: YUZU_SIZE,
                ..Default::default()
            }]
        ),
        AABB {
            x: YUZU_X + 0.3 * YUZU_WIDTH,
            y: YUZU_Y + 0.0,
            width: YUZU_WIDTH * SCALE * 0.5,
            height: YUZU_HEIGHT * SCALE * 1.0,
        }
    );

    return Ok(vec![
        aris, 
        momoi, 
        midori, 
        yuzu, 
    ]);
} 
