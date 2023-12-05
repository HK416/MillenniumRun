mod res;

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use winit::window::Window;
use rodio::{OutputStream, OutputStreamHandle};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        sprite::brush::SpriteBrush,
        text::{
            brush::TextBrush, 
            font::{FontSet, FontDecoder}
        },
        ui::brush::UiBrush,
        camera::{self, Viewport, GameCamera},
        transform::{Transform, Projection},
        user::{Language, Settings, SettingsEncoder, SettingsDecoder},
    },
    nodes::{
        consts, path, 
        intro::{self, IntroScene},
        first_time::{self, FirstTimeSetupScene}, 
    },
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// 사용자가 애플리케이션을 시작할 때 진입하는 게임 장면입니다. </br>
/// 에셋을 로드하고 다음 게임 장면으로 전환합니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the game scene that enters when user start the application. </br>
/// Load assets and change to the next game scene. </br>
/// 
#[derive(Debug)]
pub struct SetupScene {
    loading: Option<JoinHandle<AppResult<()>>>,
}

impl SceneNode for SetupScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
    
        // (한국어) 게임에서 사용되는 에셋 파일들을 로드합니다. 
        // (English Translation) Load asset files used in the game. 
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading = Some(thread::spawn(move || {
            // (한국어) 모든 게임 장면에서 사용되는 에셋들을 로드합니다.
            // (English Translation) Loads assets used in all game scenes.
            for rel_path in res::ASSETS {
                asset_bundle_cloned.get(rel_path)?;
            }

            // (한국어) `FirstTimeSetup` 게임 장면에서 사용되는 에셋들을 로드합니다.
            // (English Translation) Loads asset used in the `FirstTimeSetup` game scene.
            for rel_path in first_time::res::ASSETS {
                asset_bundle_cloned.get(rel_path)?;
            }

            // (한국어) `Intro` 게임 장면에서 사용되는 에셋들을 로드합니다.
            // (English Translation) Loads asset used in the `Intro` game scene.
            for rel_path in intro::res::ASSETS {
                asset_bundle_cloned.get(rel_path)?;
            }
    
            Ok(())
        }));

        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let window = shared.get::<Arc<Window>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();

        // (한국어) 공용으로 사용하는 텍스처 샘플러를 생성합니다.
        // (English Translation) Creates a commonly used texture sampler.
        let tex_sampler = Arc::new(device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Sampler(General)"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                min_filter: wgpu::FilterMode::Linear,
                mag_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        ));

        let camera_bind_group_layout = camera::create_camera_bind_group_layout(device);
        let camera = setup_main_camera(window, device, &camera_bind_group_layout);        
        let font_set = setup_fonts(asset_bundle)?;
        let ui_brush = setup_ui_brush(device, &camera_bind_group_layout, config.format, asset_bundle)?;
        let text_brush = setup_text_brush(device, &camera_bind_group_layout, config.format, asset_bundle)?;
        let sprite_brush = setup_sprite_brush(device, &camera_bind_group_layout, config.format, asset_bundle)?;
        let (stream, handle) = setup_sound_engine()?;
        let settings = setup_window(window, asset_bundle)?;

        // (한국어) 공유할 객체들을 공유 객체에 등록합니다.
        // (English Translation) Register objects to be shared as shared objects.
        shared.push(camera);
        shared.push(font_set);
        shared.push(sprite_brush);
        shared.push(text_brush);
        shared.push(ui_brush);
        shared.push(stream);
        shared.push(handle);
        shared.push(settings);
        shared.push(tex_sampler);

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _: f64, _: f64) -> AppResult<()> {
        // (한국어) 모든 에셋 파일의 로드가 완료되었는지 확인합니다.
        // (English Translation) Verify that all asset files have completed loading.
        if self.loading.as_ref().unwrap().is_finished() {
            self.loading.take().unwrap().join().unwrap()?;

            // (한국어) 사용할 공유 객체 가져오기.
            // (English Translation) Get shared object to use.
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let settings = asset_bundle.get(path::SETTINGS_PATH)?
                .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

            // (한국어) 다음 장면을 설정합니다.
            // (English Translation) Sets the next game scene.
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(match settings.language {
                Language::Unknown => Box::new(FirstTimeSetupScene::default()),
                _ => Box::new(IntroScene::default()),
            });
        }

        Ok(())
    }
}

impl Default for SetupScene {
    #[inline]
    fn default() -> Self {
        Self { loading: None }
    }
}



