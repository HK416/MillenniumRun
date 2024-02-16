mod state;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use ab_glyph::FontArc;

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text::{TextBrush, Text, TextBuilder},
        ui::{UiBrush, UiObject, UiObjectBuilder},
        camera::CameraCreator,
        transform::Projection, 
        anchor::Anchor,
        margin::Margin,
        script::{Script, ScriptTags},
    },
    nodes::{path, consts::PIXEL_PER_METER},
    render::texture::DdsTextureDecoder, 
    scene::{node::SceneNode, state::SceneState},
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
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap().clone();
        let script = shared.get::<Arc<Script>>().unwrap().clone();
        let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap().clone();
        let textures = shared.get::<Arc<HashMap<String, wgpu::Texture>>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();

        self.loading = Some(thread::spawn(move || {
            // (한국어) 현재 게임 장면에서 사용할 에셋들을 로드합니다. 
            // (English Translation) Loads assets to be used in the current game scene.
            asset_bundle.get(path::LOGO_TEXTURE_PATH)?;
            asset_bundle.get(path::ARIS_TITLE_SOUND_PATH)?;
            asset_bundle.get(path::MOMOI_TITLE_SOUND_PATH)?;
            asset_bundle.get(path::MIDORI_TITLE_SOUND_PATH)?;
            asset_bundle.get(path::YUZU_TITLE_SOUND_PATH)?;

            // (한국어) 로고 이미지 텍스처를 생성합니다.
            // (English Translation) Create a logo image texture. 
            let logo_texture = asset_bundle.get(path::LOGO_TEXTURE_PATH)?
                .read(&DdsTextureDecoder {
                    name: Some("Logo"), 
                    size: wgpu::Extent3d {
                        width: 512, 
                        height: 512, 
                        depth_or_array_layers: 1, 
                    }, 
                    dimension: wgpu::TextureDimension::D2, 
                    format: wgpu::TextureFormat::Bc7RgbaUnorm, 
                    mip_level_count: 10, 
                    sample_count: 1, 
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
                    view_formats: &[], 
                    device: &device, 
                    queue: &queue
                })?;

            // (한국어) 사용을 완료한 에셋을 정리합니다.
            // (English Translation) Release assets that have been used.
            asset_bundle.release(path::LOGO_TEXTURE_PATH);

            let dummy_texture = textures.get(path::DUMMY_TEXTURE_PATH)
                .expect("A registered texture could not be found.");


            let nexon_lv2_gothic_bold = fonts.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)
                .expect("A registered font could not be found.");
            let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
                .expect("A registered font could not be found.");

            // (한국어) 게임 장면에서 사용할 객체들을 생성합니다.
            // (English Translation) Creates objects to be used in the game scene.
            let mut notifications = Vec::with_capacity(2);
            notifications.push(create_notify_title(
                &device, 
                &queue,
                &nexon_lv2_gothic_bold, 
                &script, 
                &text_brush, 
            )?);
            notifications.append(&mut create_notify_texts(
                &device, 
                &queue, 
                &nexon_lv2_gothic_medium, 
                &script, 
                &text_brush
            )?);
            let foreground = create_foreground(
                &device, 
                &tex_sampler, 
                &dummy_texture, 
                &ui_brush
            );
            let logo = create_logo_image(
                &device, 
                &tex_sampler, 
                &logo_texture, 
                &ui_brush
            );
            let loading_text = create_loading_text(
                nexon_lv2_gothic_medium, 
                &device, 
                &queue, 
                &text_brush
            );
            let application_info = create_application_info(
                nexon_lv2_gothic_bold, 
                &device, 
                &queue, 
                &text_brush
            );

            Ok(IntroScene { 
                timer: 0.0, 
                state: state::IntroState::default(), 
                application_info, 
                loading: None, 
                loading_text, 
                notifications, 
                foreground, 
                logo 
            })
        }));


        // (한국어) 게임 장면에서 사용할 카메라를 생성합니다.
        // (English Translation) Creates a camera to use in the current game scene. 
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap().clone();
        let camera = camera_creator.create(
            Some("Intro"), 
            None, 
            None, 
            Some(Projection::new_ortho(
                3.0 * PIXEL_PER_METER, 
                -4.0 * PIXEL_PER_METER, 
                -3.0 * PIXEL_PER_METER, 
                4.0 * PIXEL_PER_METER, 
                0.0 * PIXEL_PER_METER, 
                1000.0 * PIXEL_PER_METER
            )), 
            None
        );
        shared.push(Arc::new(camera));

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
            let mut _rpass = encoder.begin_render_pass(
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
    timer: f64,
    state: state::IntroState,
    loading: Option<JoinHandle<AppResult<()>>>,
    application_info: Text, 
    loading_text: Text, 
    notifications: Vec<Text>,
    foreground: UiObject, 
    logo: UiObject,
}

