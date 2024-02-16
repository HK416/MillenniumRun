#[cfg(debug_assertions)]
mod parser;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use ab_glyph::FontArc;
use winit::window::Window;

use crate::{
    assets::{
        bundle::AssetBundle, 
        interface::AssetDecoder, 
    },
    components::{
        ui::UiBrush,
        text::TextBrush,
        sprite::SpriteBrush,
        camera::CameraCreator,
        sound, 
        font::FontDecoder,
        script::{Script, ScriptDecoder},
        save::{SaveDecoder, SaveEncoder},
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
        error::AppResult,
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
            asset_bundle.get(path::SAVE_PATH)?;
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
        // (한국어) 공용으로 사용하는 텍스처 샘플러를 생성합니다.
        // (English Translation) Creates a commonly used texture sampler.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
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
        shared.push(tex_sampler);

        // (한국어) 애플리케이션에서 사용할 폰트를 로드하고 공유 객체에 등록합니다.
        // (English Translation) Loads the font to be used in the application and registers it in the shared object.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let fonts = setup_fonts(asset_bundle)?;
        shared.push(fonts);

        // (한국어) 기본 소리 출력 장치가 존재하는 경우, 공유 객체에 등록합니다.
        // (English Translation) If the default sound output device exists, register it with the shared object.
        if let Some((stream, handle)) = sound::get_default_output_stream()? {
            shared.push((stream, handle));
        }

        // (한국어) 기본 카메라를 생성하고 공유 객체에 등록합니다.
        // (English Translation) Create a basic camera and register it with the shared object.
        let window = shared.get::<Arc<Window>>().unwrap().clone();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let creator = CameraCreator::new(device, window);
        let camera = Arc::new(creator.create(Some("Default"), None, None, None, None));
        shared.push(creator);
        shared.push(camera);

        // (한국어) 인터페이스 그리기 도구를 생성하고 공유 객체에 등록합니다.
        // (English Translation) Creates an interface drawing tool and registers it with the shared object.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let ui_brush = setup_ui_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        shared.push(ui_brush);

        // (한국어) 텍스트 그리기 도구를 생성하고 공유 객체에 등록합니다.
        // (English Translation) Creates an text drawing tool and registers it with the shared object.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let text_brush = setup_text_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        shared.push(text_brush);

        // (한국어) 스프라이트 그리기 도구를 생성하고 공유 객체에 등록합니다.
        // (English Translation) Creates an sprite drawing tool and registers it with the shared object. 
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
        let camera_creator = shared.get::<Arc<CameraCreator>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let sprite_brush = setup_sprite_brush(device, &camera_creator.camera_layout, config.format, asset_bundle)?;
        shared.push(sprite_brush);

        // (한국어) 미리 로드된 텍스처 모음을 생성하고 공유 객체에 등록합니다.
        // (English Translation) Creates a collection of preloaded textures and registers them with a shared object. 
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let texture_map = setup_texture_map(device, queue, asset_bundle)?;
        shared.push(texture_map);

        // (한국어) 애플리케이션 윈도우를 설정하고 윈도우 설정과 언어 스크립트를 공유 객체에 등록합니다.
        // (English Translation) Sets up an application window and registers window settings and language scripts to shared objects.
        let window = shared.get::<Arc<Window>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let (settings, script) = setup_window(window, asset_bundle)?;
        shared.push(settings);
        if let Some(script) = script {
            shared.push(Arc::new(script));
        }

        // (한국어) 사용자 데이터가 저장된 파일을 불러오고 공유 객체에 등록합니다.
        // (English Translation) Load the file where user data is stored and register it as a shared object.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let savedata = asset_bundle.get(path::SAVE_PATH)?.read_or_default(&SaveEncoder, &SaveDecoder)?;
        shared.push(savedata);

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
    asset_bundle.release(path::DUMMY_TEXTURE_PATH);

    // (한국어) 기본 스테이지 이미지 텍스처를 생성합니다.
    // (English Translation) Create an default stage image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/default.dds"));
    let default_img = DdsTextureDecoder {
        name: Some("DefaultImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 1,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;

    // (한국어) Aris 이미지 텍스처를 생성합니다.
    // (English Translation) Create an Aris image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/aris.dds"));
    let aris_img = DdsTextureDecoder {
        name: Some("ArisImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 3,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;

    // (한국어) Momoi 이미지 텍스처를 생성합니다.
    // (English Translation) Create an Momoi image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/momoi.dds"));
    let momoi_img = DdsTextureDecoder {
        name: Some("MomoiImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 3,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;

    // (한국어) Midori 이미지 텍스처를 생성합니다.
    // (English Translation) Create an Midori image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/midori.dds"));
    let midori_img = DdsTextureDecoder {
        name: Some("MidoriImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 3,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;

    // (한국어) Yuzu 이미지 텍스처를 생성합니다.
    // (English Translation) Create an Yuzu image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/yuzu.dds"));
    let yuzu_img = DdsTextureDecoder {
        name: Some("YuzuImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 3,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;

    // (한국어) Yuuka 이미지 텍스처를 생성합니다.
    // (English Translation) Create an Yuuka image texture.
    let img_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/textures/img/yuuka.dds"));
    let yuuka_img = DdsTextureDecoder {
        name: Some("YuukaImage"), 
        size: wgpu::Extent3d {
            width: 2048, 
            height: 2048, 
            depth_or_array_layers: 1,
        }, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Bc7RgbaUnorm, 
        mip_level_count: 12, 
        sample_count: 1, 
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
        view_formats: &[], 
        device, 
        queue
    }.decode(img_data)?;
    
    return Ok(HashMap::from_iter([
        (path::DUMMY_TEXTURE_PATH.to_string(), dummy), 
        (path::DEF_IMG_TEXTURE_PATH.to_string(), default_img), 
        (path::ARIS_IMG_TEXTURE_PATH.to_string(), aris_img), 
        (path::MOMOI_IMG_TEXTURE_PATH.to_string(), momoi_img),
        (path::MIDORI_IMG_TEXTURE_PATH.to_string(), midori_img),
        (path::YUZU_IMG_TEXTURE_PATH.to_string(), yuzu_img),
        (path::YUUKA_IMG_TEXTURE_PATH.to_string(), yuuka_img),
    ]).into());
}

/// #### 한국어 </br>
/// 사용자 설정 파일을 불러오고, 윈도우를 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Load the user settings file and configure Windows. </br>
/// 
fn setup_window(window: &Window, asset_bundle: &AssetBundle) -> AppResult<(Settings, Option<Script>)> {
    use crate::components::user::set_window_size;

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
