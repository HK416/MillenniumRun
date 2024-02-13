use std::sync::{Arc, Mutex};

use glam::{Vec3, Vec4, Vec4Swizzles};
use winit::{
    event::{Event, WindowEvent, MouseButton}, 
    keyboard::{KeyCode, PhysicalKey},
    dpi::PhysicalPosition,
};

use crate::{
    game_err, 
    components::{
        collider2d::Collider2d, 
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        camera::GameCamera, 
        sound, 
    }, 
    nodes::title::{
        TitleScene, 
        state::TitleState, 
    }, 
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent,
        shared::Shared, 
    }, 
};

/// #### 한국어 </br>
/// 현재 눌려져있는 인터페이스의 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation)
/// Contains data for the currently pressed interface. </br>
/// 
static FOCUSED_UI: Mutex<Option<(Buttons, Vec3)>> = Mutex::new(None);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Buttons {
    Previous, 
    Next,
}



pub fn handle_events(this: &mut TitleScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_keyboard_events(this, shared, &event)?;
    handle_mouse_events(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut TitleScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다. 
    // (English Translation) Get shared objects to use.
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(Tutorial2(Background)))"),
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
        let iter = [&this.background].into_iter().chain(this.sprites.iter().map(|(it, _)| it));
        sprite_brush.draw(&mut rpass, iter);
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(Tutorial2(Ui)))"),
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
        ui_brush.draw(&mut rpass, [&this.tutorials[2].0, &this.tutorial_prev_btn, &this.tutorial_next_btn].into_iter());
        text_brush.draw(&mut rpass, [&this.tutorials[2].1].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}

fn handle_keyboard_events(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    // (한국어) 눌려져있는 인터페이스를 원래대로 되돌립니다. 
                    // (English Translation) Returns the pressed interface to its orignal state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((btn, ui_color)) = guard.take() {
                        match btn {
                            Buttons::Previous => this.tutorial_prev_btn.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::Next => this.tutorial_next_btn.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };
                    }

                    // (한국어) `Stage` 상태로 상태를 변경합니다.
                    // (English Translation) Changes the state to `Stage` state. 
                    this.timer = 0.0;
                    this.state = TitleState::Stage;
                }
            },
            _ => { /* empty */ },
        },
        _ => { /* empty */ },
    };

    Ok(())
}

fn handle_mouse_events(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the interface area. 
                    let selected = [(Buttons::Previous, &this.tutorial_prev_btn), (Buttons::Next, &this.tutorial_next_btn)]
                        .into_iter()
                        .find_map(|(btn, ui)| ui.test(&(cursor_pos, camera)).then_some(btn));

                    if let Some(btn) = selected {
                        // (한국어) 선택된 인터페이스의 데이터를 저장합니다.
                        // (English Translation) Stores the data of the selected interface. 
                        let ui_color = match btn {
                            Buttons::Previous => this.tutorial_prev_btn.data.lock().expect("Failed to access variable.").color.xyz(),
                            Buttons::Next => this.tutorial_next_btn.data.lock().expect("Failed to access variable.").color.xyz(),
                        };
                        let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                        *guard = Some((btn, ui_color));

                        // (한국어) 선택된 인터페이스를 갱신합니다.
                        // (English Translation) Updates the selected interface. 
                        match btn {
                            Buttons::Previous =>  this.tutorial_prev_btn.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0)),
                            Buttons::Next => this.tutorial_next_btn.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0)),
                        };

                        // (한국어) 소리를 재생합니다.
                        // (English Translation) Play the sounds.
                        sound::play_click_sound(shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    // (한국어) 눌려져있는 인터페이스를 원래대로 되돌립니다. 
                    // (English Translation) Returns the pressed interface to its orignal state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((btn, ui_color)) = guard.take() {
                        match btn {
                            Buttons::Previous => this.tutorial_prev_btn.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::Next => this.tutorial_next_btn.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };

                        // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the interface area. 
                        let selected = [(Buttons::Previous, &this.tutorial_prev_btn), (Buttons::Next, &this.tutorial_next_btn)]
                            .into_iter()
                            .find_map(|(btn, ui)| ui.test(&(cursor_pos, camera)).then_some(btn));
                        if selected.is_some_and(|selected| selected == btn) {
                            // (한국어) 다음 상태로 상태를 변경합니다.
                            // (English Translation) Changes the state to next state. 
                            match btn {
                                Buttons::Previous => {
                                    this.timer = 0.0;
                                    this.state = TitleState::Tutorial1;
                                },
                                Buttons::Next => {
                                    this.timer = 0.0;
                                    this.state = TitleState::Tutorial3;
                                }
                            };
                        }
                    }
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ },
    }

    Ok(())
}
