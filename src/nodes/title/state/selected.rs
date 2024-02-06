use std::sync::{Arc, Mutex};

use glam::{Vec4, Vec3, Vec4Swizzles};
use winit::{
    event::{Event, WindowEvent, MouseButton}, 
    keyboard::{PhysicalKey, KeyCode},
    dpi::PhysicalPosition, 
};

use crate::{
    game_err,
    components::{
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        collider2d::Collider2d, 
        camera::GameCamera, 
        player::Actor, 
        sound, 
    },
    nodes::{
        title::{
            TitleScene, 
            state::TitleState,
        },
        in_game::InGameLoading,
    }, 
    render::depth::DepthBuffer, 
    scene::state::SceneState, 
    system::{
        error::{AppResult, GameError},
        event::AppEvent, 
        shared::Shared, 
    }, 
};

/// #### 한국어 </br>
/// 현재 눌려져있는 스테이지 윈도우 ui의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed stage window ui. </br>
/// 
static FOCUSED_STAGE_WND: Mutex<Option<(Vec3, Vec3)>> = Mutex::new(None);

/// #### 한국어 </br>
/// 현재 눌려져있는 시스템 버튼의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed system button. </br>
/// 
static FOCUSED_SYS_BTN: Mutex<Option<Vec3>> = Mutex::new(None);



