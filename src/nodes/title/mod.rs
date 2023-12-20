mod state;
mod utils;

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use winit::event::Event;
use rodio::{OutputStreamHandle, Source, Sink};

use crate::nodes::title::utils::create_title_scene;
use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        collider2d::shape::AABB,
        sprite::{Sprite, SpriteBrush},
        text2d::{font::FontSet, brush::Text2dBrush, section::Section2d},
        ui::{brush::UiBrush, objects::UiObject},
        camera::GameCamera,
        sound::SoundDecoder,
        script::Script,
        user::Settings,
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
    pub loading: Option<JoinHandle<AppResult<TitleScene>>>
}

impl SceneNode for TitleLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::nodes::path;

        // (한국어) 사용할 공유 객체를 가져옵니다.
        // (English Translation) Get shared object to use.
        let font_set = shared.get::<FontSet>().unwrap();
        let nexon_lv2_gothic_medium = font_set.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("A registered font could not be found.")
            .clone();
        
        let script = shared.get::<Arc<Script>>().unwrap().clone();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap().clone();
        let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        self.loading = Some(thread::spawn(move || {
            create_title_scene(
                &nexon_lv2_gothic_medium, 
                &device, 
                &queue, 
                &tex_sampler, 
                &script, 
                &ui_brush, 
                &text_brush, 
                &sprite_brush, 
                &asset_bundle
            )
        }));

        // (한국어) 카메라를 설정 합니다.
        let mut camera = shared.pop::<GameCamera>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        utils::reset_camera(&mut camera, queue);
        shared.push(camera);

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        if self.loading.as_ref().is_some_and(|it| it.is_finished()) {
            let next_scene = self.loading.take().unwrap().join().unwrap()?;
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
        let camera= shared.get::<GameCamera>().unwrap();

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
                    label: Some("RenderPass(TitleLoading)"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
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

impl Default for TitleLoading {
    #[inline]
    fn default() -> Self {
        Self { loading: None }
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
    pub elapsed_time: f64,
    pub state: state::TitleState,
    pub background: Arc<Sprite>,
    pub sprites: Vec<(Arc<Sprite>, AABB)>,
    pub menu_buttons: Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>,
    pub system_buttons: Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>,
    pub exit_msg_box: Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>,
    pub setting_window: Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>,
    pub stage_window: Vec<(Arc<UiObject>, Vec<Arc<Section2d>>)>,
}

impl SceneNode for TitleScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::{components::sound::play_sound, nodes::path};

        // (한국어) 사용할 공유 객체를 가져옵니다.
        // (English Translation) Get shared object to use.
        let stream = shared.get::<OutputStreamHandle>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let settings = shared.get::<Settings>().unwrap();

        // (한국어) 배경 음악을 재생합니다.
        // (English Translation) Play background music.
        let source = asset_bundle.get(path::THEME64_SOUND_PATH)?
            .read(&SoundDecoder)?
            .repeat_infinite();
        let sink = play_sound(settings.background_volume, source, stream)?;

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
        shared.pop::<Sink>().unwrap().detach();
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
