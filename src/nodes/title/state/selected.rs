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
        collider2d::Collider2d, 
        text2d::brush::Text2dBrush, 
        ui::UiBrush, 
        lights::PointLights,
        camera::GameCamera, 
        sprite::SpriteBrush, 
    },
    nodes::title::{
        utils,
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
/// 현재 눌려져있는 스테이지 윈도우 ui의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed stage window ui. </br>
/// 
static FOCUSED_STAGE_WND: Mutex<Option<(usize, Vec3, Vec<Vec3>)>> = Mutex::new(None);

/// #### 한국어 </br>
/// 현재 눌려져있는 시스템 버튼의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed system button. </br>
/// 
static FOCUSED_SYS_BTN: Mutex<Option<(usize, Vec3, Vec<Vec3>)>> = Mutex::new(None);



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
    let point_lights = shared.get::<Arc<PointLights>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap();
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
        sprite_brush.draw(point_lights, &mut rpass, [this.background.as_ref()].into_iter());
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
        sprite_brush.draw(point_lights, &mut rpass, this.sprites.iter().map(|(it, _)| it.as_ref()));
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
        ui_brush.draw(
            &mut rpass, 
            this.system_buttons.iter()
            .map(|(it, _)| it.as_ref())
        );
        text_brush.draw(
            &mut rpass, 
            this.system_buttons.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it.as_ref())
        );
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
        ui_brush.draw(
            &mut rpass, 
            this.stage_window.iter()
            .map(|(it, _)| it.as_ref())
        );
        text_brush.draw(
            &mut rpass, 
            this.stage_window.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it.as_ref())
        );
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


fn handle_keyboard_input(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    use crate::components::sound;
    
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite = shared.get::<utils::Sprites>().unwrap();
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
                        if let Some((index, ui_color, section_colors)) = guard.take() {
                            if let Some((ui, sections)) = this.stage_window.get(index) {
                                ui.update(queue, |data| {
                                    data.color = (ui_color, data.color.w).into();
                                });
                                for (section_color, section) in section_colors.into_iter().zip(sections.iter()) {
                                    section.update(queue, |data| {
                                        data.color = (section_color, data.color.w).into();
                                    });
                                }
                            }
                        }
                    }

                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    {
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        if let Some((index, ui_color, section_colors)) = guard.take() {
                            if let Some((ui, sections)) = this.system_buttons.get(index) {
                                ui.update(queue, |data| {
                                    data.color = (ui_color, data.color.w).into();
                                });
                                for (section_color, section) in section_colors.into_iter().zip(sections.iter()) {
                                    section.update(queue, |data| {
                                        data.color = (section_color, data.color.w).into();
                                    });
                                }
                            }
                        }
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state.
                    this.state = TitleState::ExitSelected;
                    this.elapsed_time = 0.0;
                };
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
                    let select = this.stage_window.iter()
                        .enumerate()
                        .filter(|(idx, _)| {
                            utils::StageWindow::Background as usize != *idx
                        })
                        .find(|(_, (ui, _))| {
                            ui.test(&(cursor_pos, camera))
                        });

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
                    if let Some((index, (ui, sections))) = select {
                        // <1>
                        let ui_color = ui.data.lock().expect("Failed to access variable.").color.xyz();
                        let section_colors = sections.iter().map(|it| {
                            it.data.lock().expect("Failed to access variable.").color.xyz()
                        }).collect();
                        let mut guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                        *guard = Some((index, ui_color, section_colors));

                        // <2>
                        ui.update(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });
                        for section in sections.iter() {
                            section.update(queue, |data| {
                                data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                            });
                        }

                        // <3>
                        ui_pressed(utils::StageWindow::from(index), this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                    if let Some((index, ui_color, section_colors)) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        if let Some((ui, sections)) = this.stage_window.get_mut(index) {
                            ui.update(queue, |data| {
                                data.color = (ui_color, data.color.w).into();
                            });
                            for (section_color, section) in section_colors.into_iter().zip(sections.iter()) {
                                section.update(queue, |data| {
                                    data.color = (section_color, data.color.w).into();
                                });
                            }
                        };
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let select = this.stage_window.iter()
                            .enumerate()
                            .filter(|(idx, _)| {
                                utils::StageWindow::Background as usize != *idx
                            })
                            .find_map(|(idx, (ui, _))| {
                                if ui.test(&(cursor_pos, camera)) { Some(idx) } else { None }
                            });

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if select.is_some_and(|select| index == select) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            ui_released(utils::StageWindow::from(index), this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED_STAGE_WND.lock().expect("Failed to access variable.");
                if let Some((index, _, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    ui_dragged(utils::StageWindow::from(*index), this, shared)?;
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
                    let select = this.system_buttons.iter()
                        .enumerate()
                        .find(|(_, (ui, _))| {
                            ui.test(&(cursor_pos, camera))
                        });

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
                    if let Some((index, (ui, sections))) = select {
                        // <1>
                        let ui_color = ui.data.lock().expect("Failed to access variable.").color.xyz();
                        let section_colors = sections.iter().map(|it| {
                            it.data.lock().expect("Failed to access variable.").color.xyz()
                        }).collect();
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        *guard = Some((index, ui_color, section_colors));

                        // <2>
                        ui.update(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });
                        for section in sections.iter() {
                            section.update(queue, |data| {
                                data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                            });
                        }

                        // <3>
                        sys_ui_pressed(utils::SystemButtons::from(index), this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                    if let Some((index, ui_color, section_colors)) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        if let Some((ui, sections)) = this.system_buttons.get_mut(index) {
                            ui.update(queue, |data| {
                                data.color = (ui_color, data.color.w).into();
                            });
                            for (section_color, section) in section_colors.into_iter().zip(sections.iter()) {
                                section.update(queue, |data| {
                                    data.color = (section_color, data.color.w).into();
                                });
                            }
                        };
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let select = this.system_buttons.iter()
                            .enumerate()
                            .find_map(|(idx, (ui, _))| {
                                if ui.test(&(cursor_pos, camera)) { Some(idx) } else { None }
                            });

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if select.is_some_and(|select| index == select) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            sys_ui_released(utils::SystemButtons::from(index), this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                if let Some((index, _, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    sys_ui_dragged(utils::SystemButtons::from(*index), this, shared)?;
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
fn ui_pressed(btn: utils::StageWindow, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    use crate::components::sound;

    match btn {
        utils::StageWindow::Enter => {
            sound::play_click_sound(shared)?;
            // TODO
            Ok(())
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_pressed(btn: utils::SystemButtons, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    use crate::components::sound;

    match btn {
        utils::SystemButtons::Return => {
            sound::play_cancel_sound(shared)
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_released(btn: utils::StageWindow, this: &mut TitleScene, _shared: &mut Shared) -> AppResult<()> {
    match btn {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_released(btn: utils::SystemButtons, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite = shared.get::<utils::Sprites>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match btn {
        utils::SystemButtons::Return => {
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
            this.elapsed_time = 0.0;
            Ok(())
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_dragged(btn: utils::StageWindow, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match btn {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sys_ui_dragged(btn: utils::SystemButtons, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match btn {
        _ => Ok(())
    }
}
