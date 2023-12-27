mod state;
mod utils;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;

use glam::Vec4;
use winit::event::Event;
use rand::seq::SliceRandom;

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text2d::{
            font::FontSet,
            brush::Text2dBrush, 
            section::{Section2d, Section2dBuilder},
        }, 
        ui::{UiBrush, UiObject},
        anchor::Anchor, 
        camera::GameCamera,
        lights::PointLights,
        transform::{Transform, Projection},
        sprite::SpriteBrush,
        map::{Table, TileBrush}, 
        player::{Actor, Player},
    },
    nodes::{path, consts::PIXEL_PER_METER}, 
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};

pub const INIT_RANGE: usize = 6;
pub const NUM_TILE_ROWS: usize = 101;
pub const NUM_TILE_COLS: usize = 101;
pub const NUM_TILES: usize = NUM_TILE_ROWS * NUM_TILE_COLS;
pub const SPAWN_POSITIONS: [(usize, usize); 8] = [
    (25, 25), (25, 50), (25, 75),
    (50, 25), (50, 75),
    (75, 25), (75, 50), (75, 75)
];

pub const CAMERA_VIEW_WIDTH: f32 = 80.0 * PIXEL_PER_METER;
pub const CAMERA_VIEW_HEIGHT: f32 = 60.0 * PIXEL_PER_METER;
pub const PROJECTION_MAT: Projection = Projection::new_ortho(
    0.5 * CAMERA_VIEW_HEIGHT, 
    -0.5 * CAMERA_VIEW_WIDTH, 
    -0.5 * CAMERA_VIEW_HEIGHT, 
    0.5 * CAMERA_VIEW_WIDTH, 
    0.0 * PIXEL_PER_METER, 
    1000.0 * PIXEL_PER_METER
);

pub const EDGE_COLOR: Vec4 = Vec4::new(137.0 / 255.0, 207.0 / 255.0, 243.0 / 255.0, 1.0);
pub const FILL_COLOR: Vec4 = Vec4::new(160.0 / 255.0, 233.0 / 255.0, 255.0 / 255.0, 1.0);
pub const LINE_COLOR: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);


#[derive(Debug)]
pub struct InGameLoading {
    loading: Option<JoinHandle<AppResult<InGameScene>>>,
}

impl SceneNode for InGameLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let font_set = shared.get::<FontSet>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap().clone();
        let tile_brush = shared.get::<Arc<TileBrush>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        let actor = shared.get::<Actor>().cloned().unwrap_or_default();

        let nexon_lv2_gothic_medium = font_set.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("Registered font not found!")
            .clone();

        // (한국어) 다른 스레드에서 `InGame` 게임 장면을 준비합니다.
        // (English Translation) Prepare the `InGame` game scene in another thread. 
        self.loading = Some(thread::spawn(move || {
            let background = utils::create_background(
                actor, 
                &device, 
                &queue, 
                &tex_sampler, 
                &ui_brush, 
                &asset_bundle
            )?;


            let timer = Section2dBuilder::new(
                Some("Timer"),
                &nexon_lv2_gothic_medium, 
                "-:--",
                &text_brush
            )
            .with_anchor(Anchor::new(0.75, 0.85, 0.55, 0.85))
            .build(&device, &queue);


            let mut table = Table::new(
                NUM_TILE_ROWS.try_into().unwrap(), 
                NUM_TILE_COLS.try_into().unwrap(), 
                EDGE_COLOR, 
                FILL_COLOR, 
                (-35.0 * PIXEL_PER_METER, -25.0 * PIXEL_PER_METER, -1.0 * PIXEL_PER_METER).into(), 
                (0.5 * PIXEL_PER_METER, 0.5 * PIXEL_PER_METER).into()
            );

            let mut arr = [0, 1, 2, 3, 4, 5, 6, 7];
            arr.shuffle(&mut rand::thread_rng());
            let (row, col) = SPAWN_POSITIONS[arr[0]];
            let player = Player::new(
                &device, 
                &queue, 
                &tex_sampler, 
                &sprite_brush, 
                &asset_bundle, 
                actor, 
                &table, 
                row,
                col,
                0.0 * PIXEL_PER_METER,
                (3.0 * PIXEL_PER_METER, 3.0 * PIXEL_PER_METER).into()
            )?;
            for r in row - INIT_RANGE..=row + INIT_RANGE {
                for c in col - INIT_RANGE..=col + INIT_RANGE {
                    if r == row - INIT_RANGE || r == row + INIT_RANGE || c == col - INIT_RANGE || c == col + INIT_RANGE {
                        table.tiles[r][c].color = EDGE_COLOR;
                    } else {
                        table.tiles[r][c].visited = true;
                        table.tiles[r][c].color = (160.0 / 255.0, 233.0 / 255.0, 255.0 / 255.0, 0.0).into();
                    }
                }
            }

            tile_brush.update(&queue, |instances| {
                for (instance, tile) in instances.iter_mut().zip(table.tiles.iter().flatten()) {
                    instance.transform = tile.transform;
                    instance.color = tile.color;
                    instance.size = table.size;
                }
            });


            Ok(InGameScene {  
                remaining_time: 120.0, 
                timer_ui: timer, 
                num_total_tiles: NUM_TILES as u32,
                num_owned_tiles: 0, 
                owned_tiles: VecDeque::new(), 
                state: state::InGameState::default(), 
                background, 
                table, 
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
            data.projection = PROJECTION_MAT;
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
    pub remaining_time: f64, 
    pub timer_ui: Section2d, 
    
    pub num_total_tiles: u32,
    pub num_owned_tiles: u32,
    pub owned_tiles: VecDeque<(f64, Vec<(usize, usize)>)>, 

    pub state: state::InGameState,
    pub background: Vec<UiObject>, 
    pub table: Table, 
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
