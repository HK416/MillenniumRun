use std::slice::{Iter, IterMut};

use ab_glyph::Font;
use glam::{Vec4, Vec3, Vec2};

use crate::{
    components::{
        collider2d::{Collider2d, shape::AABB},
        sprite::{
            Sprite,
            brush::SpriteBrush,
            objects::{SpriteObject, SpriteBuilder},
        },
        text::{
            Section, 
            brush::TextBrush,
            section::d2::{Section2d, Section2dBuilder}, 
        },
        ui::{
            UserInterface, 
            brush::UiBrush,
            anchor::Anchor,
            objects::{UiObject, UiObjectBuilder}, 
        }, 
    },
    nodes::consts,
    system::error::AppResult, 
};



/// #### 한국어  </br>
/// 사용자 인터페이스 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a user interface object. </br>
/// 
#[derive(Debug)]
pub struct UiComponent {
    pub inner: UiObject,
    pub texts: Vec<Section2d>,
}



/// #### 한국어  </br>
/// 스프라이트 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a sprite object. </br>
/// 
#[derive(Debug)]
pub struct SpriteComponent {
    pub inner: SpriteObject,
    pub collider: Option<Box<dyn Collider2d<(f32, f32)>>>,
}


/// #### 한국어 </br>
/// 배경 스프라이트의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for background sprite. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundSpriteTags {
    Background,
    Cabinet,
    Sofa,
}


#[derive(Debug, Clone, Copy)]
pub struct BackgroundSpriteDescriptor<'a> {
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 배경 스프라이트 모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a background sprite collection. </br>
/// 
#[derive(Debug)]
pub struct BackgroundSprites {
    inner: Vec<SpriteObject>,
}

#[allow(dead_code)]
impl BackgroundSprites {
    pub fn new<'a>(
        device: &wgpu::Device,
        tex_sampler: &wgpu::Sampler,
        sprite_brush: &SpriteBrush,
        desc: &[BackgroundSpriteDescriptor<'a>],
    ) -> AppResult<Self> {
        const SPRITE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);

