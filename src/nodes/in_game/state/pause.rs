use std::thread;
use std::sync::{Arc, Mutex};

use glam::{Vec3, Vec4Swizzles, Vec4};
use rodio::{OutputStream, OutputStreamHandle};
use winit::{
    event::{Event, WindowEvent, MouseButton}, 
    keyboard::{PhysicalKey, KeyCode}, dpi::PhysicalPosition, 
};

use crate::{
    game_err,
    assets::bundle::AssetBundle,  
    components::{
        collider2d::Collider2d, 
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        table::TileBrush, 
        bullet::BulletBrush, 
        camera::GameCamera, 
        user::Settings, 
        sound, 
    },
    nodes::{
        path, 
        in_game::{
            utils, 
            InGameScene, 
            state::InGameState, 
        }
    },
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared, 
    }, 
};

/// #### 한국어 </br>
/// 현재 눌려져있는 버튼의 원래 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color data of the currently pressed button. </br>
/// 
static FOCUSED_PAUSE_BTN: Mutex<Option<(utils::PauseButton, Vec3, Vec3)>> = Mutex::new(None); 


pub fn handle_events(this: &mut InGameScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_keyboard_input(this, shared, &event)?;
    handle_mouse_input(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &InGameScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use. 
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let bullet_brush = shared.get::<Arc<BulletBrush>>().unwrap();

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

    // (한국어) 프레임 버퍼의 텍스처 뷰를 생성합니다.
    // (English Translation) Creates a texture view of the framebuffer.
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Pause(Background)))"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Store 
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            },
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        ui_brush.draw(
            &mut rpass, 
            [
                &this.background, 
                &this.stage_images[this.result_star_index.min(3)], 
                &this.player_faces[&this.player.face_state], 
                &this.boss_faces[&this.boss.face_state], 
            ].into_iter()
        );
        ui_brush.draw(&mut rpass, this.owned_hearts.iter());
        ui_brush.draw(&mut rpass, this.lost_hearts.iter().map(|(_, it)| it));
        tile_brush.draw(&mut rpass);
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Pause(Ui)))"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Store 
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            },
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [&this.menu_button, &this.remaining_timer_bg].into_iter());
        text_brush.draw(&mut rpass, [&this.remaining_timer_text, &this.percent].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Pause(Sprite)))"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Store 
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            },
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        sprite_brush.draw(&mut rpass, [&this.player.sprite, &this.boss.sprite].into_iter());
        bullet_brush.draw(&mut rpass, [&this.enemy_bullet].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Pause(Foreground)))"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store, 
                        },
                    }),
                ],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth.view(), 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store, 
                        }), 
                        stencil_ops: None, 
                    }
                ), 
                timestamp_writes: None,
                occlusion_query_set: None, 
            }
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [&this.foreground].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Pause(PauseUI)))"), 
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load, 
                            store: wgpu::StoreOp::Store, 
                        }, 
                    }), 
                ], 
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth.view(), 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Store, 
                        }),
                        stencil_ops: None, 
                    }
                ), 
                timestamp_writes: None, 
                occlusion_query_set: None, 
            }
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, this.pause_buttons.values().map(|(it, _)| it));
        text_brush.draw(&mut rpass, [&this.pause_text].into_iter());
        text_brush.draw(&mut rpass, this.pause_buttons.values().map(|(_, it)| it));
    }


    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}

fn handle_keyboard_input(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    let mut guard = FOCUSED_PAUSE_BTN.lock().expect("Failed to access variable.");
                    if let Some((tag, ui_color, text_color)) = guard.take() {
                        if let Some((ui, text)) = this.pause_buttons.get(&tag) {
                            ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                            text.update(queue, |data| data.color = (text_color, data.color.w).into());
                        }
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state. 
                    this.timer = 0.0;
                    this.state = InGameState::ExitPause; 
                }
            },
            _ => { /* empty */ }
        }, 
        _ => { /* empty */ }
    }
    Ok(())
}

fn handle_mouse_input(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap(); 
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } =>
            if MouseButton::Left == *button && state.is_pressed() {
                // (한국어) 마우스 커서가 ui영역 안에 있는지 확인합니다.
                // (English Translation) Make sure the mouse cursor is inside the ui area. 
                let select = this.pause_buttons.iter()
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
                // 3. Calls the ui pressed function. 
                //
                if let Some((tag, (ui, text))) = select {
                    // <1>
                    let ui_color = { ui.data.lock().expect("Failed to access variable.").color.xyz() };
                    let text_color = { text.data.lock().expect("Failed to access variable.").color.xyz() };
                    let mut guard = FOCUSED_PAUSE_BTN.lock().expect("Failed to access variaboe.");
                    *guard = Some((*tag, ui_color, text_color));

                    // <2>
                    ui.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                    text.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));

                    // <3>
                    btn_pressed(*tag, this, shared)?;
                }
            } else if MouseButton::Left == *button && !state.is_pressed() {
                let mut guard = FOCUSED_PAUSE_BTN.lock().expect("Failed to access variable.");
                if let Some((tag, ui_color, text_color)) = guard.take() {
                    // (한국어) 선택했던 ui의 색상을 원래 색상으로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    if let Some((ui, text)) = this.pause_buttons.get(&tag) {
                        ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                        text.update(queue, |data| data.color = (text_color, data.color.w).into());

                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area. 
                        if ui.test(&(cursor_pos, camera)) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Transaltion) Calls the ui release function.
                            btn_released(tag, this, shared)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                let guard = FOCUSED_PAUSE_BTN.lock().expect("Failed to access variable.");
                if let Some((tag, _, _)) = guard.as_ref() {
                    // (한국어) ui 끌림 함수를 호출합니다.
                    // (English Translatioin) Calls the ui dragged function.
                    btn_dragged(*tag, this, shared)?;
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
fn btn_pressed(tag: utils::PauseButton, this: &mut InGameScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        utils::PauseButton::Resume => {
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
        utils::PauseButton::Setting | utils::PauseButton::GiveUp => {
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
        }
        _ => { /* empty */ }
    };

    Ok(())
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn btn_released(tag: utils::PauseButton, this: &mut InGameScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        utils::PauseButton::Resume => {
            this.timer = 0.0;
            this.state = InGameState::ExitPause;
            Ok(())
        },
        utils::PauseButton::Setting => {
            shared.push(1usize);
            this.timer = 0.0;
            this.state = InGameState::EnterSetting;
            Ok(())
        },
        utils::PauseButton::GiveUp => {
            this.timer = 0.0;
            this.state = InGameState::EnterMsgBox;
            Ok(())
        }
        _ => Ok(())
    }
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn btn_dragged(tag: utils::PauseButton, this: &mut InGameScene, shared: &mut Shared) -> AppResult<()> {
    match tag {
        _ => Ok(())
    }
}
