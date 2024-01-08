mod state;
mod utils;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::{VecDeque, HashMap};

use ab_glyph::FontArc;
use winit::event::Event;
use rodio::OutputStreamHandle;

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        ui::{UiBrush, UiObject},
        text::{TextBrush, Text}, 
        sprite::SpriteBrush,
        bullet::{Bullet, BulletBrush},
        camera::CameraCreator,
        transform::Projection,
        table::{Table, TileBrush}, 
        player::{Actor, Player, FaceState},
        user::Settings, 
    },
    nodes::{path, consts::PIXEL_PER_METER}, 
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};

pub const NUM_TILE_ROWS: usize = 100;
pub const NUM_TILE_COLS: usize = 100;
pub const NUM_TILES: usize = NUM_TILE_ROWS * NUM_TILE_COLS;

pub const PERCENT_DURATION: f64 = 0.25;


#[derive(Debug)]
pub struct InGameLoading {
    loading: Option<JoinHandle<AppResult<InGameScene>>>,
}

impl SceneNode for InGameLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        prepare_brushes(self, shared)?;
        prepare_in_game_scene(self, shared)?;
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
            let mut _rpass = encoder.begin_render_pass(
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

/// #### 한국어 </br>
/// `InGame` 게임 장면에서 사용되는 그리기 도구들을 준비합니다. </br>
/// 
/// #### English (Translation) </br>
/// Prepare drawing tools used in `InGame` game scene. </br>
/// 
fn prepare_brushes(_this: &mut InGameLoading, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
    let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();

    // (한국어) 총알 그리기 도구를 생성합니다.
    // (English Translation) Create a bullet drawing tool. 
    let bullet_brush = create_bullet_brush(
        device, 
        &camera_creator.camera_layout, 
        config.format, 
        asset_bundle
    )?;
    
    // (한국어) 타일 그리기 도구를 생성합니다.
    // (English Translation) Create a tile drawing tool.
    let tile_brush = create_tile_brush(
        device, 
        &camera_creator.camera_layout, 
        config.format, 
        asset_bundle
    )?;

    // (한국어) 생성된 그리기 도구들을 공유 객체에 추가합니다.
    // (English Translation) Add the created drawing tools to the shared object. </br>
    shared.push(bullet_brush);
    shared.push(tile_brush);

    Ok(())
}

/// #### 한국어 </br>
/// 총알을 그리는 도구를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a tool to draw bullets. </br>
/// 
fn create_bullet_brush(
    device: &wgpu::Device, 
    camera_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat, 
    asset_bundle: &AssetBundle
) -> AppResult<Arc<BulletBrush>> {
    BulletBrush::new(
        device, 
        camera_layout,
        render_format, 
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare:wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }), 
        wgpu::MultisampleState::default(), 
        None, 
        asset_bundle
    )
}

/// #### 한국어 </br>
/// 타일 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets tile drawing tools. </br>
/// 
fn create_tile_brush(
    device: &wgpu::Device, 
    camera_layout: &wgpu::BindGroupLayout, 
    render_format: wgpu::TextureFormat, 
    asset_bundle: &AssetBundle
) -> AppResult<Arc<TileBrush>> {
    TileBrush::new(
        device, 
        camera_layout, 
        render_format, 
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare:wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }), 
        wgpu::MultisampleState::default(), 
        None,
        asset_bundle, 
        NUM_TILES, 
    )
}

