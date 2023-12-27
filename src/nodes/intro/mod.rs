mod state;

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use ab_glyph::Font;

use crate::scene::state::SceneState;
use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text2d::{
            brush::Text2dBrush, font::FontSet,
            section::{Section2d, Section2dBuilder},
        },
        ui::{UiBrush, UiObject, UiObjectBuilder},
        camera::GameCamera,
        anchor::Anchor,
        margin::Margin,
        script::{Script, ScriptTags},
    },
    nodes::path,
    render::texture::DdsTextureDecoder,
    scene::node::SceneNode,
    system::{
        error::{AppResult, GameError},
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// `Intro` 게임 장면을 준비하는 게임 장면 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene preparing for the `Intro` game scene. </br>
/// 
#[derive(Debug)]
pub struct IntroLoading {
    loading: Option<JoinHandle<AppResult<IntroScene>>>,
}

impl SceneNode for IntroLoading {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
        let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let font_set = shared.get::<FontSet>().unwrap();
        let script = shared.get::<Arc<Script>>().unwrap();

        let nexon_lv2_gothic_bold = font_set.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)
            .expect("A registered font could not be found.")
            .clone();
        let nexon_lv2_gothic_medium = font_set.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("A registered font could not be found.")
            .clone();
        let device_cloned = device.clone();
        let queue_cloned = queue.clone();
        let tex_sampler_cloned = tex_sampler.clone();
        let ui_brush_cloned = ui_brush.clone();
        let text_brush_cloned = text_brush.clone();
        let asset_bundle_cloned = asset_bundle.clone();
        let script_cloned = script.clone();
        self.loading = Some(thread::spawn(move || {
            // (한국어) 게임 장면에서 사용할 에셋들을 로드합니다. 
            // (English Translation) Loads assets to be used in the game scene.
            asset_bundle_cloned.get(path::LOGO_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::ARIS_TITLE_SOUND_PATH)?;
            asset_bundle_cloned.get(path::MOMOI_TITLE_SOUND_PATH)?;
            asset_bundle_cloned.get(path::MIDORI_TITLE_SOUND_PATH)?;
            asset_bundle_cloned.get(path::YUZU_TITLE_SOUND_PATH)?;

            // (한국어) 게임 장면에서 사용할 객체들을 생성합니다.
            // (English Translation) Creates objects to be used in the game scene.
            let mut notifications = Vec::with_capacity(2);
            notifications.push(create_notify_title(
                &device_cloned, 
                &queue_cloned,
                &nexon_lv2_gothic_bold, 
                &script_cloned, 
                &text_brush_cloned, 
            )?);
            notifications.append(&mut create_notify_texts(
                &device_cloned, 
                &queue_cloned, 
                &nexon_lv2_gothic_medium, 
                &script_cloned, 
                &text_brush_cloned
            )?);
            let logo = create_logo_image(
                &device_cloned, 
                &queue_cloned, 
                &tex_sampler_cloned, 
                &ui_brush_cloned, 
                &asset_bundle_cloned
            )?;

            Ok(IntroScene { 
                elapsed_time: 0.0, 
                state: state::IntroState::default(), 
                loading: None, 
                notifications, 
                logo 
            })
        }));
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
                    label: Some("RenderPass(IntroLoading)"),
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

impl Default for IntroLoading {
    #[inline]
    fn default() -> Self {
        Self { loading: None }
    }
}



/// #### 한국어 </br>
/// 게임 인트로를 보여주는 게임 장면입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene showing the game intro. </br>
/// 
#[derive(Debug)]
pub struct IntroScene {
    elapsed_time: f64,
    state: state::IntroState,
    loading: Option<JoinHandle<AppResult<()>>>,
    notifications: Vec<Section2d>,
    logo: UiObject,
}

impl SceneNode for IntroScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();

        let asset_bundle_cloned = asset_bundle.clone();
        self.loading = Some(thread::spawn(move || {
            // (한국어) `Title` 게임 장면에서 사용될 에셋들을 로드합니다.
            // (English Translation) Loads assets to be used in `Title` game scene. 
            asset_bundle_cloned.get(path::BACKGROUND_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::ARIS_STANDING_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::MOMOI_STANDING_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::TITLE_BUTTON_START_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::TITLE_BUTTON_SETTING_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::TITLE_BUTTON_EXIT_TEXTURE_PATH)?;
            asset_bundle_cloned.get(path::THEME64_SOUND_PATH)?;
            Ok(())
        }));
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



/// #### 한국어 </br>
/// 게임 장면에서 사용되는 알림 타이틀을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates notification title used in game scene. </br>
/// 
fn create_notify_title<F: Font>(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    font: &F, 
    script: &Script, 
    text_brush: &Text2dBrush
) -> AppResult<Section2d> {
    let text = script.get(ScriptTags::NotifyTitle)?;
    let notify_title = Section2dBuilder::new(
        Some("Notify Title"), 
        font, 
        text, 
        &text_brush
    )
    .with_anchor(Anchor::new(0.85, 0.4, 0.65, 0.6))
    .build(device, queue);

    return Ok(notify_title);
}



/// #### 한국어 </br>
/// 게임 장면에서 사용되는 알림 텍스트를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates notification text used in game scene. </br>
/// 
fn create_notify_texts<F: Font>(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    font: &F, 
    script: &Script, 
    text_brush: &Text2dBrush
) -> AppResult<Vec<Section2d>> {
    let text = script.get(ScriptTags::NotifyTextLine0)?;
    let notify_line0 = Section2dBuilder::new(
        Some("Notify Text Line0"), 
        font, 
        text, 
        &text_brush
    )
    .with_anchor(Anchor::new(0.575, 0.3, 0.425, 0.7 ))
    .build(device, queue);

    return Ok(vec![notify_line0]);
}



/// #### 한국어 </br>
/// 게임 장면에서 사용되는 로고 이미지를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates logo image used in game scene. </br>
/// 
fn create_logo_image(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    ui_brush: &UiBrush, 
    asset_bundle: &AssetBundle
) -> AppResult<UiObject> {
    // (한국어) 로고 텍스처를 생성합니다.
    // (English Translation) Create logo texture.
    let texture = asset_bundle.get(path::LOGO_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Logo"),
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 10,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor { 
            ..Default::default()
        }
    );

    // (한국어) 로고 이미지 사용자 인터페이스를 생성합니다.
    // (English Translation) Create logo image user interface.
    let texture = UiObjectBuilder::new(
        Some("Logo"),
        tex_sampler,
        &texture_view,
        &ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(206, -206, -206, 206))
    .with_color((1.0, 1.0, 1.0, 0.0).into())
    .build(device);

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::LOGO_TEXTURE_PATH);

    return Ok(texture);
}