pub fn handle_events(this: &mut TitleScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_keyboard_input(this, shared, &event)?;
    handle_mouse_input(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut TitleScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
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
            label: Some("RenderPass(TitleScene(EnterSelected(Background)))"),
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
            label: Some("RenderPass(TitleScene(EnterSelected(Sprites)))"),
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

        // (한국어) 배경 오브젝트들 그리기.
        // (English Translation) Drawing background objects.
        sprite_brush.draw(&mut rpass, this.sprites.iter().map(|(it, _)| it));
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(EnterSelected(Ui)))"),
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

        // (한국어) 버튼 그리기.
        // (English Translation) Drawing the buttons.
        ui_brush.draw(&mut rpass, [&this.return_button].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(EnterSelected(Ui)))"),
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

        // (한국어) 스테이지 윈도우 창을 그립니다. 
        // (English Translation) Drawing the stage window.
        ui_brush.draw(&mut rpass, [
            &this.stage_window, 
            &this.stage_enter_button.0, 
            &this.stage_images[&actor].0, 
            &this.stage_images[&actor].1, 
        ].into_iter());
        text_brush.draw(&mut rpass, [
            &this.stage_enter_button.1, 
            &this.stage_images[&actor].2,
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


fn handle_keyboard_input(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite = shared.get::<Actor>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. }
            => if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    sound::play_cancel_sound(shared)?;
                    
                    // (한국어) 선택된 스프라이트의 이미지를 변경합니다. 
                    // (English Translation) Changes the image of the selected sprite. 
                    if let Some((sprite, _)) = this.sprites.get(*sprite as usize) {
                        sprite.update(queue, |instances| {
                            for instance in instances.iter_mut() {
                                instance.texture_index = 0;
                            }
                        });
                    }

                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    {
                        let mut guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                        if let Some((ui_color, text_color)) = guard.take() {
                            this.stage_enter_button.0.update(queue, |data| {
                                data.color = (ui_color, data.color.w).into();
                            });
                            this.stage_enter_button.1.update(queue, |data| {
                                data.color = (text_color, data.color.w).into();
                            });
                        }
                    }

                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    {
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        if let Some(ui_color) = guard.take() {
                            this.return_button.update(queue, |data| {
                                data.color = (ui_color, data.color.w).into();
                            });
                        }
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state.
                    this.state = TitleState::ExitSelected;
                    this.timer = 0.0;
                } else if KeyCode::Enter == code && !event.repeat && event.state.is_pressed() {
                    sound::play_click_sound(shared)?;
                    let state = shared.get_mut::<SceneState>().unwrap();
                    *state = SceneState::Change(Box::new(InGameLoading::default()));
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    };

    Ok(())
}


fn handle_mouse_input(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    handle_mouse_input_for_ui(this, shared, event)?;
    handle_mouse_input_for_sys(this, shared, event)?;
    Ok(())
}


fn handle_mouse_input_for_ui(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area.
                    let selected = this.stage_enter_button.0.test(&(cursor_pos, camera));

                    // (한국어)
                    // 마우스 커서가 ui 영역 안에 있는 경우:
                    // 1. `FOCUSED`에 해당 ui의 태그, 색상, 텍스트 색상을 저장합니다.
                    // 2. 해당 ui의 색상과 텍스트 색상을 변경합니다.
                    // 3. ui 눌림 함수를 호출합니다.
                    //
                    // (English Translation)
                    // If the mouse cursor is inside the ui area:
                    // 1. Store the tag of the ui, ui color, and text color in `FOCUSED`.
                    // 2. Change the color of the ui and the color of the text.
                    // 3. Call the ui pressed function.
                    //
                    if selected {
                        // <1>
                        let ui_color = this.stage_enter_button.0.data.lock().expect("Failed to access variable.").color.xyz();
                        let text_color = this.stage_enter_button.1.data.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                        *guard = Some((ui_color, text_color));

                        // <2>
                        this.stage_enter_button.0.update(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });
                        this.stage_enter_button.1.update(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });

                        // <3>
                        ui_pressed(this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                    if let Some((ui_color, text_color)) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        this.stage_enter_button.0.update(queue, |data| {
                            data.color = (ui_color, data.color.w).into();
                        });
                        this.stage_enter_button.1.update(queue, |data| {
                            data.color = (text_color, data.color.w).into();
                        });
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let selected = this.stage_enter_button.0.test(&(cursor_pos, camera));

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if selected {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            ui_released(this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                if let Some((_, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    ui_dragged(this, shared)?;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    };

    Ok(())
}

fn handle_mouse_input_for_sys(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
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

                    // (한국어)
                    // 마우스 커서가 ui 영역 안에 있는 경우:
                    // 1. `FOCUSED`에 해당 ui의 태그, 색상, 텍스트 색상을 저장합니다.
                    // 2. 해당 ui의 색상과 텍스트 색상을 변경합니다.
                    // 3. ui 눌림 함수를 호출합니다.
                    //
                    // (English Translation)
                    // If the mouse cursor is inside the ui area:
                    // 1. Store the tag of the ui, ui color, and text color in `FOCUSED`.
                    // 2. Change the color of the ui and the color of the text.
                    // 3. Call the ui pressed function.
                    //
                    if selected {
                        // <1>
                        let ui_color = this.return_button.data.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        *guard = Some(ui_color);

                        // <2>
                        this.return_button.update(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });

                        // <3>
                        sys_ui_pressed(this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                    if let Some(ui_color) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        this.return_button.update(queue, |data| {
                            data.color = (ui_color, data.color.w).into();
                        });
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let selected = this.return_button.test(&(cursor_pos, camera));

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if selected {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            sys_ui_released(this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                if let Some(_) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    sys_ui_dragged(this, shared)?;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    };

    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_pressed(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    sound::play_click_sound(shared)
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_pressed(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    sound::play_cancel_sound(shared)
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_released(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    let state = shared.get_mut::<SceneState>().unwrap();
    *state = SceneState::Change(Box::new(InGameLoading::default()));
    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_released(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite = shared.get::<Actor>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 선택된 스프라이트의 이미지를 변경합니다. 
    // (English Translation) Changes the image of the selected sprite. 
    if let Some((sprite, _)) = this.sprites.get(*sprite as usize) {
        sprite.update(queue, |instances| {
            for instance in instances.iter_mut() {
                instance.texture_index = 0;
            }
        });
    }

    this.state = TitleState::ExitSelected;
    this.timer = 0.0;
    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_dragged(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_dragged(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    Ok(())
}
