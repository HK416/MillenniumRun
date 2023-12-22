mod state;
mod utils;

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use winit::event::Event;

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        ui::{brush::UiBrush, objects::UiObject},
        camera::GameCamera,
        lights::PointLights,
        transform::{Transform, Projection},
        sprite::SpriteBrush,
        map::TileMap,
        player::{Actor, Player},
    },
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};



#[derive(Debug)]
pub struct InGameLoading {
    loading: Option<JoinHandle<AppResult<InGameScene>>>,
}

impl SceneNode for InGameLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::nodes::consts::PIXEL_PER_METER;

        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        let actor = shared.get::<Actor>().cloned().unwrap_or_default();


        // (한국어) 다른 스레드에서 `InGame` 게임 장면을 준비합니다.
        // (English Translation) Prepare the `InGame` game scene in another thread. 
        self.loading = Some(thread::spawn(move || {
            let background = utils::create_background(
                &device, 
                &queue, 
                &tex_sampler, 
                &ui_brush, 
                &asset_bundle
            )?;

            let tile_map = TileMap::new(
                &device, 
                &queue, 
                &tex_sampler, 
                &sprite_brush, 
                &asset_bundle, 
                std::num::NonZeroUsize::new(50).unwrap(), 
                std::num::NonZeroUsize::new(50).unwrap(), 
                (152.0 / 255.0, 223.0 / 255.0, 255.0 / 255.0, 1.0).into(), 
                (-35.0 * PIXEL_PER_METER, -25.0 * PIXEL_PER_METER, -1.0 * PIXEL_PER_METER).into(), 
                (1.0 * PIXEL_PER_METER, 1.0 * PIXEL_PER_METER).into()
            )?;

            let map = tile_map.clone();
            let player = Player::new(
                &device, 
                &queue, 
                &tex_sampler, 
                &sprite_brush, 
                &asset_bundle, 
                actor, 
                map, 
                49,
                0,
                0.0 * PIXEL_PER_METER,
                (3.0 * PIXEL_PER_METER, 3.0 * PIXEL_PER_METER).into()
            )?;

            Ok(InGameScene { 
                scene_timer: 0.0,
                state: state::InGameState::default(),
                background, 
                tile_map, 
                player, 
            })
        }));

        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let camera = shared.get::<Arc<GameCamera>>().unwrap();
        let lights = shared.get::<Arc<PointLights>>().unwrap();

        // (한국어) 카메라를 설정합니다.
        // (English Translation) Set up the camera.
        camera.update(queue, |data| {
            data.transform = Transform::default();
            data.projection = Projection::new_ortho(
                30.0 * PIXEL_PER_METER, 
                -40.0 * PIXEL_PER_METER, 
                -30.0 * PIXEL_PER_METER, 
                40.0 * PIXEL_PER_METER, 
                0.0 * PIXEL_PER_METER, 
                1000.0 * PIXEL_PER_METER
            );
        });

        // (한국어) 조명을 설정합니다.
        // (English Translation) Set up the lights.
        lights.update(queue, |data| {
            data.num_points = 0;
        });

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        // (한국어) `InGame` 게임 장면이 로드 될 때까지 기다립니다.
        // (English Translation) Wait for the `InGame` game scene to load.
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
        let camera = shared.get::<Arc<GameCamera>>().unwrap();

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

        // (한국어) 프레임 버퍼의 텍스처 뷰를 생성합니다.
        // (English Translation) Creates a texture view of the framebuffer.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        
        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(InGameLoading)"),
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
                },
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


impl Default for InGameLoading {
    #[inline]
    fn default() -> Self {
        Self { loading: None }
    }
}



#[derive(Debug)]
pub struct InGameScene {
    scene_timer: f64,
    pub state: state::InGameState,
    pub background: UiObject, 
    pub tile_map: Arc<TileMap>, 
    pub player: Player, 
}

impl SceneNode for InGameScene {
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
