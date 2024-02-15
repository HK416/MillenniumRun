use std::thread;
use std::sync::{Arc, Mutex};

use glam::{Vec4, Vec4Swizzles, Vec3};
use rodio::{Sink, OutputStream, OutputStreamHandle};
use winit::{
    window::Window, 
    keyboard::{KeyCode, PhysicalKey}, 
    event::{Event, WindowEvent, MouseButton}, 
    dpi::PhysicalPosition, 
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
        script::{ScriptDecoder, ScriptTags}, 
        sound, 
        user::{
            Language, 
            Resolution, 
            Settings, 
            SettingsEncoder, 
        }, 
    },
    nodes::{
        path, 
        in_game::{
            utils, 
            InGameScene, 
            state::InGameState, 
        },
    },
    render::depth::DepthBuffer, 
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared,
    },
};

/// #### 한국어 </br>
/// 선택된 설정창 인터페이스의 색상 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the color data of the selected settings window interface. </br>
/// 
static FOCUSED_ITEM: Mutex<Option<(Items, Vec3, Vec3)>> = Mutex::new(None);


/// #### 한국어 </br>
/// 설정창의 인터페이스 옵션 목록입니다. </br> 
/// 
/// #### English (Translation) </br>
/// This is a list of interface options in the setting window. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Items {
    Language(Language), 
    Resolution(Resolution), 
    Volume(utils::VolumeOptions), 
    Return, 
}

pub fn handle_events(this: &mut InGameScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_keyboard_input(this, shared, &event)?;
    handle_mouse_input(this, shared, &event)?;
    Ok(())
}

pub fn update(_this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    Ok(())
}

pub fn draw(this: &InGameScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();
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
                label: Some("RenderPass(InGameScene(Setting(Background)))"), 
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
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth.view(), 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Store, 
                        }),
                        stencil_ops: None,
                    },
                ),
                timestamp_writes: None,
                occlusion_query_set: None, 
            },
        );

        camera.bind(&mut rpass);

        let iter = [
                &this.background, 
                &this.stage_images[this.result_star_index.min(3)], 
                &this.player_faces[&this.player.face_state], 
                &this.boss_faces[&this.boss.face_state], 
                &this.menu_button, 
                &this.remaining_timer_bg, 
            ].into_iter()
            .chain(this.owned_hearts.iter())
            .chain(this.lost_hearts.iter().map(|(_, it)| it));
        ui_brush.draw(&mut rpass, iter);

        text_brush.draw(&mut rpass, [
            &this.remaining_timer_text, 
            &this.percent, 
        ].into_iter());

        tile_brush.draw(&mut rpass);
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Setting(Sprite)))"),
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

        camera.bind(&mut rpass);
        
        sprite_brush.draw(&mut rpass, [
            &this.player.sprite, 
            &this.boss.sprite, 
        ].into_iter());

        bullet_brush.draw(&mut rpass, [&this.enemy_bullet].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Setting(Foreground)))"),
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

        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [&this.foreground].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Setting(SettingUI)))"),
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

        camera.bind(&mut rpass);

        let iter = [
                &this.setting_return_button.0, 
            ].into_iter()
            .chain(this.setting_windows.iter())
            .chain(this.setting_languages.values().map(|(it, _)| it))
            .chain(this.setting_resolutions.values().map(|(it, _)| it))
            .chain(this.setting_volume_background.values().map(|(it, _)| it))
            .chain(this.setting_volume_bar.values());
        ui_brush.draw(&mut rpass, iter);

        let iter = [
                &this.setting_return_button.1, 
            ].into_iter()
            .chain(this.setting_titles.iter())
            .chain(this.setting_languages.values().map(|(_, it)| it))
            .chain(this.setting_resolutions.values().map(|(_, it)| it))
            .chain(this.setting_volume_background.values().map(|(_, it)| it));
        text_brush.draw(&mut rpass, iter);
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}

