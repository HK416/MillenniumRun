mod state;
pub mod res;

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use winit::event::Event;

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text::{
            brush::TextBrush, font::FontSet,
            section::d2::{Section2d, Section2dBuilder},
        },
        ui::{
            brush::UiBrush,
            anchor::Anchor,
            objects::{UiObject, UiObjectBuilder},
        },
        user::{Settings, Language},
    },
    nodes::{path, title},
    render::texture::ImageDecoder,
    scene::node::SceneNode,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// 게임 인트로를 보여주는 게임 장면입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene showing the game intro. </br>
/// 
#[derive(Debug)]
pub struct IntroScene {
    state: state::IntroState,
    loading: Option<JoinHandle<AppResult<()>>>,
    elapsed_time: f64,
    notify_texts: Vec<Section2d>,
    logo_images: Vec<UiObject>,
}

impl SceneNode for IntroScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        setup_notify_texts(self, shared)?;
        setup_logo_images(self, shared)?;
        load_game_assets(self, shared)?;
        Ok(())
    }

    fn handle_events(&mut self, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
        Ok(())
    }

    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        state::UPDATE[self.state as usize](self, shared, total_time, elapsed_time)
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        state::DRAW[self.state as usize](self, shared)
    }
}

impl Default for IntroScene {
    #[inline]
    fn default() -> Self {
        Self {
            state: state::IntroState::FadeIn,
            loading: None,
            elapsed_time: 0.0,
            notify_texts: Vec::new(),
            logo_images: Vec::new(),
        }
    }
}


/// #### 한국어 </br>
/// `intro` 게임 장면에 사용되는 알림 텍스트들을 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set the notification text used in the `intro` game scene. </br>
/// 
fn setup_notify_texts(this: &mut IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let text_brush = shared.get::<TextBrush>().unwrap();
    let font_set = shared.get::<FontSet>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 알림 텍스트 생성하기.
    // (English Translation) Create notification text.
    let text = match settings.language {
        Language::Unknown => Err(game_err!("Game Logic Error!", "Unknown locale!")),
        Language::Korean => Ok("알 림"),
    }?;
    this.notify_texts.push(
        Section2dBuilder::new(
            Some("Notify Title"),
            text,
            font_set.get(path::FONT_BLOD_PATH).unwrap(), 
            text_brush.ref_texture_sampler(),
            text_brush.ref_buffer_layout(),
            text_brush.ref_texture_layout()
        )
        .with_anchor(Anchor::new(
            0.75 + 0.1, 
            0.5 - 0.1, 
            0.75 - 0.1, 
            0.5 + 0.1
        ))
        .build(&device, &queue)
    );

    let text = match settings.language {
        Language::Unknown => Err(game_err!("Game Logic Error!", "Unknown locale!")),
        Language::Korean => Ok("이 게임은 Blue Archive의 팬 제작 게임입니다."),
    }?;
    this.notify_texts.push(
        Section2dBuilder::new(
            Some("Notify contents"),
            text,
            font_set.get(path::FONT_MEDIUM_PATH).unwrap(),
            text_brush.ref_texture_sampler(),
            text_brush.ref_buffer_layout(),
            text_brush.ref_texture_layout()
        )
        .with_anchor(Anchor::new(
            0.5 + 0.1, 
            0.5 - 0.2, 
            0.5 - 0.1, 
            0.5 + 0.2    
        ))
        .build(&device, &queue)
    );

    Ok(())
}


/// #### 한국어 </br>
/// `intro` 게임 장면에 사용되는 로고 이미지들을 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set the logo images used in the `intro` game scene. </br>
/// 
fn setup_logo_images(this: &mut IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let ui_brush = shared.get::<UiBrush>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap();

    // (한국어) 로고 텍스처를 생성합니다.
    // (English Translation) Create logo texture.
    let texture = asset_bundle.get(path::intro::LOGO_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Texture(Logo)"), device, queue))?;
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 로고 이미지 사용자 인터페이스를 생성합니다.
    // (English Translation) Create logo image user interface.
    this.logo_images.push(
        UiObjectBuilder::new(
            Some("Logo"),
            tex_sampler,
            &texture_view,
            ui_brush.ref_texture_layout(),
        )
        .with_anchor(Anchor::new(
            0.5 + 0.267, 
            0.5 - 0.15, 
            0.5 - 0.267, 
            0.5 + 0.15
        ))
        .with_color((1.0, 1.0, 1.0, 0.0).into())
        .build(device)
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::intro::LOGO_TEXTURE_PATH);

    Ok(())
}


fn load_game_assets(this: &mut IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let asset_bundle = shared.get::<AssetBundle>().unwrap();

    // (한국어) 다음 장면에 사용될 게임 에셋을 로드합니다.
    // (English Translation) Load the game assets that will be used in the next scene.
    let asset_bundle_cloned = asset_bundle.clone();
    this.loading = Some(thread::spawn(move || {
        // (한국어) `Title` 게임 장면에서 사용될 에셋들을 로드합니다.
        // (English Translation) Loads assets used in the `Title` game scene.
        for rel_path in title::res::ASSETS {
            asset_bundle_cloned.get(rel_path)?;
        }

        Ok(())
    }));

    Ok(())
}