        // (한국어) `배경` 스프라이트를 생성합니다.
        // (English Translation) Create a `background` sprite.
        const BACKGROUND_POSITION: Vec3 = Vec3::new(0.0 * consts::PIXEL_PER_METER, 0.0 * consts::PIXEL_PER_METER, -10.0 * consts::PIXEL_PER_METER);
        const BACKGROUND_SIZE: Vec2 = Vec2::new(18.0 * consts::PIXEL_PER_METER, 18.0 * consts::PIXEL_PER_METER);
        let background = SpriteBuilder::new(
            Some("Background"),
            tex_sampler,
            desc[BackgroundSpriteTags::Background as usize].texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(BACKGROUND_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(BACKGROUND_POSITION)
        .build(device);


        // (한국어) `캐비넷` 스프라이트를 생성합니다.
        // (English Translation) Create a `cabinet` sprite.
        const CABINET_POSITION: Vec3 = Vec3::new(4.0 * consts::PIXEL_PER_METER, 4.0 * consts::PIXEL_PER_METER, -5.0 * consts::PIXEL_PER_METER);
        const CABINET_SIZE: Vec2 = Vec2::new(2.0 * consts::PIXEL_PER_METER, 4.0 * consts::PIXEL_PER_METER);
        let cabinet = SpriteBuilder::new(
            Some("Cabinet"),
            tex_sampler,
            desc[BackgroundSpriteTags::Cabinet as usize].texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(CABINET_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(CABINET_POSITION)
        .build(device);


        // (한국어) `소파` 스프라이트를 생성합니다.
        // (English Translation) Create a `sofa` sprite.
        const SOFA_POSITION: Vec3 = Vec3::new(0.0 * consts::PIXEL_PER_METER, 2.5 * consts::PIXEL_PER_METER, -3.0 * consts::PIXEL_PER_METER);
        const SOFA_SIZE: Vec2 = Vec2::new(4.5 * consts::PIXEL_PER_METER, 2.25 * consts::PIXEL_PER_METER);
        let sofa = SpriteBuilder::new(
            Some("Sofa"),
            tex_sampler,
            desc[BackgroundSpriteTags::Sofa as usize].texture_view,
            sprite_brush.ref_texture_layout()
        )
        .with_size(SOFA_SIZE)
        .with_color(SPRITE_COLOR)
        .with_translation(SOFA_POSITION)
        .build(device);


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                background,
                cabinet,
                sofa,
            ]
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&SpriteObject> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut SpriteObject> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, SpriteObject> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, SpriteObject> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn sprites(&self) -> Vec<&dyn Sprite> {
        self.inner.iter()
        .map(|it| it as &dyn Sprite)
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


/// #### 한국어 </br>
/// 스프라이트 버튼의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for sprite buttons. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpriteButtonTags {
    Yuzu,
    Aris,
    Momoi,
    Midori,
}


#[derive(Debug, Clone, Copy)]
pub struct SpriteButtonDescriptor<'a> {
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 스프라이트 버튼 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a sprite button collection. </br>
/// 
#[derive(Debug)]
pub struct SpriteButtons {
    inner: Vec<SpriteComponent>,
}

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
        desc: &[SpriteButtonDescriptor<'a>]
    ) -> AppResult<Self> {
        const SPRITE_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

        // (한국어) `Yuzu` 스프라이트를 생성합니다.
        // (English Translation) Create a `Yuzu` sprite.
        const YUZU_POSITION: Vec3 = Vec3::new(3.2 * consts::PIXEL_PER_METER, 3.7 * consts::PIXEL_PER_METER, -4.0 * consts::PIXEL_PER_METER);
        const YUZU_SIZE: Vec2 = Vec2::new(2.0 * consts::PIXEL_PER_METER, 2.0 * consts::PIXEL_PER_METER);
        let yuzu_button = SpriteComponent {
            inner: SpriteBuilder::new(
                Some("Yuzu"),
                tex_sampler,
                desc[SpriteButtonTags::Yuzu as usize].texture_view,
                sprite_brush.ref_texture_layout()
            )
            .with_size(YUZU_SIZE)
            .with_color(SPRITE_COLOR)
            .with_translation(YUZU_POSITION)
            .build(device),
            collider: Some(Box::new(AABB {
                x: YUZU_POSITION.x,
                y: YUZU_POSITION.y,
                width: YUZU_SIZE.x,
                height: YUZU_SIZE.y
            }))
        };


        // (한국어) `Aris` 스프라이트를 생성합니다.
        // (English Translation) Create a `Aris` sprite.
        const ARIS_POSITION: Vec3 = Vec3::new(0.0 * consts::PIXEL_PER_METER, 2.7 * consts::PIXEL_PER_METER, -2.0 * consts::PIXEL_PER_METER);
        const ARIS_SIZE: Vec2 = Vec2::new(2.0 * consts::PIXEL_PER_METER, 2.0 * consts::PIXEL_PER_METER);
        let aris_button = SpriteComponent {
            inner: SpriteBuilder::new(
                Some("Aris"), 
                tex_sampler, 
                desc[SpriteButtonTags::Aris as usize].texture_view,
                sprite_brush.ref_texture_layout()
            )
            .with_size(ARIS_SIZE)
            .with_color(SPRITE_COLOR)
            .with_translation(ARIS_POSITION)
            .build(device),
            collider: Some(Box::new(AABB {
                x: ARIS_POSITION.x,
                y: ARIS_POSITION.y,
                width: ARIS_SIZE.x,
                height: ARIS_SIZE.y
            })),
        };

        
        // (한국어) `Momoi` 스프라이트를 생성합니다.
        // (English Translation) Create a `Momoi` sprite.
        const MOMOI_POSITION: Vec3 = Vec3::new(-3.0 * consts::PIXEL_PER_METER, 1.5 * consts::PIXEL_PER_METER, -1.0 * consts::PIXEL_PER_METER);
        const MOMOI_SIZE: Vec2 = Vec2::new(2.3 * consts::PIXEL_PER_METER, 2.3 * consts::PIXEL_PER_METER);
        let momoi_button = SpriteComponent {
            inner: SpriteBuilder::new(
                Some("Momoi"), 
                tex_sampler, 
                desc[SpriteButtonTags::Momoi as usize].texture_view,
                sprite_brush.ref_texture_layout()
            )
            .with_size(MOMOI_SIZE)
            .with_color(SPRITE_COLOR)
            .with_translation(MOMOI_POSITION)
            .build(device),
            collider: Some(Box::new(AABB {
                x: MOMOI_POSITION.x,
                y: MOMOI_POSITION.y,
                width: MOMOI_SIZE.x,
                height: MOMOI_SIZE.y
            }))
        };


        // (한국어) `Midori` 스프라이트를 생성합니다.
        // (English Translation) Create a `Midori` sprite.
        const MIDORI_POSITION: Vec3 = Vec3::new(3.0 * consts::PIXEL_PER_METER, 1.5 * consts::PIXEL_PER_METER, -1.0 * consts::PIXEL_PER_METER);
        const MIDORI_SIZE: Vec2 = Vec2::new(2.3 * consts::PIXEL_PER_METER, 2.3 * consts::PIXEL_PER_METER);
        let midori_button = SpriteComponent {
            inner: SpriteBuilder::new(
                Some("Midori"), 
                tex_sampler, 
                desc[SpriteButtonTags::Midori as usize].texture_view,
                sprite_brush.ref_texture_layout()
            )
            .with_size(MIDORI_SIZE)
            .with_color(SPRITE_COLOR)
            .with_translation(MIDORI_POSITION)
            .build(device),
            collider: Some(Box::new(AABB {
                x: MIDORI_POSITION.x,
                y: MIDORI_POSITION.y,
                width: MIDORI_SIZE.x,
                height: MIDORI_SIZE.y
            }))
        };


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                yuzu_button,
                aris_button,
                momoi_button,
                midori_button,
            ]
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&SpriteComponent> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut SpriteComponent> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, SpriteComponent> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, SpriteComponent> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn sprites(&self) -> Vec<&dyn Sprite> {
        self.inner.iter()
        .map(|it| &it.inner as &dyn Sprite)
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


/// #### 한국어 </br>
/// 메뉴 버튼의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for menu buttons. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuButtonTags {
    Start,
    Setting,
    Exit,
}


#[derive(Debug, Clone, Copy)]
pub struct MenuButtonDescriptor<'a> {
    pub text: &'a str,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 메뉴 버튼모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a menu button collection. </br>
#[derive(Debug)]
pub struct MenuButtons {
    inner: Vec<UiComponent>,
}

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
        desc: &[MenuButtonDescriptor<'a>],
    ) -> AppResult<Self> {
        const DEV_WINDOW_WIDTH: f32 = 1280.0;
        const DEV_WINDOW_HEIGHT: f32 = 720.0;
        const UI_PIXEL_WIDTH: f32 = 384.0;
        const UI_PIXEL_HEIGHT: f32 = UI_PIXEL_WIDTH / 4.0;
        const UI_PIXEL_GAP: f32 = UI_PIXEL_HEIGHT / 4.0;
        const UI_RATIO_WIDTH: f32 = UI_PIXEL_WIDTH / DEV_WINDOW_WIDTH;
        const UI_RATIO_HEIGHT: f32 = UI_PIXEL_HEIGHT / DEV_WINDOW_HEIGHT;
        const UI_RATIO_GAP: f32 = UI_PIXEL_GAP / DEV_WINDOW_HEIGHT;
        
        const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.2);
        const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);
        const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.1);
        const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);


