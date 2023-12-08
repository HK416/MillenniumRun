use std::sync::Arc;

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
        sprite::brush::SpriteBrush,
        text::brush::TextBrush,
        ui::brush::UiBrush, 
        camera::GameCamera,
    },
    nodes::title::{
        TitleScene,
        ty, state::TitleState, 
    },
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }
};



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
    let sprite_brush = shared.get::<SpriteBrush>().unwrap();
    let ui_brush = shared.get::<UiBrush>().unwrap();
    let text_brush = shared.get::<TextBrush>().unwrap();
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
            label: Some("RenderPass(TitleScene(StageState(Sprite)))"),
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
        this.background.draw(sprite_brush, &mut rpass);

        // (한국어) 스프라이트 오브젝트들 그리기.
        // (English Translation) Drawing sprite objects.
        this.sprite.draw(sprite_brush, &mut rpass);
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(StageState(Ui)))"),
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
        this.system.draw(ui_brush, text_brush, &mut rpass);
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


fn handle_keyboard_input(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. }
            => if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    super::play_cancel_sound(this, shared)?;

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state.
                    this.state = TitleState::ExitStage;
                    this.elapsed_time = 0.0;
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
    handle_mouse_input_for_sprites(this, shared, event)?;
    Ok(())
}


fn handle_mouse_input_for_sprites(this: &mut TitleScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    use std::sync::Mutex;
    static FOCUSED: Mutex<Option<(ty::SpriteButtonTags, Vec3)>> = Mutex::new(None);

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<GameCamera>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 
                    // 윈도우 좌표계상 마우스 위치를 월드 좌표계상 마우스 위치로 변환합니다.
                    // 
                    // (English Translation) 
                    // Converts the mouse position in the Window coordinate system 
                    // to the mouse position in the world coordinate system.
                    // 
                    let (x, y) = camera.to_world_coordinates(cursor_pos);
                    
                    // (한국어) 마우스 커서가 스프라이트 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the sprite area.
                    let pickable = this.sprite.pickable();
                    let select = pickable.into_iter()
                        .find(|(_, sprite)| {
                            sprite.1.test(&(x, y))
                        });

                    // (한국어)
                    // 마우스 커서가 스프라이트 영역 안에 있는 경우:
                    // 1. `FOCUSED`에 해당 스프라이트의 태그와 스프라이트의 색상을 저장합니다.
                    // 2. 해당 스프라이트의 색상을 변경합니다.
                    // 3. 스프라이트 눌림 함수를 호출합니다.
                    //
                    // (English Translation)
                    // If the mouse cursor is inside the sprite area:
                    // 1. Store the tag of the sprite, and sprite color in `FOCUSED`.
                    // 2. Change the color of the sprite.
                    // 3. Calls the sprite pressed function.
                    //
                    if let Some((tag, sprite)) = select {
                        // <1>
                        let sprite_color = sprite.0.data.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                        *guard = Some((tag, sprite_color));

                        // <2>
                        sprite.0.update_sprite(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });

                        // <3>
                        sprite_pressed(tag, this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                    if let Some((tag, sprite_color)) = guard.take() {
                        // (한국어) 
                        // 윈도우 좌표계상 마우스 위치를 월드 좌표계상 마우스 위치로 변환합니다.
                        // 
                        // (English Translation) 
                        // Converts the mouse position in the Window coordinate system 
                        // to the mouse position in the world coordinate system.
                        // 
                        let (x, y) = camera.to_world_coordinates(cursor_pos);

                        // (한국어) 스프라이트를 원래 색상으로 되돌립니다.
                        // (English Translation) Returns the sprite to its origin color.
                        if let Some(sprite) = this.sprite.get_mut(tag as usize) {
                            sprite.0.update_sprite(queue, |data| {
                                data.color = (sprite_color, data.color.w).into();
                            });
                        };

                        // (한국어) 마우스 커서가 스프라이트 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the sprite area.
                        let pickable = this.sprite.pickable();
                        let select = pickable.into_iter()
                            .find_map(|(tag, sprite)| {
                                sprite.1.test(&(x, y)).then(|| tag)
                            });

                        // (한국어) 선택된 스프라이트가 이전에 선택된 스프라이트와 일치할 경우:
                        // (English Translation) If the selected sprite matches a previously selected sprite:
                        if select.is_some_and(|select| tag == select) {
                            // (한국어) 스프라이트 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the sprite released function.
                            sprite_released(tag, this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED.lock().expect("Failed to access variable.");
                if let Some((tag, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    sprite_dragged(*tag, this, shared)?;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    };

    Ok(())
}


fn handle_mouse_input_for_ui(
    this: &mut TitleScene, 
    shared: &mut Shared, 
    event: &Event<AppEvent>
) -> AppResult<()> {
    use std::sync::Mutex;
    static FOCUSED: Mutex<Option<(ty::SystemButtonTags, Vec3, Vec<Vec3>)>> = Mutex::new(None);

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<GameCamera>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area.
                    let pickable = this.system.pickable();
                    let select = pickable.into_iter()
                        .find(|(_, (ui, _))| ui.test(&(cursor_pos, camera)));

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
                    // 3. Calls the ui pressed function.
                    //
                    if let Some((tag, (ui, texts))) = select {
                        // <1>
                        let ui_color = ui.data.lock().expect("Failed to access variable.").color.xyz();
                        let text_colors = texts.iter().map(|it| {
                            it.data.lock().expect("Failed to get variable").color.xyz()
                        }).collect();
                        let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                        *guard = Some((tag, ui_color, text_colors));

                        // <2>
                        ui.update_buffer(queue, |data| {
                            data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                        });
                        for text in texts.iter() {
                            text.update_section(queue, |data| {
                                data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                            });
                        }

                        // <3>
                        ui_pressed(tag, this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED.lock().expect("Failed to access variable.");
                    if let Some((tag, ui_color, text_colors)) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        if let Some((ui, texts)) = this.system.get_mut(tag as usize) {
                            ui.update_buffer(queue, |data| {
                                data.color = (ui_color, data.color.w).into();
                            });
                            for (text_color, text) in text_colors.into_iter().zip(texts.iter()) {
                                text.update_section(queue, |data| {
                                    data.color = (text_color, data.color.w).into();
                                });
                            }
                        };
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let pickable = this.system.pickable();
                        let select = pickable.into_iter()
                            .find_map(|(tag, (ui, _))| {
                                ui.test(&(cursor_pos, camera)).then_some(tag)
                            });

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if select.is_some_and(|select| tag == select) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            ui_released(tag, this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED.lock().expect("Failed to access variable.");
                if let Some((tag, _, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    ui_dragged(*tag, this, shared)?;
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
fn sprite_pressed(tag: ty::SpriteButtonTags, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_pressed(tag: ty::SystemButtonTags, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        ty::SystemButtonTags::ReturnButton => {
            super::play_click_sound(this, shared)
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sprite_released(tag: ty::SpriteButtonTags, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        ty::SpriteButtonTags::Yuzu => {
            shared.push(ty::SpriteButtonTags::Yuzu);
            this.state = TitleState::EnterSelected;
            this.elapsed_time = 0.0;
            Ok(())
        },
        ty::SpriteButtonTags::Aris => {
            shared.push(ty::SpriteButtonTags::Aris);
            this.state = TitleState::EnterSelected;
            this.elapsed_time = 0.0;
            Ok(())
        },
        ty::SpriteButtonTags::Momoi => {
            shared.push(ty::SpriteButtonTags::Momoi);
            this.state = TitleState::EnterSelected;
            this.elapsed_time = 0.0;
            Ok(())
        },
        ty::SpriteButtonTags::Midori => {
            shared.push(ty::SpriteButtonTags::Midori);
            this.state = TitleState::EnterSelected;
            this.elapsed_time = 0.0;
            Ok(())
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_released(tag: ty::SystemButtonTags, this: &mut TitleScene, _shared: &mut Shared) -> AppResult<()> {
    match tag {
        ty::SystemButtonTags::ReturnButton => {
            this.state = TitleState::ExitStage;
            this.elapsed_time = 0.0;
            Ok(())
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sprite_dragged(tag: ty::SpriteButtonTags, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_dragged(tag: ty::SystemButtonTags, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        _ => Ok(())
    }
}