/// #### 한국어 </br>
/// `InGame` 게임 장면을 준비합니다. </br>
/// 
/// #### English (Translation) </br>
/// Prepare the `InGame` game scene. </br>
/// 
fn prepare_in_game_scene(this: &mut InGameLoading, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let actor = shared.get::<Actor>().cloned().unwrap_or_default();
    let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap().clone();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
    let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap().clone();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap().clone();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap().clone();
    let bullet_brush = shared.get::<Arc<BulletBrush>>().unwrap().clone();
    let texture_map = shared.get::<Arc<HashMap<String, wgpu::Texture>>>().unwrap().clone();
    let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();

    // (한국어) 다른 스레드에서 `InGame` 게임 장면을 준비합니다.
    // (English Translation) Prepare the `InGame` game scene in another thread. 
    this.loading = Some(thread::spawn(move || {
        // (한국어) 현재 게임 장면에서 사용할 에셋들을 불러옵니다.
        // (English Translation) Loads assets to be used in the current game scene. 
        asset_bundle.get(path::CLICK_SOUND_PATH)?;
        asset_bundle.get(path::CANCEL_SOUND_PATH)?;
        asset_bundle.get(path::BUTTON_ETC_TEXTURE_PATH)?;
        asset_bundle.get(path::INGAME_BACKGROUND_TEXTURE_PATH)?;
        asset_bundle.get(path::WINDOW_RATIO_4_3_TEXTURE_PATH)?;

        let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)
            .expect("Registered font not found!");

        utils::create_game_scene(
            actor, 
            nexon_lv2_gothic_medium, 
            &device, 
            &queue, 
            &tex_sampler, 
            &text_brush, 
            &ui_brush, 
            &sprite_brush, 
            &tile_brush, 
            &bullet_brush, 
            &texture_map, 
            &asset_bundle
        )
    }));

    Ok(())
}



#[derive(Debug)]
pub struct InGameScene {
    pub mouse_pressed: bool, 
    pub keyboard_pressed: bool, 

    pub timer: f64, 
    pub remaining_time: f64, 
    pub state: state::InGameState,
    
    pub percent: Text, 
    pub percent_timer: f64, 
    pub num_total_tiles: u32,
    pub num_owned_tiles: u32,
    pub owned_tiles: VecDeque<(f64, Vec<(usize, usize)>)>, 

    pub owned_hearts: VecDeque<UiObject>, 
    pub lost_hearts: VecDeque<(f64, UiObject)>, 

    pub foreground: UiObject, 
    pub background: UiObject, 
    pub stage_image: UiObject, 
    pub menu_button: UiObject, 
    pub remaining_timer_bg: UiObject, 
    pub remaining_timer_text: Text, 

    
    pub table: Table, 
    pub player: Player, 
    pub player_faces: HashMap<FaceState, UiObject>, 
    pub player_bullet: Bullet, 

    pub player_startup_sound: &'static str, 
    pub player_smile_sounds: Vec<&'static str>, 
    pub player_damage_sounds: Vec<&'static str>,
}

impl SceneNode for InGameScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 현재 게임 장면에서 사용할 카메라를 생성합니다.
        // (English Translation) Creates a camera to use in the current game scene. 
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap().clone();
        let camera = camera_creator.create(
            Some("InGame"), 
            None, 
            None, 
            Some(Projection::new_ortho(
                30.0 * PIXEL_PER_METER, 
                -40.0 * PIXEL_PER_METER, 
                -30.0 * PIXEL_PER_METER, 
                40.0 * PIXEL_PER_METER, 
                0.0 * PIXEL_PER_METER, 
                1000.0 * PIXEL_PER_METER
            )), 
            None
        );
        shared.push(Arc::new(camera));

        // (한국어) 현재 게임 장면에서 사용되는 [`rodio::Sink`] 집합을 생성합니다.
        // (English Translation) Creates a set of [`rodio::Sink`] used in current game scene. 
        let settings = shared.get::<Settings>().unwrap();
        let stream = shared.get::<OutputStreamHandle>().unwrap();
        let audio = utils::InGameAudio::new(settings, stream)?;
        shared.push(audio);

        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 현재 게임 장면에서 사용되는 [`rodio::Sink`] 집합을 해제합니다.
        // (English Translation) Releases a set of [`rodio::Sink`] used in current game scene. 
        shared.pop::<Arc<utils::InGameAudio>>().unwrap();

        // (한국어) 사용한 그리기 도구를 공유객체에서 해제합니다.
        // (English Translation) Release the used drawing tool from the shared object. 
        shared.pop::<Arc<BulletBrush>>().unwrap();
        shared.pop::<Arc<TileBrush>>().unwrap();

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
