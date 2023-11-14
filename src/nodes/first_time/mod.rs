mod ui;

use std::sync::Arc;

use glam::Vec3;
use rodio::Sink;
use ab_glyph::FontArc;
use winit::{
    event::{
        Event, 
        WindowEvent, 
        MouseButton
    }, 
    window::Window, 
    dpi::PhysicalPosition
};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        text::{
            section::{
                Align,
                Section,
                SectionBuilder,
            }, 
            brush::TextBrush,
        },
        ui::{
            brush::UiBrush, 
            objects::{
                UiObject, 
                UiButtonObject,
            },
        }, 
        margin::Margin, 
        anchor::Anchor, 
        sound::SoundDecoder,
        user::{
            Language, 
            Settings, 
            SettingsEncoder, 
        }, 
    },
    nodes::{
        consts,
        path,
        entry::EntryScene,
    },
    render::{
        depth::DepthBuffer,
        texture::DdsImageDecoder,
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
        event::AppEvent,
        shared::Shared,
    }, 
};

const ANIMATION_TIME: f64 = 0.5;
const DEF_BTN_SCALE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
const MAX_BTN_SCALE: Vec3 = Vec3 { x: 1.25, y: 1.25, z: 1.0 };


/// #### 한국어 </br>
/// 애플리케이션이 처음 실행될 때 애플리케이션 언어를 설정하는 게임 장면입니다. </br>
/// 
/// #### English Translation
/// A game scene that sets the application language when the application first runs. </br>
/// 
#[derive(Debug)]
pub struct FirstTimeSetupScene { 
    btn_kor: Option<ui::UiButton>,
    btn_kor_text: Option<Section>,
    select_language: Language,
    run_animation: bool,
    animation_time: f64,
}

impl SceneNode for FirstTimeSetupScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let font = shared.get::<FontArc>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let ui_brush = shared.get::<UiBrush>().unwrap();

        // (한국어) 버튼의 텍스처, 텍스처 뷰, 텍스처 샘플러를 생성합니다.
        // (English Translation) Create a texture, texture view, and texture sampler for the button.
        let btn_texture = asset_bundle.get(path::BUTTON_TEXTURE_PATH)?
            .read(&DdsImageDecoder::new(
                Some("Button - Texture"),
                &device,
                &queue
            ))?;
        let btn_texture_view = btn_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let btn_tex_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Button - Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                min_filter: wgpu::FilterMode::Linear,
                mag_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        // (한국어) 한국어 선택 버튼을 생성합니다.
        // (English Translation) Create Korean selection button.
        let btn_kor = ui::UiButtonBuilder::new(
            Some("Korean Button"), 
            &btn_tex_sampler, 
            &btn_texture_view, 
            ui_brush.ref_bind_group_layout()
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(30, -120, -30, 120))
        .with_depth(-1.0)
        .build(device);

        // (한국어) 한국어 선택 버튼의 텍스트를 생성합니다.
        // (English Translation) Create text for the Korean selection button.
        let btn_kor_text = SectionBuilder::new(
            font,
            48.0,
            "한국어"
        )
        .with_align(Align::Center((0.0, 0.0).into()))
        .build(device);


        self.btn_kor = Some(btn_kor);
        self.btn_kor_text = Some(btn_kor_text);

        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        use crate::components::user::set_window_title;

        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let mut settings = shared.pop::<Settings>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let window = shared.get::<Arc<Window>>().unwrap();

        // (한국어) 설정의 내용을 갱신합니다.
        // (English Translation) Update the contents of the settings.
        settings.language = self.select_language;
        asset_bundle.get(path::SETTINGS_PATH)?
            .write(&SettingsEncoder, &settings)?;

        // (한국어) 윈도우 제목을 설정합니다.
        // (English Translation) Set the window title.
        set_window_title(window, self.select_language);

        shared.push(settings);

        Ok(())
    }

    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let sink = shared.get::<Arc<Sink>>().unwrap();
        let window = shared.get::<Arc<Window>>().unwrap();
        let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
        let buttons = [
            (self.btn_kor.as_ref().unwrap(), Language::Korean)
        ];

        if !self.run_animation {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::MouseInput { state, button, .. } => {
                        if MouseButton::Left == button && state.is_pressed() {
                            for (button, language) in buttons {
                                if button.mouse_pressed(
                                    cursor_pos.x as f32 / window.inner_size().width as f32, 
                                    cursor_pos.y as f32 / window.inner_size().height as f32, 
                                    &consts::VIEW_ORTHO
                                ) {
                                    // (한국어) 버튼 클릭 소리를 재생합니다.
                                    // (English Translation) Play a button click sound.
                                    let sound = asset_bundle.get(path::CLICK_SOUND_PATH)?
                                        .read(&SoundDecoder)?;
                                    sink.append(sound);

                                    // (한국어) 선택한 언어를 설정한 후 버튼 애니메이션을 재생합니다.
                                    // (English Translation) After setting the selected language, play the button animation.
                                    self.select_language = language;
                                    self.run_animation = true;
                                    break;
                                }
                            }
                        }
                    },
                    _ => { }
                },
                _ => { }
            };
        };

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _: f64, elapsed_time: f64) -> AppResult<()> {
        const MAX_ANIMATION_TIME: f64 = 1.0;
        if self.run_animation {
            self.animation_time += elapsed_time;
            if self.animation_time >= MAX_ANIMATION_TIME {
                *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(EntryScene::default()));
                return Ok(());
            }

            let buttons = [
                (self.btn_kor.as_mut().unwrap(), Language::Korean),
            ];
            let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
            let delta_time = (self.animation_time.min(ANIMATION_TIME) / ANIMATION_TIME) as f32;
            let scale = DEF_BTN_SCALE + (MAX_BTN_SCALE - DEF_BTN_SCALE) * delta_time;
            let alpha = 1.0 - 1.0 * delta_time;
            for (button, language) in buttons {
                if language == self.select_language {
                    button.data.scale = scale;
                }
                button.data.color.w = alpha;
                button.update_buffer(queue);
            }
        }

        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let mut text_brush = shared.pop::<TextBrush>().unwrap();
        let ui_brush = shared.pop::<UiBrush>().unwrap();
        let font = shared.get::<FontArc>().unwrap();
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let sections = [
            self.btn_kor_text.as_ref().unwrap()
        ];
        let ui_objects: [&dyn UiObject; 1] = [
            self.btn_kor.as_ref().unwrap()
        ];

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
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Interface Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth.view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // (한국어) 유저 인터페이스 오브젝트 그리기.
            // (English Translation) Drawing user interface objects.
            ui_brush.draw(&ui_objects, &mut rpass);

            // (한국어) 텍스트 그리기.
            // (English Translation) Drawing texts.
            text_brush.update_texture(font, device, queue, &sections);
            text_brush.draw(&sections, &mut rpass);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();


        shared.push(ui_brush);
        shared.push(text_brush);

        Ok(())
    }
}

impl Default for FirstTimeSetupScene {
    #[inline]
    fn default() -> Self {
        Self { 
            btn_kor: None,
            btn_kor_text: None,
            select_language: Language::Unknown,
            run_animation: false,
            animation_time: 0.0,
        }
    }
}
