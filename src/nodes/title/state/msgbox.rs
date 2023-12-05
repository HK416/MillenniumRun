use std::sync::Arc;

use glam::{Vec3, Vec4Swizzles};
use winit::{
    event::{Event, WindowEvent, MouseButton},
    keyboard::{PhysicalKey, KeyCode},
    dpi::PhysicalPosition,
};

use crate::{
    game_err,
    components::{
        collider2d::Collider2d,
        sprite::brush::SpriteBrush,
        text::{brush::TextBrush, section::d2::Section2d},
        ui::{brush::UiBrush, objects::UiObject},
        camera::GameCamera,
    },
    nodes::title::{
        TitleScene,
        ty, state::TitleState,
    }, 
    render::depth::DepthBuffer,
    scene::state::SceneState,
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent,
        shared::Shared, 
    }
};



pub fn handle_events(this: &mut TitleScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    check_button_pressed(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut TitleScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite_brush = shared.get::<SpriteBrush>().unwrap();
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
            label: Some("RenderPass(TitleScene(MsgBoxState(Background)))"),
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
        
        // (한국어) 배경 오브젝트 그리기.
        // (English Translation) Drawing background objects.
        this.background.draw(sprite_brush, &mut rpass);
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(MsgBoxState(Ui)))"),
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

        // (한국어) 메시지 박스 그리기.
        // (English Translation) Drawing the message box.
        this.exit_window.draw(ui_brush, text_brush, &mut rpass);
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


fn check_button_pressed(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    use std::sync::Mutex;
    static FOCUSED: Mutex<Option<(usize, Vec3, Vec<Vec3>)>> = Mutex::new(None);

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<GameCamera>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(code) = event.physical_key {
                if !event.repeat && event.state.is_pressed() && code == KeyCode::Enter {
                    *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
                } else if !event.repeat && event.state.is_pressed() && code == KeyCode::Escape {
                    super::play_cancel_sound(this, shared)?;

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state.
                    this.elapsed_time = 0.0;
                    this.state = TitleState::ExitMsgBox;
                };
            },
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 버튼 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the button area.
                    let select = this.exit_window
                        .iter_mut()
                        .enumerate()
                        .skip(1)
                        .find(|(_, it)| it.inner.test(&(cursor_pos, camera)));

                    // (한국어) 
                    // 마우스 커서가 버튼 영역 안에 있는 경우:
                    // 1. `FOCUSED`에 해당 버튼의 인덱스와 버튼의 색상, 텍스트의 색상을 저장합니다.
                    // 2. 해당 버튼의 색상과 텍스트의 색상을 변경합니다.
                    // 3. `click` 소리를 재생 합니다.
                    //
                    // (English Translation)
                    // If the mouse cursor is inside the button area:
                    // 1. Store the index of the button, button color, and text color in `FOCUSED`.
                    // 2. Change the color of the button and the color of the text.
                    // 3. Play the `click` sound.
                    //
                    if let Some((idx, ui)) = select {
                        // <1>
                        let ui_color = ui.inner.data.color.xyz();
                        let text_color = ui.texts.iter().map(|it| it.data.color.xyz()).collect();
                        let mut gaurd = FOCUSED.lock().expect("Failed to access variable.");
                        *gaurd = Some((idx, ui_color, text_color));

                        // <2>
                        update_ui_color(&mut ui.inner, queue, ui_color * 0.5);
                        for text in ui.texts.iter_mut() {
                            update_text_color(text, queue, text.data.color.xyz() * 0.5);
                        }

                        // <3>
                        play_button_sound(idx, this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                    if let Some((focused_idx, ui_color, text_color)) = guard.take() {
                        // (한국어) 버튼을 원래 색상으로 되돌립니다.
                        // (English Translation) Returns the button to its origin color.
                        if let Some(ui) = this.exit_window.get_mut(focused_idx) {
                            update_ui_color(&mut ui.inner, queue, ui_color);
                            for (i, text) in ui.texts.iter_mut().enumerate() {
                                update_text_color(text, queue, text_color[i]);
                            }
                        };
                        
                        // (한국어) 마우스 커서가 버튼 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the button area.
                        let select = this.exit_window
                            .iter_mut()
                            .enumerate()
                            .skip(1)
                            .find_map(|(idx, it)| {
                                it.inner.test(&(cursor_pos, camera)).then_some(idx)
                            });

                        // (한국어) 선택된 마우스 버튼이 이전에 선택된 버튼과 일치할 경우:
                        // (English Translation) If the selected mouse button matches a previously selected button:
                        if select.is_some_and(|idx| focused_idx == idx) {
                            handles_button(focused_idx, this, shared);
                        } 
                    }
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */}
    };

    Ok(())
}

/// #### 한국어 </br>
/// 버튼이 눌렸을 때 소리를 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Plays a sound when the button is pressed. </br>
/// 
#[inline]
fn play_button_sound(button_idx: usize, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    if button_idx == ty::ExitWindowTags::Okay as usize {
        super::play_click_sound(this, shared)
    } else if button_idx == ty::ExitWindowTags::Cancel as usize {
        super::play_cancel_sound(this, shared)
    } else {
        Ok(())
    }
}

/// #### 한국어 </br>
/// 주어진 버튼 인덱스에 따라 다음 상태로 변경합니다. </br>
/// 
/// #### English (Translation) </br>
/// Changes to the next state based on the given button index. </br>
/// 
#[inline]
fn handles_button(button_idx: usize, this: &mut TitleScene, shared: &mut Shared) {
    if button_idx == ty::ExitWindowTags::Okay as usize {
        *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
    } else if button_idx == ty::ExitWindowTags::Cancel as usize {
        this.elapsed_time = 0.0;
        this.state = TitleState::ExitMsgBox;
    }
}


/// #### 한국어 </br>
/// 사용자 인터페이스의 색상을 갱신합니다. </br>
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