impl SceneNode for IntroScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        self.loading = Some(thread::spawn(move || {
            // (한국어) `Title` 게임 장면에서 사용될 에셋들을 로드합니다.
            // (English Translation) Loads assets to be used in `Title` game scene. 
            asset_bundle.get(path::CLICK_SOUND_PATH)?;
            asset_bundle.get(path::CANCEL_SOUND_PATH)?;
            asset_bundle.get(path::YUUKA_TITLE_SOUND_PATH)?;
            asset_bundle.get(path::YUUKA_HIDDEN_SOUND_PATH)?;
            asset_bundle.get(path::LOGO_TEXTURE_PATH)?;
            asset_bundle.get(path::STAR_TEXTURE_PATH)?;
            asset_bundle.get(path::TUTORIAL_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_WIDE_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_MEDIUM_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_INFO_TEXTURE_PATH)?;
            asset_bundle.get(path::BUTTON_RETURN_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_START_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_SETTING_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BUTTON_EXIT_TEXTURE_PATH)?;
            asset_bundle.get(path::TITLE_BACKGROUND_TEXTURE_PATH)?;
            asset_bundle.get(path::WINDOW_RATIO_4_3_TEXTURE_PATH)?;
            asset_bundle.get(path::WINDOW_RATIO_8_1_TEXTURE_PATH)?;
            asset_bundle.get(path::ARIS_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::MOMOI_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::MIDORI_STANDING_TEXTURE_PATH)?;
            asset_bundle.get(path::YUZU_STANDING_TEXTURE_PATH)?;
            
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
fn create_notify_title(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    font: &FontArc, 
    script: &Script, 
    text_brush: &TextBrush
) -> AppResult<Text> {
    let text = script.get(ScriptTags::IntroTitle)?;
    let notify_title = TextBuilder::new(
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
fn create_notify_texts(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    font: &FontArc, 
    script: &Script, 
    text_brush: &TextBrush
) -> AppResult<Vec<Text>> {
    let text = script.get(ScriptTags::IntroText)?;
    let notify_line0 = TextBuilder::new(
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
    tex_sampler: &wgpu::Sampler, 
    logo_texture: &wgpu::Texture, 
    ui_brush: &UiBrush
) -> UiObject {
    let texture_view = logo_texture.create_view(
        &wgpu::TextureViewDescriptor { 
            ..Default::default()
        }
    );

    // (한국어) 로고 이미지 사용자 인터페이스를 생성합니다.
    // (English Translation) Create logo image user interface.
    let logo = UiObjectBuilder::new(
        Some("Logo"),
        tex_sampler,
        &texture_view,
        &ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(206, -206, -206, 206))
    .with_color((18.0 / 255.0, 23.0 / 255.0, 40.0 / 255.0, 0.0).into())
    .build(device);

    return logo;
}



/// #### 한국어 </br>
/// 장면 전환에 사용할 전경 이미지를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a foreground image to use for scene transitions. </br>
/// 
fn create_foreground(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    dummy_texture: &wgpu::Texture, 
    ui_brush: &UiBrush
) -> UiObject {
    let texture_view = dummy_texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 전경 이미지 사용자 인터페이스를 생성합니다.
    // (English Translation) Create a foreground image user interface. 
    let foreground = UiObjectBuilder::new(
        Some("Foreground"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(1.0, 0.0, 0.0, 1.0))
    .with_color((0.0, 0.0, 0.0, 1.0).into())
    .build(device);

    return foreground;
}

/// #### 한국어 </br>
/// 로딩 텍스트를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a loading text. </br>
/// 
fn create_loading_text(
    font: &FontArc,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> Text {
    TextBuilder::new(
        Some("LoadingText"), 
        font, 
        "Loading", 
        text_brush
    )
    .with_anchor(Anchor::new(0.0, 1.0, 0.0, 1.0))
    .with_margin(Margin::new(128, -256, 0, 0))
    .with_color((0.0, 0.0, 0.0, 1.0).into())
    .build(device, queue)
}

/// #### 한국어 </br>
/// 애플리케이션 빌드 정보 텍스트를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a application build information text. </br>
/// 
fn create_application_info(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> Text {
    use crate::system::APPLICATION_INFORMATION;
    TextBuilder::new(
        Some("ApplicationInformationText"), 
        font, 
        APPLICATION_INFORMATION, 
        text_brush
    )
    .with_anchor(Anchor::new(0.0, 0.5, 0.0, 0.5))
    .with_margin(Margin::new(32, -400, 0, 400))
    .with_color((0.0, 0.0, 0.0, 1.0).into())
    .with_translation((0.0, 0.0, 0.1).into())
    .build(device, queue)
}
