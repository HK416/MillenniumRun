mod state;
pub mod res;

use std::sync::Arc;
use std::thread::JoinHandle;
use std::collections::HashMap;

use ab_glyph::Font;
use glam::{Vec3, Vec4};
use winit::{event::Event, window::Window};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text::{
            font::FontSet,
            brush::TextBrush, 
            section::d2::{Section2d, Section2dBuilder}, 
        },
        ui::{
            brush::UiBrush,
            anchor::Anchor,
            objects::{UiObject, UiObjectBuilder},
        },
        script::Script,
        user::{Language, Settings, SettingsEncoder}, 
    },
    nodes::path,
    render::texture::ImageDecoder, 
    scene::node::SceneNode,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }, 
};

const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.2);
const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.1);

const INIT_BUTTON_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const MAX_BUTTON_SCALE: Vec3 = Vec3::new(1.25, 1.25, 1.0);



/// #### 한국어 </br>
/// 애플리케이션이 처음 실행될 때 애플리케이션 언어를 설정하는 게임 장면입니다. </br>
/// 
/// #### English Translation
/// A game scene that sets the application language when the application first runs. </br>
/// 
#[derive(Debug)]
pub struct FirstTimeSetupScene { 
    state: state::FirstTimeSetupSceneState,
    elapsed_time: f64,
    loading: Option<JoinHandle<AppResult<Script>>>,
    buttons: HashMap<Language, (UiObject, Section2d)>,
    language: Language,
}

impl SceneNode for FirstTimeSetupScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap();
        let ui_brush = shared.get::<UiBrush>().unwrap();
        let text_brush = shared.get::<TextBrush>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let font_set = shared.get::<FontSet>().unwrap();

        // (한국어) 한국어 선택 버튼을 생성합니다.
        // (English Translation) Create a Korean selection button.
        let texture = asset_bundle.get(path::sys::BUTTON_BASE_TEXTURE_PATH)?
            .read(&ImageDecoder::new(Some("Button(Base)"), device, queue))?;
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.buttons.insert(
            Language::Korean, 
            setup_korean_button(
                font_set.get(path::FONT_MEDIUM_PATH).unwrap(), 
                device, 
                queue, 
                tex_sampler, 
                &texture_view, 
                ui_brush, 
                text_brush
            )?
        );

        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let mut settings = shared.pop::<Settings>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let window = shared.get::<Arc<Window>>().unwrap();
        window.set_title("Millennium Run");

        // (한국어) 선택된 언어를 확인합니다.
        // (English Translation) Confirm the selected language.
        if Language::Unknown == self.language {
            return Err(game_err!("Game Logic Error", "Unknown locale!"));
        }

        // (한국어) 설정의 내용을 갱신합니다.
        // (English Translation) Update the contents of the settings.
        settings.language = self.language;
        asset_bundle.get(path::SETTINGS_PATH)?
            .write(&SettingsEncoder, &settings)?;

        // (한국어) 사용을 완료한 공유 객체를 반환합니다.
        // (English Translation) Returns a shared object that has been used.
        shared.push(settings);

        Ok(())
    }

    #[inline]
    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        state::HANDLE_EVENTS[self.state as usize](self, shared, event)
    }

    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        state::UPDATES[self.state as usize](self, shared, total_time, elapsed_time)
    }

    #[inline]
    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        state::DRAWS[self.state as usize](self, shared)
    }
}

impl Default for FirstTimeSetupScene {
    #[inline]
    fn default() -> Self {
        Self { 
            state: state::FirstTimeSetupSceneState::default(),
            elapsed_time: 0.0,
            loading: None,
            buttons: HashMap::new(),
            language: Language::Unknown,
        }
    }
}


/// #### 한국어 </br>
/// 한국어 선택 버튼의 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface for the Korean selection button. </br>
/// 
fn setup_korean_button<F: Font>(
    font: &F,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex_sampler: &wgpu::Sampler,
    texture_view: &wgpu::TextureView,
    ui_brush: &UiBrush,
    text_brush: &TextBrush,
) -> AppResult<(UiObject, Section2d)> {
    let anchor = Anchor::new(
        0.5 + 0.05, 
        0.5 - 0.2, 
        0.5 - 0.05, 
        0.5 + 0.2
    );

    let ui = UiObjectBuilder::new(
        Some("Button(Korean)"), 
        tex_sampler, 
        texture_view, 
        ui_brush.ref_texture_layout()
    )
    .with_anchor(anchor)
    .with_color(UI_COLOR)
    .with_scale(INIT_BUTTON_SCALE)
    .with_translation(UI_TRANSLATION)
    .build(device);
    let text = Section2dBuilder::new(
        Some("Text(Korean)"),
        "한국어",
        font,
        text_brush.ref_texture_sampler(),
        text_brush.ref_buffer_layout(),
        text_brush.ref_texture_layout()
    )
    .with_anchor(anchor)
    .with_color(TEXT_COLOR)
    .with_scale(INIT_BUTTON_SCALE)
    .with_translation(TEXT_TRANSLATION)
    .build(device, queue);

    Ok((ui, text))
}