        // (한국어) `시작` 버튼을 생성합니다.
        // (English Translation) Create a `start` button.
        const START_CENTER_X: f32 = 0.5;
        const START_CENTER_Y: f32 = 0.5 + 1.0 * (UI_RATIO_GAP + UI_RATIO_HEIGHT);
        let anchor = Anchor::new(
            START_CENTER_Y + 0.5 * UI_RATIO_HEIGHT, 
            START_CENTER_X - 0.5 * UI_RATIO_WIDTH, 
            START_CENTER_Y - 0.5 * UI_RATIO_HEIGHT, 
            START_CENTER_X + 0.5 * UI_RATIO_WIDTH
        );
        let start_button = UiComponent {
            inner: UiObjectBuilder::new(
                Some("Start"), 
                tex_sampler, 
                desc[MenuButtonTags::Start as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("Start"), 
                    desc[MenuButtonTags::Start as usize].text,
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue)
            ],
        };


        // (한국어) `게임 설정` 버튼을 생성합니다.
        // (English Translation) Create a `setting` button.
        const SETTING_CENTER_X: f32 = 0.5;
        const SETTING_CENTER_Y: f32 = 0.5 + 0.0 * (UI_RATIO_GAP + UI_RATIO_HEIGHT);
        let anchor = Anchor::new(
            SETTING_CENTER_Y + 0.5 * UI_RATIO_HEIGHT, 
            SETTING_CENTER_X - 0.5 * UI_RATIO_WIDTH, 
            SETTING_CENTER_Y - 0.5 * UI_RATIO_HEIGHT, 
            SETTING_CENTER_X + 0.5 * UI_RATIO_WIDTH
        );
        let setting_button = UiComponent {
            inner: UiObjectBuilder::new(
                Some("Setting"), 
                &tex_sampler, 
                desc[MenuButtonTags::Setting as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("Setting"), 
                    desc[MenuButtonTags::Setting as usize].text,
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ],
        };


        // (한국어) `게임 종료` 버튼을 생성합니다.
        // (English Translation) Create a `exit` button.
        const EXIT_CENTER_X: f32 = 0.5;
        const EXIT_CENTER_Y: f32 = 0.5 - 1.0 * (UI_RATIO_GAP + UI_RATIO_HEIGHT);
        let anchor = Anchor::new(
            EXIT_CENTER_Y + 0.5 * UI_RATIO_HEIGHT, 
            EXIT_CENTER_X - 0.5 * UI_RATIO_WIDTH, 
            EXIT_CENTER_Y - 0.5 * UI_RATIO_HEIGHT, 
            EXIT_CENTER_X + 0.5 * UI_RATIO_WIDTH
        );
        let exit_button = UiComponent {
            inner: UiObjectBuilder::new(
                Some("Exit"), 
                &tex_sampler, 
                desc[MenuButtonTags::Exit as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("Exit"), 
                    desc[MenuButtonTags::Exit as usize].text,
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ],
        };


        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                start_button,
                setting_button,
                exit_button,
            ]
        })
    } 

    #[inline]
    pub fn get(&self, index: usize) -> Option<&UiComponent> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut UiComponent> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, UiComponent> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, UiComponent> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn interfaces(&self) -> Vec<&dyn UserInterface> {
        self.inner.iter()
        .map(|it| &it.inner as &dyn UserInterface)
        .collect()
    }

    #[inline]
    pub fn sections(&self) -> Vec<&dyn Section> {
        self.inner.iter()
        .map(|it| &it.texts)
        .flatten()
        .map(|it| it as &dyn Section)
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


/// #### 한국어 </br>
/// 시스템 버튼의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for system buttons. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemButtonTags {
    Return,
}


