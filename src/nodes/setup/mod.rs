#[cfg(debug_assertions)]
mod parser;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use ab_glyph::FontArc;
use winit::window::Window;
use rodio::{OutputStream, OutputStreamHandle};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        ui::UiBrush,
        text::TextBrush,
        sprite::SpriteBrush,
        camera::CameraCreator,
        font::FontDecoder,
        script::{Script, ScriptDecoder},
        user::{Language, Settings, SettingsEncoder, SettingsDecoder},
    },
    nodes::{
        path, 
        intro::IntroLoading,
        first_time::FirstTimeSetupLoading, 
    },
    render::texture::DdsTextureDecoder, 
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
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
    
        // (한국어) 게임에서 사용되는 에셋 파일들을 로드합니다. 
        // (English Translation) Load asset files used in the game. 
        self.loading = Some(thread::spawn(move || {
            asset_bundle.get(path::SETTINGS_PATH)?;

            asset_bundle.get(path::DUMMY_TEXTURE_PATH)?;

            asset_bundle.get(path::NEXON_LV2_GOTHIC_PATH)?;
            asset_bundle.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)?;
            asset_bundle.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)?;

            asset_bundle.get(path::UI_SHADER_PATH)?;
            asset_bundle.get(path::UI_TEXT_SHADER_PATH)?;
            asset_bundle.get(path::SPRITE_SHADER_PATH)?;

            asset_bundle.get(path::CLICK_SOUND_PATH)?;
            asset_bundle.get(path::CANCEL_SOUND_PATH)?;

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
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
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
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        ));

        let fonts = setup_fonts(asset_bundle)?;
        let (stream, handle) = setup_sound_engine()?;
        let camera_creator = CameraCreator::new(device.clone(), window.clone());
        let ui_brush = setup_ui_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        let text_brush = setup_text_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        let sprite_brush = setup_sprite_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        let textures = setup_texture_map(device, queue, asset_bundle)?;
        let (settings, script) = setup_window(window, asset_bundle)?;

        // (한국어) 공유할 객체들을 공유 객체에 등록합니다.
        // (English Translation) Register objects to be shared as shared objects.
        shared.push(tex_sampler);
        shared.push(fonts);
        shared.push(stream);
        shared.push(handle);
        shared.push(camera_creator);
        shared.push(text_brush);
        shared.push(ui_brush);
        shared.push(sprite_brush);
        shared.push(textures);
        shared.push(settings);
        if let Some(script) = script {
            shared.push(Arc::new(script));
        };

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _: f64, _: f64) -> AppResult<()> {
        // (한국어) 모든 에셋 파일의 로드가 완료되었는지 확인합니다.
        // (English Translation) Verify that all asset files have completed loading.
        if self.loading.as_ref().is_some_and(|it| it.is_finished()) {
            self.loading.take().unwrap().join().unwrap()?;
            
            #[cfg(debug_assertions)] {
                // (한국어) 주어진 명령줄을 구문분석 합니다.
                // (English Translation) 주어진 명령줄을 구문분석 합니다.
                let config = parser::parse_command_lines();
                if let Some(next_scene) = config.next_scene {
                    // (한국어) 다음 장면이 설정되어 있는 경우 다음 장면으로 변경합니다.
                    // (English Translation) If the next scene is set, change to the next scene.
                    *shared.get_mut().unwrap() = SceneState::Change(next_scene);
                    
                    // (한국어) 설정된 언어의 스크립트 파일을 불러옵니다.
                    // (English Translation) Loads the script file of the set language.
                    let asset_bundle = shared.get::<AssetBundle>().unwrap();
                    let rel_path = match config.language {
                        Language::Korean | Language::Unknown => path::KOR_SCRIPTS_PATH,
                    };
                    let script = asset_bundle.get(rel_path)?.read(&ScriptDecoder)?;
                    shared.push(Arc::new(script));
                    return  Ok(());
                } 
            }

            // (한국어) 사용할 공유 객체 가져오기.
            // (English Translation) Get shared object to use.
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let settings = asset_bundle.get(path::SETTINGS_PATH)?
                .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

            // (한국어) 다음 장면을 설정합니다.
            // (English Translation) Sets the next game scene.
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(match settings.language {
                Language::Unknown => Box::new(FirstTimeSetupLoading::default()),
                _ => Box::new(IntroLoading::default()),
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
/// 게임에서 사용할 폰트를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set the font to use in the game. </br>
/// 
fn setup_fonts(asset_bundle: &AssetBundle) -> AppResult<Arc<HashMap<String, FontArc>>> {
    // (한국어) 폰트 에셋 가져오기.
    // (English Translation) Gets the font assets.
    let nexon_lv2_gothic_bold = asset_bundle.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)?
        .read(&FontDecoder)?;
    let nexon_lv2_gothic_medium = asset_bundle.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)?
        .read(&FontDecoder)?;
    let nexon_lv2_gothic = asset_bundle.get(path::NEXON_LV2_GOTHIC_PATH)?
        .read(&FontDecoder)?;

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::NEXON_LV2_GOTHIC_PATH);
    asset_bundle.release(path::NEXON_LV2_GOTHIC_MEDIUM_PATH);
    asset_bundle.release(path::NEXON_LV2_GOTHIC_BOLD_PATH);

    return Ok(HashMap::from_iter([
        (path::NEXON_LV2_GOTHIC_BOLD_PATH.to_string(), nexon_lv2_gothic_bold),
        (path::NEXON_LV2_GOTHIC_MEDIUM_PATH.to_string(), nexon_lv2_gothic_medium),
        (path::NEXON_LV2_GOTHIC_PATH.to_string(), nexon_lv2_gothic),
    ]).into());
}


