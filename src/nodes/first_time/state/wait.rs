//! #### 한국어 </br>
//! 사용자가 언어를 선택할 때 까지 대기 중인 상태입니다. </br>
//! 사용자의 키보드, 마우스 입력을 처리합니다. </br>
//! 
//! #### English (Translation) </br>
//! It is waiting for the user to select a language. </br>
//! Processes the user's keyboard and mouse input. </br>
//! 
use std::thread;
use std::sync::Arc;

use glam::{Vec4, Vec3, Vec4Swizzles};
use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent, MouseButton},
};

use crate::{
    game_err,
    assets::bundle::AssetBundle, 
    components::{
        collider2d::Collider2d,
        text::TextBrush,
        ui::UiBrush,
        script::ScriptDecoder,
        camera::GameCamera,
        user::Language, 
    },
    render::depth::DepthBuffer,
    nodes::{
        path, 
        first_time::{
            FirstTimeSetupScene,
            state::FirstTimeSetupSceneState,
        },
    },
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }, 
};



pub fn handle_events(this: &mut FirstTimeSetupScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    use std::sync::Mutex;
    use crate::components::sound::play_click_sound;

    // (한국어) 눌린 버튼의 색상을 저장하는 변수입니다. 
    // (English Translation) This is a variable that stores the color of the pressed button. 
    static FOCUSED: Mutex<Option<(Language, Vec3, Vec3)>> = Mutex::new(None);
    
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } 
            => if MouseButton::Left == button && state.is_pressed() {
                // (한국어) 선택된 버튼이 있는지 확인합니다.
                // (English Translation) Checks if any button is selected.
                let select = this.buttons
                    .iter_mut()
                    .find(|(_, (ui, _))| {
                        ui.test(&(cursor_pos, camera))
                    });

                // (한국어)
                // 마우스 커서가 버튼 영역 안에 있는 경우:
                // 1. `FOCUSED`에 해당 버튼의 언어와 버튼의 색상, 텍스트의 색상을 저장합니다.
                // 2. 해당 버튼의 색상과 텍스트의 색상을 변경합니다.
                // 3. `click` 소리를 재생합니다.
                //
                // (English Translation)
                // If the mouse cursor is inside the button area:
                // 1. Store the language of the button, button color, and text color in `FOCUSED`.
                // 2. Change the color of the button and the color of the text.
                // 3. Play the `click`sound.
                //
                if let Some((language, (ui, text))) = select {
                    // <1>
                    let ui_color = ui.data.lock().expect("Failed to access variable.").color.xyz();
                    let text_color = text.data.lock().expect("Failed to access variable").color.xyz();
                    let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                    *guard = Some((*language, ui_color, text_color));

                    // <2>
                    ui.update(queue, |data| {
                        data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                    });
                    text.update(queue, |data| {
                        data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                    });

                    // <3>
                    play_click_sound(shared)?;
                }
            } else if MouseButton::Left == button && !state.is_pressed() {
                let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                if let Some((language, ui_color, text_color)) = guard.take() {
                    // (한국어) 버튼을 원래 색상으로 되돌립니다.
                    // (English Translation) Returns the button to its origin color.
                    if let Some((ui, text)) = this.buttons.get_mut(&language) {
                        ui.update(queue, |data| {
                            data.color = (ui_color, 1.0).into();
                        });
                        text.update(queue, |data| {
                            data.color = (text_color, 1.0).into();
                        });
                    }

                    // (한국어) 마우스 커서가 버튼 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the button area.
                    let select = this.buttons
                        .iter()
                        .find_map(|(language, (ui, _))| {
                            ui.test(&(cursor_pos, camera)).then_some(*language)
                        });

                    // (한국어) 선택된 마우스 버튼이 이전에 선택된 버튼과 일치할 경우:
                    // (English Translation) If the selected mouse button matches a previously selected button:
                    if select.is_some_and(|select| select == language) {
                        // (한국어) 
                        // 선택된 언어의 스크립트를 로드하고, `Exit` 상태로 변경한다.
                        // 
                        // (English Translation) 
                        // Loads the script of the selected language 
                        // and change to `Exit` state.
                        //
                        let language_cloned = language.clone();
                        let asset_bundle_cloned = asset_bundle.clone();
                        this.loading = Some(thread::spawn(move || {
                            let rel_path = match language_cloned {
                                Language::Korean => Ok(path::KOR_SCRIPTS_PATH),
                                Language::Unknown => Err(game_err!("Game Logic Error", "Unknown locale!"))
                            }?;

                            let script = asset_bundle_cloned.get(rel_path)?
                                .read(&ScriptDecoder)?;

                            Ok(Arc::new(script))
                        }));
                        this.state = FirstTimeSetupSceneState::Exit;
                        this.language = language;
                        this.timer = 0.0;
                    }
                }
            }
            _ => { /* empty */ }
        },
        _ => { /* emtpy */ }
    };

    Ok(())
}

pub fn update(_this: &mut FirstTimeSetupScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &FirstTimeSetupScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
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

    // (한국어) 프레임 버퍼의 텍스쳐 뷰를 생성합니다.
    // (English Translation) Creates a texture view of the framebuffer.
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(FirstTimeSetupScene(Wait(Ui))))"),
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

        // (한국어) 카메라 바인딩.
        // (English Translation) Bind the camera.
        camera.bind(&mut rpass);

        // (한국어) 유저 인터페이스 오브젝트 그리기.
        // (English Translation) Drawing user interface objects.
        ui_brush.draw(&mut rpass, this.buttons.values().map(|(ui, _)| ui));

        // // (한국어) 텍스트 그리기.
        // // (English Translation) Drawing texts.
        text_brush.draw(&mut rpass, this.buttons.values().map(|(_, text)| text));
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