#[derive(Debug, Clone, Copy)]
pub struct SystemButtonDescriptor<'a> {
    pub text: Option<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 시스템 버튼 모음 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a system button collection. </br>
/// 
#[derive(Debug)]
pub struct SystemButtons {
    inner: Vec<UiComponent>
}

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
        desc: &[SystemButtonDescriptor<'a>],
    ) -> AppResult<Self> {
        const DEV_WINDOW_WIDTH: f32 = 1280.0;
        const DEV_WINDOW_HEIGHT: f32 = 720.0;

        const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.2);
        const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);
        const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.1);
        const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);


        // (한국어) `되돌아가기` 버튼을 생성합니다.
        // (English Translation) Create a `return` button.
        const UI_RETURN_PIXEL_GAP: f32 = 16.0;
        const UI_RETURN_PIXEL_WIDTH: f32 = 64.0;
        const UI_RETURN_PIXEL_HEIGHT: f32 = UI_RETURN_PIXEL_WIDTH;
        const UI_RETURN_RATIO_GAP_WIDTH: f32 = UI_RETURN_PIXEL_GAP / DEV_WINDOW_WIDTH;
        const UI_RETURN_RATIO_GAP_HEIGHT: f32 = UI_RETURN_PIXEL_GAP / DEV_WINDOW_HEIGHT;
        const UI_RETURN_RATIO_WIDTH: f32 = UI_RETURN_PIXEL_WIDTH / DEV_WINDOW_WIDTH;
        const UI_RETURN_RATIO_HEIGHT: f32 = UI_RETURN_PIXEL_HEIGHT / DEV_WINDOW_HEIGHT;
        let anchor = Anchor::new(
            1.0 - UI_RETURN_RATIO_GAP_HEIGHT,
            0.0 + UI_RETURN_RATIO_GAP_WIDTH,
            1.0 - (UI_RETURN_RATIO_GAP_HEIGHT + UI_RETURN_RATIO_HEIGHT),
            0.0 + (UI_RETURN_RATIO_GAP_WIDTH + UI_RETURN_RATIO_WIDTH)
        );
        let return_button = UiComponent {
            inner: UiObjectBuilder::new(
                Some("Return"),
                tex_sampler,
                desc[SystemButtonTags::Return as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![],
        };

        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                return_button,
            ]
        })
    }
    
    #[inline]
    pub fn get(&self, index: usize) -> Option<&UiComponent> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut UiComponent> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, UiComponent> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, UiComponent> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn interfaces(&self) -> Vec<&dyn UserInterface> {
        self.inner.iter()
        .map(|it| &it.inner as &dyn UserInterface)
        .collect()
    }

    #[inline]
    pub fn sections(&self) -> Vec<&dyn Section> {
        self.inner.iter()
        .map(|it| &it.texts)
        .flatten()
        .map(|it| it as &dyn Section)
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




/// #### 한국어 </br>
/// 종료 윈도우의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for exit window. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExitWindowTags {
    Window,
    Okay,
    Cancel,
}


