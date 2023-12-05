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

use glam::{Vec3, Vec4Swizzles};
use rodio::OutputStreamHandle;
use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent, MouseButton},
};

use crate::{
    game_err,
    assets::bundle::AssetBundle, 
    components::{
        collider2d::Collider2d,
        text::{Section, section::d2::Section2d, brush::TextBrush}, 
        ui::{UserInterface, objects::UiObject, brush::UiBrush},
        sound::{self, SoundDecoder}, 
        script::ScriptDecoder,
        camera::GameCamera,
        user::{Language, Settings}, 
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
    static FOCUSED: Mutex<Option<(Language, Vec3, Vec3)>> = Mutex::new(None);
    
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<GameCamera>().unwrap();

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
                    let ui_color = ui.data.color.xyz();
                    let text_color = text.data.color.xyz();
                    let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                    *guard = Some((*language, ui_color, text_color));

                    // <2>
                    update_ui_color(ui, queue, ui_color * 0.5);
                    update_text_color(text, queue, text_color * 0.5);

                    // <3>
                    play_click_sound(this, shared)?;
                }
            } else if MouseButton::Left == button && !state.is_pressed() {
                let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                if let Some((language, ui_color, text_color)) = guard.take() {
                    // (한국어) 버튼을 원래 색상으로 되돌립니다.
                    // (English Translation) Returns the button to its origin color.
                    if let Some((ui, text)) = this.buttons.get_mut(&language) {
                        update_ui_color(ui, queue, ui_color);
                        update_text_color(text, queue, text_color);
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
                                Language::Korean => Ok(path::sys::SCRIPT_KOR_PATH),
                                Language::Unknown => Err(game_err!("Game Logic Error", "Unknown locale!"))
                            }?;

                            asset_bundle_cloned.get(rel_path)?.read(&ScriptDecoder)
                        }));
                        this.state = FirstTimeSetupSceneState::Exit;
                        this.language = language;
                        this.elapsed_time = 0.0;
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
    let text_brush = shared.get::<TextBrush>().unwrap();
    let ui_brush = shared.get::<UiBrush>().unwrap();
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<GameCamera>().unwrap();


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
        ui_brush.draw(&mut rpass, this.buttons.values().map(|(ui, _)| ui as &dyn UserInterface));

        // // (한국어) 텍스트 그리기.
        // // (English Translation) Drawing texts.
        text_brush.draw_2d(&mut rpass, this.buttons.values().map(|(_, text)| text as &dyn Section));
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


/// #### 한국어 </br>
/// 클릭음을 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Play a click sound. </br>
/// 
fn play_click_sound(_this: &mut FirstTimeSetupScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream_handle = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) 클릭 소리를 재생합니다.
    // (English Translation) Play a click sound.
    let source = asset_bundle.get(path::sys::CLICK_SOUND_PATH)?
        .read(&SoundDecoder)?;
    let sink = sound::create_sink(stream_handle)?;
    sink.set_volume(settings.effect_volume.get_norm());
    thread::spawn(move || {
        sink.append(source);
        sink.sleep_until_end();
    });

    Ok(())
}


/// #### 한국어 </br>
/// 사용자 인터페이스 색상을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the color of the user interface. </br>
/// 
#[inline]
fn update_ui_color(ui: &mut UiObject, queue: &wgpu::Queue, color: Vec3) {
    ui.data.color.x = color.x;
    ui.data.color.y = color.y;
    ui.data.color.z = color.z;
    ui.update_buffer(queue);
}


/// #### 한국어 </br>
/// 텍스트의 색상을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the color of the text. </br>
/// 
#[inline]
fn update_text_color(text: &mut Section2d, queue: &wgpu::Queue, color: Vec3) {
    text.data.color.x = color.x;
    text.data.color.y = color.y;
    text.data.color.z = color.z;
    text.update_buffer(queue);
}
 