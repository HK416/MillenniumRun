use std::thread;
use std::sync::{Arc, Mutex};

use glam::{Vec4, Vec3, Vec4Swizzles, Vec3Swizzles};
use rodio::{OutputStream, OutputStreamHandle};
use winit::{
    event::{Event, WindowEvent, MouseButton}, 
    keyboard::{PhysicalKey, KeyCode}, 
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
        }
    },
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }
};

/// #### 한국어 </br>
/// 현재 눌려져있는 스프라이트의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed sprite. </br>
/// 
static FOCUSED_SPRITE: Mutex<Option<(usize, Vec<Vec3>)>> = Mutex::new(None);

/// #### 한국어 </br>
/// 현재 눌려져있는 시스템 버튼의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed system button. </br>
/// 
static FOCUSED_SYS_BTN: Mutex<Option<(Buttons, Vec3)>> = Mutex::new(None);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Buttons {
    Return, 
    Infomation,
}



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
            label: Some("RenderPass(TitleScene(StageState(Background)))"),
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
            label: Some("RenderPass(TitleScene(EnterStage(Sprites)))"),
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

        // (한국어) 스프라이트 오브젝트들 그리기.
        // (English Translation) Drawing sprite objects.
        sprite_brush.draw(&mut rpass, this.sprites.iter().map(|(it, _)| it));
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

        // (한국어) 시스템 버튼 그리기.
        // (English Translation) Drawing the system buttons.
        ui_brush.draw(&mut rpass, [&this.return_button, &this.info_button].into_iter());
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

                    // (한국어) 스프라이트를 원래 색상으로 되돌립니다.
                    // (English Translation) Returns the sprite to its origin color.
                    {
                        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
                        let mut guard = FOCUSED_SPRITE.lock().expect("Failed to access variable.");
                        if let Some((index, sprite_colors)) = guard.take() {
                            if let Some((sprite, _)) = this.sprites.get(index) {
                                sprite.update(queue, |instances| {
                                    for (&sprite_color, instance) in sprite_colors.iter().zip(instances.iter_mut()) {
                                        instance.color = (sprite_color, instance.color.w).into();
                                    }
                                });
                            }
                        }
                    }

                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    {
                        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        if let Some((btn, ui_color)) = guard.take() {
                            match btn {
                                Buttons::Infomation => &this.info_button,
                                Buttons::Return => &this.return_button
                            }.update(queue, |data| data.color = (ui_color, data.color.w).into());
                        }
                    }


                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state.
                    this.state = TitleState::ExitStage;
                    this.timer = 0.0;
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
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
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
                    let select = this.sprites.iter()
                        .enumerate()
                        .find(|(_, (_, collider))| collider.test(&(x, y)));

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
                    if let Some((index, (sprite, _))) = select {
                        // <1>
                        let sprite_colors = sprite.instances.lock()
                            .expect("Failed to access variable.")
                            .iter()
                            .map(|data| data.color.xyz())
                            .collect();
                        let mut guard = FOCUSED_SPRITE.lock().expect("Failed to access variable.");
                        *guard = Some((index, sprite_colors));

                        // <2>
                        sprite.update(queue, |instances| {
                            for instance in instances.iter_mut() {
                                instance.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                            }
                        });

                        // <3>
                        sprite_pressed(Actor::from(index), this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_SPRITE.lock().expect("Failed to access variable.");
                    if let Some((index, sprite_colors)) = guard.take() {
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
                        if let Some((sprite, _)) = this.sprites.get_mut(index) {
                            sprite.update(queue, |instances| {
                                for (instance, sprite_color) in instances.iter_mut().zip(sprite_colors.iter()) {
                                    instance.color = (sprite_color.xyz(), instance.color.w).into();
                                }
                            });
                        };

                        // (한국어) 마우스 커서가 스프라이트 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the sprite area.
                        let select = this.sprites.iter()
                            .enumerate()
                            .find_map(|(idx, (_, collider))| {
                                if collider.test(&(x, y)) { Some(idx) } else { None }
                            });

                        // (한국어) 선택된 스프라이트가 이전에 선택된 스프라이트와 일치할 경우:
                        // (English Translation) If the selected sprite matches a previously selected sprite:
                        if select.is_some_and(|select| index == select) {
                            // (한국어) 스프라이트 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the sprite released function.
                            sprite_released(Actor::from(index), this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                // (한국어) 선택된 ui가 있는 경우:
                // (English Translation) If there is a selected ui:
                let guard = FOCUSED_SPRITE.lock().expect("Failed to access variable.");
                if let Some((index, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translation) Calls the ui dragged function.
                    sprite_dragged(Actor::from(*index), this, shared)?;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    };

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
                    let selected = [(Buttons::Return, &this.return_button), (Buttons::Infomation, &this.info_button)]
                        .into_iter()
                        .find_map(|(btn, ui)| ui.test(&(cursor_pos, camera)).then_some(btn));

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
                    if let Some(btn) = selected {
                        // <1>
                        let ui_color = match btn {
                            Buttons::Infomation => &this.info_button.data,
                            Buttons::Return => &this.return_button.data,
                        }.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                        *guard = Some((btn, ui_color));

                        // <2>
                        match btn {
                            Buttons::Infomation => &this.info_button,
                            Buttons::Return => &this.return_button
                        }.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));

                        // <3>
                        ui_pressed(btn, this, shared)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    let mut guard = FOCUSED_SYS_BTN.lock().expect("Failed to access variable.");
                    if let Some((btn, ui_color)) = guard.take() {
                        // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                        // (English Translation) Returns the color of the selected ui to its original color.
                        match btn {
                            Buttons::Infomation => &this.info_button, 
                            Buttons::Return => &this.return_button
                        }.update(queue, |data| data.color = (ui_color, data.color.w).into());
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area.
                        let selected = [(Buttons::Return, &this.return_button), (Buttons::Infomation, &this.info_button)]
                            .into_iter()
                            .find_map(|(btn, ui)| ui.test(&(cursor_pos, camera)).then_some(btn));

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if selected.is_some_and(|selected| selected == btn) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            ui_released(btn, this, shared)?;
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
                    ui_dragged(this, shared)?;
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
fn sprite_pressed(sp: Actor, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match sp {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
fn ui_pressed(btn: Buttons, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match btn {
        Buttons::Infomation => {
            if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                    let settings = shared.get::<Settings>().unwrap();
                    let asset_bundle = shared.get::<AssetBundle>().unwrap();
                    let source = asset_bundle.get(path::CLICK_SOUND_PATH)?.read(&sound::SoundDecoder)?;
                    sink.set_volume(settings.effect_volume.norm());
                    sink.append(source);
                    thread::spawn(move || {
                        sink.sleep_until_end();
                    });
                    shared.push((stream, stream_handle));
                }
            }
        },
        Buttons::Return => {
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
        },
    };

    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sprite_released(sp: Actor, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 선택된 스프라이트의 이미지를 변경합니다. 
    // (English Translation) Changes the image of the selected sprite. 
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let (sprite, _) = this.sprites.get(sp as usize).unwrap();
    sprite.update(queue, |instances| {
        for instance in instances.iter_mut() {
            instance.texture_index = 1;
        }
    });
    
    match sp {
        Actor::Aris => {
            shared.push(Actor::Aris);
            this.state = TitleState::EnterSelected;
            this.timer = 0.0;
            Ok(())
        },
        Actor::Momoi => {
            shared.push(Actor::Momoi);
            this.state = TitleState::EnterSelected;
            this.timer = 0.0;
            Ok(())
        },
        Actor::Midori => {
            shared.push(Actor::Midori);
            this.state = TitleState::EnterSelected;
            this.timer = 0.0;
            Ok(())
        },
        Actor::Yuzu => {
            shared.push(Actor::Yuzu);
            this.state = TitleState::EnterSelected;
            this.timer = 0.0;
            Ok(())
        },
        _ => Ok(())
    }
}


#[allow(unused_variables)]
fn ui_released(btn: Buttons, this: &mut TitleScene, _shared: &mut Shared) -> AppResult<()> {
    match btn {
        Buttons::Infomation => {
            this.state = TitleState::Tutorial0;
            this.timer = 0.0;
        },
        Buttons::Return => {
            this.state = TitleState::ExitStage;
            this.timer = 0.0;
        }
    }
    Ok(())
}


#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn sprite_dragged(sp: Actor, this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    match sp {
        _ => Ok(())
    }
}


#[allow(unused_variables)]
fn ui_dragged(this: &mut TitleScene, shared: &mut Shared) -> AppResult<()> {
    Ok(())
}
