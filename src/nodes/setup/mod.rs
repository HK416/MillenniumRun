use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;

use winit::window::Window;
use rodio::{OutputStream, Sink};

use crate::{
    game_err,
    assets::{
        bundle::AssetBundle,
        handle::AssetHandle,
    },
    components::{
        text::{
            brush::TextBrush,
            font::FontDecoder,
        },
        ui::brush::UiBrush,
        user::{
            Language,
            SettingsDecoder,
            SettingsEncoder,
        },
    },
    nodes::{
        consts,
        path,
        entry::EntryScene,
        first_time::FirstTimeSetupScene,
    },
    scene::{
        node::SceneNode,
        state::SceneState,
    },
    system::{
        error::{
            AppResult,
            GameError,
        },
        shared::Shared,
    },
};



type Loading = JoinHandle<AppResult<AssetHandle>>;

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
    loading_assets: VecDeque<Loading>,
    num_loading_assets: usize,
}

impl SceneNode for SetupScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();

        // (한국어) 설정 파일 불러오기.
        // (English Translation) Load settings file.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::SETTINGS_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 폰트 파일 불러오기.
        // (English Translation) Load font file.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::FONT_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 폰트 쉐이더 파일 불러오기.
        // (English Translation) Load font shader file.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::FONT_SHADER_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 유저 인터페이스 쉐이더 파일 불러오기.
        // (English Translation) Load the user interface shader file.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::UI_SHADER_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 버튼 텍스처 불러오기.
        // (English Translation) Load button texture.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::BUTTON_TEXTURE_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 로고 텍스처 불러오기.
        // (English Translation) Load logo texture.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::LOGO_TEXTURE_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) 클릭음 불러오기.
        // (English Translation) Load click sound.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::CLICK_SOUND_PATH))
        );
        self.num_loading_assets += 1;


        // (한국어) Yuzu 타이틀 소리 불러오기.
        // (English Translation) Load Yuzu title sound.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::YUZU_TITLE_SOUND_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) Aris 타이틀 소리 불러오기.
        // (English Translation) Load Aris title sound.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::ARIS_TITLE_SOUND_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) Momoi 타이틀 소리 불러오기.
        // (English Translation) Load Momoi title sound.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::MOMOI_TITLE_SOUND_PATH))
        );
        self.num_loading_assets += 1;

        // (한국어) Midori 타이틀 소리 불러오기.
        // (English Translation) Load Midori title sound.
        let asset_bundle_cloned = asset_bundle.clone();
        self.loading_assets.push_back(
            thread::spawn(move || asset_bundle_cloned.get(path::MIDORI_TITLE_SOUND_PATH))
        );
        self.num_loading_assets += 1;


        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::components::user::{
            set_window_title,
            set_window_size,
            set_screen_mode,
        };

        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let window = shared.get::<Arc<Window>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();

        // (한국어) 폰트 에셋 가져오기.
        // (English Translation) Gets the font assets.
        let font = asset_bundle.get(path::FONT_PATH)?
            .read(&FontDecoder)?;

        // (한국어) 텍스트 브러쉬 생성하기.
        // (English Translation) Create a text brush.
        let text_brush = TextBrush::new(
            device, 
            config.format, 
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
            consts::VIEW_ORTHO,
        )?;

        // (한국어) 유저 인터페이스 브러쉬 생성하기.
        // (English Translation) Create a user interface brush.
        let ui_brush = UiBrush::new(
            device,
            config.format,
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
            consts::VIEW_ORTHO
        )?;

        // (한국어) 사운드 엔진 초기화.
        // (English Translation) Sound engine initialization.
        let (stream, handle) = OutputStream::try_default()
            .map_err(|err| game_err!(
                "Sound engine initialization failed",
                "Sound engine initialization failed for following reasons: {}",
                err.to_string()
            ))?;
        let sink = Arc::new(
            Sink::try_new(&handle)
                .map_err(|err| game_err!(
                    "Sound engine initialization failed",
                    "Sound engine initialization failed for following reasons: {}",
                    err.to_string()
                ))?
        );

        // (한국어) 설정 파일 가져오기.
        // (English Translation) Get settings file.
        let mut settings = asset_bundle.get(path::SETTINGS_PATH)?
            .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

        // (한국어) 애플리케이션 윈도우를 설정합니다.
        // (English Translation) Set the application window.
        set_window_title(window, settings.language);
        settings.resolution = set_window_size(window, settings.resolution)?;
        set_screen_mode(window, settings.screen_mode);
        window.set_visible(true);

        // (한국어) 설정 파일을 갱신합니다.
        // (English Translation) Updates the settings file.
        asset_bundle.get(path::SETTINGS_PATH)?.write(&SettingsEncoder, &settings)?;


        // (한국어) 사용을 완료한 에셋을 정리합니다.
        // (English Translation) Release assets that have been used.
        asset_bundle.release(path::FONT_PATH);
        asset_bundle.release(path::FONT_SHADER_PATH);
        asset_bundle.release(path::UI_SHADER_PATH);

        // (한국어) 공유할 객체들을 공유 객체에 등록합니다.
        // (English Translation) Register objects to be shared as shared objects.
        shared.push(font);
        shared.push(text_brush);
        shared.push(ui_brush);
        shared.push(stream);
        shared.push(sink);
        shared.push(settings);

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _: f64, _: f64) -> AppResult<()> {
        // (한국어) 모든 에셋 파일의 로드가 완료되었는지 확인합니다.
        // (English Translation) Verify that all asset files have completed loading.
        let mut temp = VecDeque::with_capacity(self.loading_assets.len());
        while let Some(loading) = self.loading_assets.pop_front() {
            if loading.is_finished() {
                loading.join().unwrap()?;
            } else {
                temp.push_back(loading);
            }
        }

        // (한국어) 모든 에셋 파일이 로드될 경우 다음 게임 장면으로 변경합니다.
        // (English Translation) Once all asset files are loaded, change to the next game scene.
        if temp.is_empty() {
            // (한국어) 사용자 설정을 불러옵니다.
            // (English Translation) Load user settings.
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let settings = asset_bundle.get(path::SETTINGS_PATH)?
                .read_or_default(&SettingsEncoder, &SettingsDecoder)?;

            // (한국어) 다음 장면을 설정합니다.
            // (English Translation) Sets the next game scene.
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(match settings.language {
                Language::Unknown => Box::new(FirstTimeSetupScene::default()),
                Language::Korean => Box::new(EntryScene::default()),
            });
        } else {
            self.loading_assets = temp;
        }

        Ok(())
    }
}

impl Default for SetupScene {
    #[inline]
    fn default() -> Self {
        Self { 
            loading_assets: VecDeque::new(), 
            num_loading_assets: 0 
        }
    }
}