/// #### 한국어 </br>
/// 메인 카메라를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set up the main camera. </br>
/// 
fn setup_main_camera(
    window: &Window,
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout
) -> GameCamera {
    let scale_factor = window.current_monitor()
        .map_or(1.0, |handle| handle.scale_factor() as f32);
    GameCamera::new(
        Some("Main"), 
        Viewport::default(), 
        Transform::default(), 
        Projection::new_ortho(
            4.5 * consts::PIXEL_PER_METER, 
            -8.0 * consts::PIXEL_PER_METER, 
            -4.5 * consts::PIXEL_PER_METER, 
            8.0 * consts::PIXEL_PER_METER, 
            0.0 * consts::PIXEL_PER_METER, 
            1000.0 * consts::PIXEL_PER_METER 
        ),
        scale_factor, 
        device, 
        layout
    )
}


/// #### 한국어 </br>
/// 게임에서 사용할 폰트를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set the font to use in the game. </br>
/// 
fn setup_fonts(asset_bundle: &AssetBundle) -> AppResult<FontSet> {
    // (한국어) 폰트 에셋 가져오기.
    // (English Translation) Gets the font assets.
    let font_regular = asset_bundle.get(path::FONT_MEDIUM_PATH)?
        .read(&FontDecoder)?;
    let font_bold = asset_bundle.get(path::FONT_BLOD_PATH)?
        .read(&FontDecoder)?;
    let font_set = FontSet::from([
        (path::FONT_MEDIUM_PATH.to_string(), font_regular),
        (path::FONT_BLOD_PATH.to_string(), font_bold),
    ]);

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::FONT_MEDIUM_PATH);
    asset_bundle.release(path::FONT_BLOD_PATH);

    return Ok(font_set);
}


/// #### 한국어 </br>
/// 사용자 인터페이스 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets user interface drawing tools. </br>
/// 
fn setup_ui_brush(
    device: &wgpu::Device,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat,
    asset_bundle: &AssetBundle
) -> AppResult<UiBrush> {
    UiBrush::new(
        device, 
        camera_bind_group_layout, 
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
/// 텍스트 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets text drawing tools. </br>
/// 
fn setup_text_brush(
    device: &wgpu::Device,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat,
    asset_bundle: &AssetBundle
) -> AppResult<TextBrush> {
    TextBrush::new(
        device, 
        &camera_bind_group_layout,
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
    )
}


/// #### 한국어 </br>
/// 스프라이트 이미지 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets sprite image drawing tools. </br>
/// 
fn setup_sprite_brush(
    device: &wgpu::Device,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat,
    asset_bundle: &AssetBundle
) -> AppResult<SpriteBrush> {
    SpriteBrush::new(
        &device,
        &camera_bind_group_layout,
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
    )
}


/// #### 한국어 </br>
/// 게임에서 사용할 사운드 엔진을 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set the sound engine to use in the game. </br>
/// 
fn setup_sound_engine() -> AppResult<(OutputStream, OutputStreamHandle)> {
    let (stream, handle) = OutputStream::try_default()
        .map_err(|err| game_err!(
            "Sound engine initialization failed",
            "Sound engine initialization failed for following reasons: {}",
            err.to_string()
        ))?;
    return Ok((stream, handle));
}


/// #### 한국어 </br>
/// 사용자 설정 파일을 불러오고, 윈도우를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Load the user settings file and configure Windows. </br>
/// 
fn setup_window(window: &Window, asset_bundle: &AssetBundle) -> AppResult<Settings> {
    use crate::components::user::{
        set_window_size,
        set_screen_mode,
    };

    // (한국어) 설정 파일 가져오기.
    // (English Translation) Get settings file.
    let mut settings = asset_bundle.get(path::SETTINGS_PATH)?
        .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

    // (한국어) 애플리케이션 윈도우를 설정합니다.
    // (English Translation) Set the application window.
    settings.resolution = set_window_size(window, settings.resolution)?;
    set_screen_mode(window, settings.screen_mode);
    window.set_title(match settings.language {
        Language::Unknown => "Select a language",
        _ => "Millennium Run",
    });
    window.set_visible(true);

    // (한국어) 설정 파일을 갱신합니다.
    // (English Translation) Updates the settings file.
    asset_bundle.get(path::SETTINGS_PATH)?.write(&SettingsEncoder, &settings)?;

    return Ok(settings);
}
