mod state;
mod utils;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use ab_glyph::FontArc;
use winit::event::Event;
use rodio::{OutputStreamHandle, Source, Sink};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        ui::{UiBrush, UiObject},
        text::{TextBrush, Text, TextBuilder},
        sprite::{Sprite, SpriteBrush},
        collider2d::shape::AABB,
        anchor::Anchor, margin::Margin, 
        camera::{CameraCreator, GameCamera},
        transform::Projection, 
        sound::SoundDecoder,
        script::Script,
        user::Settings,
        player::Actor, 
    },
    render::depth::DepthBuffer, 
    nodes::{
        path, 
        consts::PIXEL_PER_METER, 
        title::state::TitleState, 
    }, 
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// `Title` 게임 장면을 준비하는 게임 장면 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene preparing for the `Title` game scene. </br>
/// 
#[derive(Debug)]
pub struct TitleLoading {
    actor: Option<Actor>, 
    loading_text: Option<Text>, 
    loading: Option<JoinHandle<AppResult<TitleScene>>>, 
}

impl TitleLoading {
    #[inline]
    pub fn new(actor: Actor) -> Self {
        Self { 
            actor: Some(actor), 
            ..Default::default()
        }
    }
}

impl SceneNode for TitleLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체를 가져옵니다.
        // (English Translation) Get shared object to use.
        let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap().clone();
        let script = shared.get::<Arc<Script>>().unwrap().clone();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap().clone();
        let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap().clone();
        let texture_map = shared.get::<Arc<HashMap<String, wgpu::Texture>>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();

        self.loading = Some(thread::spawn(move || {
            // (한국어) 현재 장면에서 사용할 에셋들을 불러옵니다. 
            // (English Translation) Loads assets to be used in the current game scene. 
            asset_bundle.get(path::CLICK_SOUND_PATH)?;
            asset_bundle.get(path::CANCEL_SOUND_PATH)?;
            asset_bundle.get(path::BUTTON_WIDE_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_MEDIUM_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_RETURN_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_START_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_SETTING_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_EXIT_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BACKGROUND_TEXTURE_PATH)?;
            asset_bundle.get(path::WINDOW_RATIO_4_3_TEXTURE_PATH)?;
            asset_bundle.get(path::ARIS_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::MOMOI_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::MIDORI_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::YUZU_STANDING_TEXTURE_PATH)?;
        
            // TODO: 스테이지 이미지들을 추가하세요.
            asset_bundle.get(path::TEMP_STAGE_TEXTURE_PATH)?;

            let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("A registered font could not be found.");

            utils::create_title_scene(
                &nexon_lv2_gothic_medium, 
                &device, 
                &queue, 
                &tex_sampler, 
                &script, 
                &ui_brush, 
                &text_brush, 
                &sprite_brush, 
                &texture_map, 
                &asset_bundle
            )
        }));

        // (한국어) 로딩 텍스트를 생성합니다.
        // (English Translation) Create a loading text.
        let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap();

        let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("Registered font could not found!");
        let text = TextBuilder::new(
            Some("LoadingText"), 
            nexon_lv2_gothic_medium, 
            "Loading", 
            text_brush
        )
        .with_anchor(Anchor::new(0.0, 1.0, 0.0, 1.0))
        .with_margin(Margin::new(128, -256, 0, 0))
        .with_color(if self.actor.is_some() { (1.0, 1.0, 1.0, 1.0) } else { (0.0, 0.0, 0.0, 1.0) }.into())
        .build(device, queue);
        self.loading_text = Some(text);
        
        // (한국어) 현재 게임 장면에서 사용할 카메라를 생성합니다.
        // (English Translation) Creates a camera to use in the current game scene. 
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap().clone();
        let camera = if self.actor.is_some() {
            camera_creator.create(
                Some("Title"), 
                None, 
                None, 
                Some(Projection::new_ortho(
                    utils::STAGE_TOP, 
                    utils::STAGE_LEFT, 
                    utils::STAGE_BOTTOM, 
                    utils::STAGE_RIGHT, 
                    0.0 * PIXEL_PER_METER, 
                    1000.0 * PIXEL_PER_METER
                )), 
                None
            )
        } else {
            camera_creator.create(
                Some("Title"), 
                None, 
                None, 
                Some(Projection::new_ortho(
                    utils::MENU_TOP, 
                    utils::MENU_LEFT, 
                    utils::MENU_BOTTOM, 
                    utils::MENU_RIGHT, 
                    0.0 * PIXEL_PER_METER, 
                    1000.0 * PIXEL_PER_METER
                )), 
                None
            )
        };
        shared.push(Arc::new(camera));

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared objects to use.
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

        if self.loading.as_ref().is_some_and(|it| it.is_finished()) {
            let mut next_scene = self.loading.take().unwrap().join().unwrap()?;
            if self.actor.is_some() {
                next_scene.state = TitleState::ReturnStage;
                next_scene.foreground.update(queue, |data| {
                    data.color = (0.0, 0.0, 0.0, 1.0).into();
                });
            } else {
                next_scene.state = TitleState::Enter;
                next_scene.foreground.update(queue, |data| {
                    data.color = (1.0, 1.0, 1.0, 1.0).into();
                });
            }
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(next_scene));
        }
        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let camera = shared.get::<Arc<GameCamera>>().unwrap();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap();

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
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(TitleLoading)"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(if self.actor.is_none() { 
                                    wgpu::Color::WHITE 
                                } else {
                                    wgpu::Color::BLACK
                                }),
                                store: wgpu::StoreOp::Store,
                            }, 
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                        view: depth.view(), 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Discard
                        }), 
                        stencil_ops: None 
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );

            camera.bind(&mut rpass);
            text_brush.draw(&mut rpass, [
                self.loading_text.as_ref().unwrap()
            ].into_iter());
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

