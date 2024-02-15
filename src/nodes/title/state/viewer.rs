use std::thread;
use std::sync::{Arc, Mutex};

use glam::{Vec4, Vec3, Vec4Swizzles};
use rodio::{OutputStream, OutputStreamHandle};
use winit::{
    event::{Event, WindowEvent, MouseButton}, 
    keyboard::{KeyCode, PhysicalKey},
    dpi::PhysicalPosition, 
};

use crate::{
    game_err, 
    assets::bundle::AssetBundle, 
    components::{
        ui::UiBrush,
        sprite::SpriteBrush, 
        collider2d::Collider2d, 
        camera::GameCamera, 
        player::Actor, 
        user::Settings, 
        sound, 
    }, 
    nodes::{
        path,
        title::{
            TitleScene, 
            state::TitleState, 
        },
    }, 
    render::depth::DepthBuffer, 
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared, 
    }
};

/// #### 한국어 </br>
/// 현재 눌려져있는 인터페이스의 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data for the currently pressed interface. </br>
/// 
static FOCUSED_UI: Mutex<Option<Vec3>> = Mutex::new(None);



pub fn handle_events(this: &mut TitleScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_keyboard_events(this, shared, &event)?;
    handle_mouse_events(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut TitleScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let actor = shared.get::<Actor>().unwrap();

    
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
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(Viewer(Background)))"),
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

        camera.bind(&mut rpass);

        // (한국어) 배경 오브젝트들 그리기.
        // (English Translation) Drawing background objects.
        sprite_brush.draw(&mut rpass, [&this.background].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(Viewer(Ui)))"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
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

        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.stage_viewer_images[actor], 
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}

fn handle_keyboard_events(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                            let settings = shared.get::<Settings>().unwrap();
                            let asset_bundle = shared.get::<AssetBundle>().unwrap();
                            let source = asset_bundle.get(path::CANCEL_SOUND_PATH)?.read(&sound::SoundDecoder)?;
                            sink.set_volume(settings.effect_volume.norm());
                            sink.append(source);
                            thread::spawn(move || {
                                sink.sleep_until_end();
                            });
                            shared.push((stream, stream_handle));
                        }
                    }

                    // (한국어) 선택된 인터페이스를 원래 상태로 되돌립니다.
                    // (English Translation) Return the selected interface to its original state. 
                    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some(ui_color) = guard.take() {
                        this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into());
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state. 
                    this.timer = 0.0;
                    this.state = TitleState::ExitViewer;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }

    Ok(())
}

fn handle_mouse_events(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area.
                    let selected = this.return_button.test(&(cursor_pos, camera));

                    // (한국어) 마우스 커서가 ui 영역 안에 있는 경우:
                    // (English Translation) If the mouse cursor is inside the ui area:
                    if selected {
                        // (한국어) `FOCUSED`에 해당 ui의 색상을 저장합니다.
                        // (English Translation) Store the ui color in `FOCUSED`.
                        let ui_color = this.return_button.data.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                        *guard = Some(ui_color);

                        // (한국어) 해당 ui의 색상을 변경합니다.
                        // (English Translation) Change the color of the ui.
                        this.return_button.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));

                        // (한국어) 소리를 재생합니다.
                        // (English Translation) Play the sounds.
                        if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                            if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                                let settings = shared.get::<Settings>().unwrap();
                                let asset_bundle = shared.get::<AssetBundle>().unwrap();
                                let source = asset_bundle.get(path::CANCEL_SOUND_PATH)?.read(&sound::SoundDecoder)?;
                                sink.set_volume(settings.effect_volume.norm());
                                sink.append(source);
                                thread::spawn(move || {
                                    sink.sleep_until_end();
                                });
                                shared.push((stream, stream_handle));
                            }
                        }
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    // (한국어) 선택된 인터페이스를 원래 상태로 되돌립니다.
                    // (English Translation) Return the selected interface to its original state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some(ui_color) = guard.take() {
                        this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into());
                    }

                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area.
                    let selected = this.return_button.test(&(cursor_pos, camera));

                    // (한국어) 마우스 커서가 ui 영역 안에 있는 경우:
                    // (English Translation) If the mouse cursor is inside the ui area:
                    if selected {
                        // (한국어) 다음 게임 장면 상태로 변경합니다.
                        // (English Translation) Change to the next game scene state. 
                        this.timer = 0.0;
                        this.state = TitleState::ExitViewer;
                    }
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }

    Ok(())
}