fn handle_keyboard_input(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && event.state.is_pressed() {
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

                    // (한국어) 선택된 설정 창 인터페이스를 원래 상태로 되돌립니다.
                    // (English Translation) Returns the selected settings window interface to its original state. 
                    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
                    let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                    if let Some((item, ui_color, text_color)) = guard.take() {
                        match item {
                            Items::Language(it) => {
                                if let Some((ui, text)) = this.setting_languages.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                    text.update(queue, |data| data.color = (text_color, data.color.w).into());
                                }
                            }, 
                            Items::Resolution(it) => {
                                if let Some((ui, text)) = this.setting_resolutions.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                    text.update(queue, |data| data.color = (text_color, data.color.w).into());
                                }
                            },
                            Items::Volume(it) => {
                                if let Some(ui) = this.setting_volume_bar.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                }
                            },
                            Items::Return => {
                                this.setting_return_button.0.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                this.setting_return_button.1.update(queue, |data| data.color = (text_color, data.color.w).into());
                            }
                        }
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다. 
                    // (English Translation) Change to the next game scene state. 
                    this.timer = 0.0;
                    this.state = InGameState::ExitSetting;
                }
            }, 
            _ => { /* empty */ }
        }, 
        _ => { /* empty */ }
    };

    Ok(())
}

