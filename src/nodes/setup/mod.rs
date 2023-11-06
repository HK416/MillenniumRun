use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;

use winit::window::Window;

use crate::{
    game_err,
    assets::{
        bundle::AssetBundle,
        handle::AssetHandle,
    },
    components::user::{
        Settings,
        SettingsDecoder,
        SettingsEncoder,
    },
    nodes::test::TestScene,
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
            thread::spawn(move || asset_bundle_cloned.get("user.settings"))
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

        // (한국어) 설정 파일 가져오기.
        // (English Translation) Get settings file.
        let mut settings = asset_bundle.get("user.settings")?
            .read_or_default::<Settings, SettingsDecoder, SettingsEncoder>()?;

        // (한국어) 애플리케이션 윈도우를 설정합니다.
        // (English Translation) Set the application window.
        set_window_title(window, settings.language);
        settings.resolution = set_window_size(window, settings.resolution)?;
        set_screen_mode(window, settings.screen_mode);

        // (한국어) 설정 파일을 갱신합니다.
        // (English Translation) Updates the settings file.
        asset_bundle.get("user.settings")?.write::<Settings, SettingsEncoder>(&settings)?;

        // (한국어) 설정 파일을 공유 객체에 등록합니다.
        // (English Translation) Register the settings file to the shared object.
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
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(TestScene::default()));
        } else {
            self.loading_assets = temp;
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
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("draw render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

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