impl Default for TitleLoading {
    #[inline]
    fn default() -> Self {
        Self { 
            actor: None, 
            loading_text: None, 
            loading: None  
        }
    }
}



/// #### 한국어 </br>
/// 사용자가 게임의 시작, 설정, 종료, 등을 선택할 수 있는 게임 장면입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene where user can select the start, setup, exit, etc. of the game. </br>
/// 
#[derive(Debug)]
pub struct TitleScene {
    pub light_timer: f64,
    pub elapsed_time: f64,
    pub state: state::TitleState,
    pub foreground: UiObject, 
    pub background: Sprite,
    pub sprites: Vec<(Sprite, AABB)>,
    pub menu_buttons: Vec<(UiObject, Vec<Text>)>,
    pub system_buttons: Vec<(UiObject, Vec<Text>)>,
    pub exit_msg_box: Vec<(UiObject, Vec<Text>)>,
    pub setting_window: Vec<(UiObject, Vec<Text>)>,
    pub stage_window: Vec<(UiObject, Vec<Text>)>,
}

impl SceneNode for TitleScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::components::sound;

        // (한국어) 사용할 공유 객체를 가져옵니다.
        // (English Translation) Get shared object to use.
        let stream = shared.get::<OutputStreamHandle>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let settings = shared.get::<Settings>().unwrap();

        // (한국어) 배경 음악을 재생합니다.
        // (English Translation) Play background music.
        let source = asset_bundle.get(path::THEME64_SOUND_PATH)?
            .read(&SoundDecoder)?
            .amplify(0.5)
            .repeat_infinite();
        let sink = sound::play_sound(settings.background_volume, source, stream)?;

        // (한국어) 사용을 완료한 에셋을 정리합니다.
        // (English Translation) Release assets that have been used.
        asset_bundle.release(path::THEME64_SOUND_PATH);
    
        // (한국어) 배경 음악을 공유 객체에 등록합니다.
        // (English Translation) Register background music to a shared object.
        shared.push(sink);

        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 배경 음악을 제거합니다.
        // (English Translation) Detach background music.
        shared.pop::<Sink>().unwrap().stop();
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