#[derive(Debug, Clone, Copy)]
pub struct ExitWindowDescriptor<'a> {
    pub text: &'a str,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 종료 윈도우 요소의 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// A collection of exit window elements. </br>
/// 
#[derive(Debug)]
pub struct ExitWindow {
    inner: Vec<UiComponent>,
}

#[allow(dead_code)]
impl ExitWindow {
    /// #### 한국어 </br>
    /// 종료 메시지 상자를 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create a exit message box. </br>
    /// 
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        desc: &[ExitWindowDescriptor<'a>],
    ) -> AppResult<Self> {
        const UI_SCALE: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
        const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.2);
        const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
        const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.1);



        // (한국어) 메시지 박스 윈도우를 생성합니다.
        // (English Translation) Create a message box window.
        let window = UiComponent {
            inner: UiObjectBuilder::new(
                Some("MessageBoxWindow"),
                tex_sampler,
                desc[ExitWindowTags::Window as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(Anchor::new(
                0.5 + 0.4,
                0.5 - 0.3,
                0.5 - 0.4,
                0.5 + 0.3
            ))
            .with_scale(UI_SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("MessageBoxWindow"),
                    desc[ExitWindowTags::Window as usize].text,
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(Anchor::new(
                    0.5 + 0.1, 
                    0.5 - 0.2, 
                    0.5 - 0.0, 
                    0.5 + 0.2
                ))
                .with_scale(UI_SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ],
        };


        // (한국어) 메시지 박스 `확인` 버튼을 생성합니다.
        // (English Translation) Create a message box `okay` button.
        let anchor = Anchor::new(
            0.5 - 0.15, 
            0.5 - 0.2, 
            0.5 - 0.25, 
            0.5 - 0.025
        );
        let okay = UiComponent {
            inner: UiObjectBuilder::new(
                Some("OkayButton"),
                &tex_sampler,
                desc[ExitWindowTags::Okay as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_scale(UI_SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("OkayButton"),
                    desc[ExitWindowTags::Okay as usize].text,
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_scale(UI_SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ],
        };


        // (한국어) 메시지 박스 `확인` 버튼을 생성합니다.
        // (English Translation) Create a message box `okay` button.
        let anchor = Anchor::new(
            0.5 - 0.15, 
            0.5 + 0.025, 
            0.5 - 0.25, 
            0.5 + 0.2
        );
        let cancel = UiComponent {
            inner: UiObjectBuilder::new(
                Some("CancelButton"),
                &tex_sampler,
                desc[ExitWindowTags::Cancel as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_scale(UI_SCALE)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("CancelButton"),
                    desc[ExitWindowTags::Cancel as usize].text,
                    font,
                    text_brush.ref_texture_sampler(),
                    text_brush.ref_buffer_layout(),
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_scale(UI_SCALE)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ],
        };
        
        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                window,
                okay,
                cancel,
            ]
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&UiComponent> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut UiComponent> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, UiComponent> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, UiComponent> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn interfaces(&self) -> Vec<&dyn UserInterface> {
        self.inner.iter()
        .map(|it| &it.inner as &dyn UserInterface)
        .collect()
    }

    #[inline]
    pub fn sections(&self) -> Vec<&dyn Section> {
        self.inner.iter()
        .map(|it| &it.texts)
        .flatten()
        .map(|it| it as &dyn Section)
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


/// #### 한국어 </br>
/// 스테이지 윈도우의 태그 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of tags for stage window. </br>
/// 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageWindowTags {
    Window,
    Enter,
}


#[derive(Debug, Clone, Copy)]
pub struct StageWindowDescriptor<'a> {
    pub text: Option<&'a str>,
    pub texture_view: &'a wgpu::TextureView,
}


/// #### 한국어 </br>
/// 스테이지 윈도우 요소의 모음입니다. </br>
/// 
/// #### English (Translation) </br>
/// A collection of exit window elements. </br>
/// 
#[derive(Debug)]
pub struct StageWindow {
    inner: Vec<UiComponent>,
}

#[allow(dead_code)]
impl StageWindow {
    pub fn new<'a, F: Font>(
        font: &F,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler,
        ui_brush: &UiBrush,
        text_brush: &TextBrush,
        desc: &[StageWindowDescriptor<'a>]
    ) -> AppResult<Self> {
        const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.0);
        const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.2);
        const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.1);


        // (한국어) 메시지 박스 윈도우를 생성합니다.
        // (English Translation) Create a message box window.
        let anchor = Anchor::new(0.85, 0.1, 0.185, 0.6);    
        let window = UiComponent {
            inner: UiObjectBuilder::new(
                Some("MessageBoxWindow"),
                tex_sampler,
                desc[StageWindowTags::Window as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![]
        };


        // (한국어) `입장` 버튼을 생성합니다.
        // (English Translation) Create a `enter` button.
        let anchor = Anchor::new(0.325, 0.15, 0.225, 0.55);
        let enter = UiComponent {
            inner: UiObjectBuilder::new(
                Some("EnterButton"),
                tex_sampler,
                desc[StageWindowTags::Enter as usize].texture_view,
                ui_brush.ref_texture_layout()
            )
            .with_anchor(anchor)
            .with_color(UI_COLOR)
            .with_translation(UI_TRANSLATION)
            .build(device),
            texts: vec![
                Section2dBuilder::new(
                    Some("EnterButton"), 
                    desc[StageWindowTags::Enter as usize].text.unwrap(), 
                    font, 
                    text_brush.ref_texture_sampler(), 
                    text_brush.ref_buffer_layout(), 
                    text_brush.ref_texture_layout()
                )
                .with_anchor(anchor)
                .with_color(TEXT_COLOR)
                .with_translation(TEXT_TRANSLATION)
                .build(device, queue),
            ]
        };

        //-------------------------------------------------------------------------*
        // (한국어) 주의: 순서를 바꾸지 마세요.                                            |
        // (English Translation) Caution: Do not change the order.                 |
        //-------------------------------------------------------------------------*
        Ok(Self { 
            inner: vec![
                window,
                enter,
            ]
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&UiComponent> {
        self.inner.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut UiComponent> {
        self.inner.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, UiComponent> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, UiComponent> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn interfaces(&self) -> Vec<&dyn UserInterface> {
        self.inner.iter()
        .map(|it| &it.inner as &dyn UserInterface)
        .collect()
    }

    #[inline]
    pub fn sections(&self) -> Vec<&dyn Section> {
        self.inner.iter()
        .map(|it| &it.texts)
        .flatten()
        .map(|it| it as &dyn Section)
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