fn handle_mouse_input(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == *button && state.is_pressed() {
                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area. 
                    let select = [
                            (Items::Return, &this.setting_return_button.0), 
                        ].into_iter()
                        .chain(this.setting_languages.iter().map(|(&language, (it, _))| (Items::Language(language), it)))
                        .chain(this.setting_resolutions.iter().map(|(&resolution, (it, _))| (Items::Resolution(resolution), it)))
                        .chain(this.setting_volume_bar.iter().map(|(&volume, it)| (Items::Volume(volume), it)))
                        .find_map(|(it, ui)| {
                            ui.test(&(cursor_pos, camera)).then_some(it)
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
                    if let Some(item) = select {
                        match item {
                            Items::Language(language) => {
                                if let Some((ui, text)) = this.setting_languages.get(&language) {
                                    let ui_color = { ui.data.lock().expect("Failed to access variable.").color.xyz() };
                                    let text_color = { text.data.lock().expect("Failed to access variable.").color.xyz() };
                                    
                                    let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                                    *guard = Some((item, ui_color, text_color));

                                    ui.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                    text.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                }
                            },
                            Items::Resolution(resolution) => {
                                if let Some((ui, text)) = this.setting_resolutions.get(&resolution) {
                                    let ui_color = { ui.data.lock().expect("Failed to access variable.").color.xyz() };
                                    let text_color = { text.data.lock().expect("Failed to access variable.").color.xyz() };
                                    
                                    let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                                    *guard = Some((item, ui_color, text_color));

                                    ui.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                    text.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                }
                            },
                            Items::Volume(volume) => {
                                if let Some(ui) = this.setting_volume_bar.get(&volume) {
                                    let ui_color = { ui.data.lock().expect("Failed to access variable.").color.xyz() };

                                    let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                                    *guard = Some((item, ui_color, Vec3::ZERO));

                                    ui.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                }
                            },
                            Items::Return => {
                                let ui_color = { this.setting_return_button.0.data.lock().expect("Failed to access variable.").color.xyz() };
                                let text_color = { this.setting_return_button.1.data.lock().expect("Failed to access variable.").color.xyz() };

                                let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                                *guard = Some((item, ui_color, text_color));

                                this.setting_return_button.0.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                                this.setting_return_button.1.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));
                            }
                        };

                        ui_pressed(this, shared, item)?;
                    }
                } else if MouseButton::Left == *button && !state.is_pressed() {
                    // (한국어) 선택된 설정 창 인터페이스를 원래 상태로 되돌립니다.
                    // (English Translation) Returns the selected settings window interface to its original state. 
                    let mut guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                    if let Some((item, ui_color, text_color)) = guard.take() {
                        match item {
                            Items::Language(it) => {
                                if let Some((ui, text)) = this.setting_languages.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                    text.update(queue, |data| data.color = (text_color, data.color.w).into());
                                }
                            }, 
                            Items::Resolution(it) => {
                                if let Some((ui, text)) = this.setting_resolutions.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                    text.update(queue, |data| data.color = (text_color, data.color.w).into());
                                }
                            },
                            Items::Volume(it) => {
                                if let Some(ui) = this.setting_volume_bar.get(&it) {
                                    ui.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                    return ui_released(this, shared, item);
                                }
                            },
                            Items::Return => {
                                this.setting_return_button.0.update(queue, |data| data.color = (ui_color, data.color.w).into());
                                this.setting_return_button.1.update(queue, |data| data.color = (text_color, data.color.w).into());
                            }
                        };
                        
                        // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the ui area. 
                        let select = [
                                (Items::Return, &this.setting_return_button), 
                            ].into_iter()
                            .chain(this.setting_languages.iter().map(|(&language, it)| (Items::Language(language), it)))
                            .chain(this.setting_resolutions.iter().map(|(&resolution, it)| (Items::Resolution(resolution), it)))
                            .find_map(|(it, (ui, _))| {
                                ui.test(&(cursor_pos, camera)).then_some(it)
                            });

                        // (한국어) 선택된 ui가 이전에 선택된 ui와 일치하는 경우:
                        // (English Translation) If the selected ui matches a previously selected ui:
                        if select.is_some_and(|select| select == item) {
                            // (한국어) ui 떼어짐 함수를 호출합니다.
                            // (English Translation) Calls the ui released function.
                            ui_released(this, shared, item)?;
                        }
                    }
                }
            },
            WindowEvent::CursorMoved { .. } => {
                let guard = FOCUSED_ITEM.lock().expect("Failed to access variable.");
                if let Some((item, _, _)) = guard.as_ref() {
                    ui_dragged(this, shared, *item)?;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }

    Ok(())
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_pressed(this: &mut InGameScene, shared: &mut Shared, item: Items) -> AppResult<()> {
    match item {
        Items::Language(_) | Items::Resolution(_) => {
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
        Items::Return => {
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
        _ => { /* empty */ }
    };

    Ok(())
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_released(this: &mut InGameScene, shared: &mut Shared, item: Items) -> AppResult<()> {
    match item {
        Items::Language(new) => {
            change_language(this, shared, new)?;
        },
        Items::Resolution(new) => {
            change_resolution(this, shared, new)?;
        },
        Items::Volume(option) => match option {
            utils::VolumeOptions::Background => { /* empty */ }, 
            utils::VolumeOptions::Effect => {
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
            utils::VolumeOptions::Voice => {
                const NUM_HIDDEN: usize = 7;

                // (한국어) 사용할 공유 객체들을 가져옵니다.
                // (English Translation) Get shared objects to use.
                let cnt = shared.pop::<usize>().unwrap();
                if let Some((_, voice)) = shared.get::<(Sink, Sink)>() {
                    let settings = shared.get::<Settings>().unwrap();
                    let asset_bundle = shared.get::<AssetBundle>().unwrap();

                    // (한국어) 캐릭터 목소리 데이터를 가져옵니다.
                    // (English Translation) Get character voice data. 
                    let source = match cnt % NUM_HIDDEN == 0 {
                        true => asset_bundle.get(path::YUUKA_HIDDEN_SOUND_PATH)?.read(&sound::SoundDecoder)?,
                        false => asset_bundle.get(path::YUUKA_TITLE_SOUND_PATH)?.read(&sound::SoundDecoder)?
                    };

                    // (한국어) 캐릭터 목소리를 재생시킵니다.
                    // (English Translation) Play the character's voice.
                    voice.stop();
                    voice.set_volume(settings.voice_volume.norm());
                    voice.append(source);

                    // (한국어) 사용한 공유 객체들 반환합니다.
                    // (English Translation) Returns the shared objects used. 
                }
                shared.push((cnt + 1) % NUM_HIDDEN);
            },
        },
        Items::Return => {
            this.timer = 0.0;
            this.state = InGameState::ExitSetting;
        },
        _ => { /* empty */ }
    };

    Ok(())
}

#[allow(unused_variables)]
#[allow(unreachable_patterns)]
fn ui_dragged(this: &mut InGameScene, shared: &mut Shared, item: Items) -> AppResult<()> {
    match item {
        Items::Volume(option) => {
            // (한국어) 사용할 공유 객체들을 가져옵니다.
            // (English Translation) Get shared object to use. 
            let mut settings = shared.pop::<Settings>().unwrap();
            let camera = shared.get::<Arc<GameCamera>>().unwrap();
            let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();

            // (한국어) 인터페이스의 위치를 계산합니다.
            // (English Translation) Calculate the position of the interface. 
            const RANGE: i32 = utils::SETTING_VOLUME_RANGE_MAX - utils::SETTING_VOLUME_RANGE_MIN;
            let (scale, center) = {
                let guard = camera.data.lock().expect("Failed to access variable.");
                (guard.scale_factor, guard.viewport.x + guard.viewport.width / 2.0)
            };
            
            let pos = (cursor_pos.x as f32 - center).clamp(
                utils::SETTING_VOLUME_RANGE_MIN as f32 * scale, 
                utils::SETTING_VOLUME_RANGE_MAX as f32 * scale
            ) / scale;
            let pos = pos as i32;

            // (한국어) 인터페이스의 위치를 갱신합니다.
            // (English Translation) Updates the position of the interface.
            let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
            if let Some(ui) = this.setting_volume_bar.get(&option) {
                ui.update(queue, |data| {
                    data.margin.set_left(pos - utils::VOLUME_BAR_WIDTH / 2);
                    data.margin.set_right(pos + utils::VOLUME_BAR_WIDTH / 2);
                });
            }
            
            // (한국어) 계산된 볼륨 값을 설정합니다.
            // (English Translation) Sets the calculated volume value. 
            let delta = pos - utils::SETTING_VOLUME_RANGE_MIN;
            let volume = (delta as f32 / RANGE as f32 * 100.0) as u8;
            match option {
                utils::VolumeOptions::Background => {
                    settings.background_volume.set(volume);
                    if let Some((background, _)) = shared.get::<(Sink, Sink)>() {
                        background.set_volume(settings.background_volume.norm());
                    }
                }, 
                utils::VolumeOptions::Effect => settings.effect_volume.set(volume),
                utils::VolumeOptions::Voice => {
                    settings.voice_volume.set(volume);
                    if let Some((_, voice)) = shared.get::<(Sink, Sink)>() {
                        voice.set_volume(settings.voice_volume.norm());
                    }
                }, 
            };

            // (한국어) 갱신된 설정을 저장합니다.
            // (English Translation) Save updated settings. 
            asset_bundle.get(path::SETTINGS_PATH)?.write(&SettingsEncoder, &settings)?;
            shared.push(settings);

            Ok(())
        },
        _ => Ok(())
    }
}

fn change_language(this: &mut InGameScene, shared: &mut Shared, new: Language) -> AppResult<()> {    
    // (한국어) 현재 설정된 언어와 같을 경우 실행하지 않습니다.
    // (English Translation) If it is the same as the currently set language, it will not run.
    let settings = shared.get::<Settings>().unwrap();
    if settings.language == new {
        return Ok(())
    }

    // (한국어) 사용자가 선택한 언어로 설정합니다.
    // (English Translation) Set to the language selected by the user.
    let mut settings = shared.pop::<Settings>().unwrap();
    settings.language = new;

    // (한국어) 설정된 언어의 스크립트 파일을 불러옵니다.
    // (English Translation) Loads the script file of the set language.
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let script = match settings.language {
        Language::Korean => asset_bundle.get(path::KOR_SCRIPTS_PATH)?
            .read(&ScriptDecoder)?, 
        Language::Unknown => panic!("The given language is an unknown language.")
    };

    // (한국어) 현재 게임 장면의 표시 언어를 변경합니다.
    // (English Translation) Change the display language of the current game scene. 
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    this.pause_text.change(
        script.get(ScriptTags::InGamePauseTitle)?, 
        device, 
        queue, 
        text_brush
    );

    const PAUSE_BTN: [(utils::PauseButton, ScriptTags); 3] = [
        (utils::PauseButton::Resume, ScriptTags::InGameResumeButton), 
        (utils::PauseButton::Setting, ScriptTags::InGameSettingButton), 
        (utils::PauseButton::GiveUp, ScriptTags::InGameGiveUpButton), 
    ];
    for (key, tag) in PAUSE_BTN {
        this.pause_buttons.get_mut(&key).unwrap().1.change(
            script.get(tag)?, 
            device, 
            queue, 
            text_brush
        );
    }

    const MSG_BOX: [(utils::ExitWndButton, ScriptTags); 2] = [
        (utils::ExitWndButton::No, ScriptTags::InGameGiveUpCancelButton),
        (utils::ExitWndButton::Yes, ScriptTags::InGameGiveUpOkayButton), 
    ];
    this.pause_exit_window.1.change(
        script.get(ScriptTags::InGameGiveUpReconfirmMessage)?, 
        device, 
        queue, 
        text_brush
    );
    for (key, tag) in MSG_BOX {
        this.pause_exit_buttons.get_mut(&key).unwrap().1.change(
            script.get(tag)?, 
            device, 
            queue, 
            text_brush
        );
    }

    this.result_window_btn.1.change(
        script.get(ScriptTags::InGameExitButton)?, 
        device, 
        queue, 
        text_brush
    );
    const CHALLENGE: [ScriptTags; 3] = [
        ScriptTags::InGameChallenge0, 
        ScriptTags::InGameChallenge1, 
        ScriptTags::InGameChallenge2, 
    ];
    for (idx, tag) in CHALLENGE.into_iter().enumerate() {
        this.result_challenge_texts[idx].change(
            script.get(tag)?, 
            device, 
            queue, 
            text_brush
        );
    }



    const SETTING_TITLES: [ScriptTags; 7] = [
        ScriptTags::SettingTitle, 
        ScriptTags::SettingLanguageOptionTitle, 
        ScriptTags::SettingLanguageOptionSubTitle,
        ScriptTags::SettingResolutionOptionTitle, 
        ScriptTags::SettingResolutionOptionSubTitle, 
        ScriptTags::SettingVolumeOptionTitle, 
        ScriptTags::SettingVolumeOptionSubTitle,  
    ];
    for (idx, tag) in SETTING_TITLES.into_iter().enumerate() {
        this.setting_titles[idx].change(
            script.get(tag)?, 
            device, 
            queue, 
            text_brush
        );
    }
    this.setting_return_button.1.change(
        script.get(ScriptTags::SettingReturnButton)?, 
        device, 
        queue, 
        text_brush
    );

    // (한국어) 설정 파일을 갱신합니다.
    // (English Translation) Updates the settings file.
    asset_bundle.get(path::SETTINGS_PATH)?.write(&SettingsEncoder, &settings)?;

    // (한국어) 공유 객체를 갱신합니다.
    // (English Translation) Updates a shared object. 
    shared.push(settings);
    shared.push(Arc::new(script));

    Ok(())
}

fn change_resolution(_this: &mut InGameScene, shared: &mut Shared, new: Resolution) -> AppResult<()> {
    use crate::components::user::set_window_size;

    // (한국어) 현재 해상도와 같을 경우 실행하지 않습니다.
    // (English Translation) If it is the same as the current resolution, it will not run.
    let settings = shared.get::<Settings>().unwrap();
    if settings.resolution == new {
        return Ok(());
    }

    // (한국어) 사용자가 선택한 해상도로 설정합니다.
    // (English Translation) Set to the resolution selected by the user.
    let mut settings = shared.pop::<Settings>().unwrap();
    let window = shared.get::<Arc<Window>>().unwrap();
    settings.resolution = set_window_size(window, new)?;
    

    // (한국어) 설정 파일을 갱신합니다.
    // (English Translation) Updates the settings file.
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    asset_bundle.get(path::SETTINGS_PATH)?.write(&SettingsEncoder, &settings)?;

    // (한국어) 공유 객체를 갱신합니다.
    // (English Translation) Updates a shared object. 
    shared.push(settings);

    Ok(())
}