/// #### 한국어 </br>
/// 사용자 인터페이스 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets user interface drawing tools. </br>
/// 
fn setup_ui_brush(
    device: &wgpu::Device,
    camera_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat,
    asset_bundle: &AssetBundle
) -> AppResult<Arc<UiBrush>> {
    UiBrush::new(
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
/// 텍스트 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets text drawing tools. </br>
/// 
fn setup_text_brush(
    device: &wgpu::Device,
    camera_layout: &wgpu::BindGroupLayout,
    render_format: wgpu::TextureFormat,
    asset_bundle: &AssetBundle
) -> AppResult<Arc<TextBrush>> {
    TextBrush::new(
        device, 
        &camera_layout,
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
/// 스프라이트 그리기 도구를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Sets sprite drawing tools. </br>
/// 
fn setup_sprite_brush(
    device: &wgpu::Device, 
    camera_layout: &wgpu::BindGroupLayout, 
    render_format: wgpu::TextureFormat, 
    asset_bundle: &AssetBundle
) -> AppResult<Arc<SpriteBrush>> {
    let sprite_brush = SpriteBrush::new(
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
    )?;

    return Ok(sprite_brush.into());
}


/// #### 한국어 </br>
/// 텍스처 캐시를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Set up the texture cache. </br>
/// 
fn setup_texture_map(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    asset_bundle: &AssetBundle
) -> AppResult<Arc<HashMap<String, wgpu::Texture>>> {
    // (한국어) 더미 텍스처를 생성합니다.
    // (English Translation) Create a dummy texture.
    let dummy = asset_bundle.get(path::DUMMY_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Dummy"), 
            size: wgpu::Extent3d {
                width: 1, 
                height: 1, 
                depth_or_array_layers: 1, 
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 1, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue
        })?;

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation)  Release assets that have been used. 
    asset_bundle.release(path::DUMMY_TEXTURE_PATH);

    return Ok(HashMap::from_iter([
        (path::DUMMY_TEXTURE_PATH.to_string(), dummy), 
    ]).into());
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
fn setup_window(window: &Window, asset_bundle: &AssetBundle) -> AppResult<(Settings, Option<Script>)> {
    use crate::components::user::{
        set_window_size,
        set_screen_mode,
    };

    // (한국어) 설정 파일 가져오기.
    // (English Translation) Get settings file.
    let mut settings = asset_bundle.get(path::SETTINGS_PATH)?
        .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

    // (한국어) 설정된 언어의 스크립트 파일을 불러옵니다.
    // (English Translation) Loads the script file of the set language.
    let script = match settings.language {
        Language::Korean => Some(asset_bundle.get(path::KOR_SCRIPTS_PATH)?.read(&ScriptDecoder)?),
        Language::Unknown => None,
    };

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

    return Ok((settings, script));
}
