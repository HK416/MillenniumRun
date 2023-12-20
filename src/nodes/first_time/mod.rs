mod state;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use ab_glyph::Font;
use glam::{Vec3, Vec4};
use winit::{event::Event, window::Window};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text2d::{
            font::FontSet,
            brush::Text2dBrush, 
            section::{Section2d, Section2dBuilder},
        },
        ui::{
            brush::UiBrush,
            objects::{UiObject, UiObjectBuilder},
        },
        camera::GameCamera,
        anchor::Anchor, margin::Margin, script::Script,
        user::{Language, Settings, SettingsEncoder}, 
    },
    nodes::path,
    render::texture::DdsTextureDecoder, 
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }, 
};

const ANCHOR_TOP: f32 = 0.5;
const ANCHOR_LEFT: f32 = 0.5;
const ANCHOR_BOTTOM: f32 = 0.5;
const ANCHOR_RIGHT: f32 = 0.5;

const BTN_TOP: i32 = 48;
const BTN_LEFT: i32 = -256;
const BTN_BOTTOM: i32 = -48;
const BTN_RIGHT: i32 = 256;
const BTN_GAP: i32 = 64;

const UI_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
const UI_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);

const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);

const INIT_BUTTON_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const MAX_BUTTON_SCALE: Vec3 = Vec3::new(1.25, 1.25, 1.0);



/// #### 한국어 </br>
/// `FirstTimeSetup` 게임 장면을 준비하는 게임 장면 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene preparing for the `FirstTimeSetup` game scene. </br>
/// 
#[derive(Debug)]
pub struct FirstTimeSetupLoading {
    loading: Option<JoinHandle<AppResult<FirstTimeSetupScene>>>,
}

impl SceneNode for FirstTimeSetupLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush= shared.get::<Arc<UiBrush>>().unwrap().clone();
        let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        let font_set = shared.get::<FontSet>().unwrap();
        let nexon_lv2_gothic_medium = font_set.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("A registered font could not be found.")
            .clone();
        self.loading = Some(thread::spawn(move || {
            // (한국어) 버튼 텍스처를 생성합니다.
            // (English Translation) Create a button texture.
            let texture = asset_bundle
                .get(path::BUTTON_WIDE_TEXTURE_PATH)?
                .read(&DdsTextureDecoder {
                    name: Some("General"),
                    size: wgpu::Extent3d {
                        width:1024,
                        height:192,
                        depth_or_array_layers:1,
                    },
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    mip_level_count: 11,
                    sample_count:1,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                    device: &device,
                    queue: &queue,
                })?;
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());


            // (한국어)한국어 선택 버튼을 생성합니다.
            // (English Translation) Create a Korean selection button.
            let mut buttons = HashMap::new();
            buttons.insert(
                Language::Korean, 
                setup_korean_button(
                    &nexon_lv2_gothic_medium, 
                    &device, 
                    &queue, 
                    &tex_sampler, 
                    &texture_view, 
                    &ui_brush, 
                    &text_brush
                )?
            );

            Ok(FirstTimeSetupScene {
                state: state::FirstTimeSetupSceneState::Wait,
                elapsed_time: 0.0,
                loading: None,
                buttons,
                language: Language::default(),
            })
        }));

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        // (한국어) `FirstTimeSetup` 게임 장면이 준비된 경우 게임 장면으로 변경합니다.
        // (English Translation) If the `FirstTimeSetup` game scene is ready, change to the game scene.
        if self.loading.as_ref().is_some_and(|it| it.is_finished()) {
            let result = self.loading.take().unwrap().join().unwrap();
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(result?));
        }

        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let camera= shared.get::<Arc<GameCamera>>().unwrap();


        // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
        // (English Translation) Wait until the previous operation is finished.
        device.poll(wgpu::Maintain::Wait);

        // (한국어) 다음 프레임을 가져옵니다.
        // (English Translation) Get the next frame.
        let frame = surface.get_current_texture()
            .map_err(|err| game_err!(
                "Failed to get next frame",
                "Failed to get next frame for the following reasons: {}",
                err.to_string()
            ))?;

        // (한국어) 프레임 버퍼의 텍스쳐 뷰를 생성합니다.
        // (English Translation) Creates a texture view of the framebuffer.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(FirstTimeSetupLoading)"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            }, 
                        }),
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );

            // (한국어) 카메라를 바인드 합니다.
            // (English Translation) Bind the camera.
            camera.bind(&mut rpass);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

impl Default for FirstTimeSetupLoading {
    #[inline]
    fn default() -> Self {
        Self { loading: None }
    }
}



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
    loading: Option<JoinHandle<AppResult<Arc<Script>>>>,
    buttons: HashMap<Language, (UiObject, Section2d)>,
    language: Language,
}

impl SceneNode for FirstTimeSetupScene {
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
    text_brush: &Text2dBrush,
) -> AppResult<(UiObject, Section2d)> {
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(BTN_TOP + 0 * BTN_GAP, BTN_LEFT, BTN_BOTTOM + 0 * BTN_GAP, BTN_RIGHT);
    let ui = UiObjectBuilder::new(
        Some("Button(Korean)"), 
        tex_sampler, 
        texture_view, 
        &ui_brush
    )
    .with_anchor(anchor)
    .with_margin(margin)
    .with_color(UI_COLOR)
    .with_scale(INIT_BUTTON_SCALE)
    .with_translation(UI_TRANSLATION)
    .build(device);
    let text = Section2dBuilder::new(
        Some("Text(Korean)"),
        font,
        "한국어",
        &text_brush
    )
    .with_anchor(anchor)
    .with_margin(margin)
    .with_color(TEXT_COLOR)
    .with_scale(INIT_BUTTON_SCALE)
    .with_translation(TEXT_TRANSLATION)
    .build(device, queue);

    Ok((ui, text))
}
