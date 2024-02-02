use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use rand::prelude::*;
use glam::{Vec3, Vec4Swizzles, Vec4};
use rodio::{Sink, OutputStreamHandle};
use winit::{
    keyboard::{PhysicalKey, KeyCode},
    event::{Event, WindowEvent, MouseButton},
    dpi::PhysicalPosition, 
};

use crate::{
    game_err,
    assets::bundle::AssetBundle, 
    components::{
        collider2d::Collider2d, 
        text::TextBrush,
        ui::{UiBrush, UiObject},
        camera::GameCamera,
        sprite::SpriteBrush,
        user::Settings,
        table::{self, TileBrush},
        bullet::{self, BulletBrush, Instance as BulletData}, 
        player::{self, Player, PlayerControlState, PlayerFaceState, PlayerGameState}, 
        boss::{self, Boss, BossFaceState}, 
        sound::{self, SoundDecoder}, 
        interpolation, 
    },
    nodes::in_game::{
        utils, 
        InGameScene, 
        state::InGameState, 
    },
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};

/// #### 한국어 </br>
/// 현재 선택된 버튼의 원래 색상을 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the original color of the currently selected button. </br>
/// 
static FOCUSED_MENU_BTN: Mutex<Option<Vec3>> = Mutex::new(None);


pub fn handle_events(this: &mut InGameScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_player_mouse_events(this, shared, &event)?;
    handle_player_keyboard_events(this, shared, &event)?;
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    player_update(this, shared, total_time, elapsed_time)?;
    update_boss(this, shared, total_time, elapsed_time)?;
    
    update_bullets(this, shared, total_time, elapsed_time)?;
    
    handles_collision(this, shared, total_time, elapsed_time)?;

    update_lost_hearts(this, shared, total_time, elapsed_time)?;
    update_owned_tiles(this, shared, total_time, elapsed_time)?;

    update_percent_text(this, shared, total_time, elapsed_time)?;
    update_remaining_time(this, shared, total_time, elapsed_time)?;
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
                label: Some("RenderPass(InGameScene(Run(Background)))"),
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
                label: Some("RenderPass(InGameScene(Run(Ui)))"),
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
                label: Some("RenderPass(InGameScene(Run(Sprite)))"),
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

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}



/// #### 한국어 </br>
/// 플레이어의 마우스 입력 이벤트를 처리합니다. </br>
/// 
/// #### English (Translation) </br>
/// Handles the player's mouse input events. </br>
/// 
fn handle_player_mouse_events(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    use crate::nodes::path;

    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared object to use.
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => 
            if MouseButton::Left == *button && state.is_pressed() {
                // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                // (English Translation) Make sure the mouse cursor is inside the ui area. 
                let is_inside = this.menu_button.test(&(cursor_pos, camera));

                // (한국어)
                // 마우스 커서가 ui 영역 안에 있는 경우:
                // 1. `FOCUSED`에 해당 ui의 색상을 저장합니다.
                // 2. 해당 ui의 색상을 변경합니다.
                //
                // (English Translation)
                // If the mouse cursor is inside the ui area:
                // 1. Store the ui color in `FOCUSED`.
                // 2. Change the color of the ui.
                //
                if is_inside {
                    // <1>
                    let ui_color = this.menu_button.data.lock().expect("Failed to access variable.").color.xyz();
                    let mut guard = FOCUSED_MENU_BTN.lock().expect("Failed to access variable.");
                    *guard = Some(ui_color);

                    // <2>
                    this.menu_button.update(queue, |data| {
                        data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0);
                    });

                    // (한국어) 일시정지 사운드를 재생합니다.
                    // (English Translation) Play pause sound. 
                    let source = asset_bundle.get(path::PAUSE_SOUND_PATH)?
                        .read(&SoundDecoder)?;
                    let sink = sound::play_sound(settings.effect_volume, source, stream)?;
                    thread::spawn(move || {
                        sink.sleep_until_end();
                        sink.detach();
                    });
                }
            } else if MouseButton::Left == *button && !state.is_pressed() {
                let mut guard = FOCUSED_MENU_BTN.lock().expect("Failed to access variable.");
                if let Some(ui_color) = guard.take() {
                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다.
                    // (English Translation) Returns the color of the selected ui to its original color.
                    this.menu_button.update(queue, |data| {
                        data.color = (ui_color, data.color.w).into();
                    });

                    // (한국어) 마우스 커서가 ui 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the ui area. 
                    let is_inside = this.menu_button.test(&(cursor_pos, camera));

                    // (한국어) 마우스 커서가 ui 영역 안에 있는 경우.
                    // (English Translation) When the mouse cursor is inside the ui area. 
                    if is_inside {
                        // (한국어) 일시정지 상태로 변경합니다. 
                        // (English Translation) Changes to pause state. 
                        this.timer = 0.0;
                        this.state = InGameState::EnterPause;
                        this.player.control_state = PlayerControlState::Idle;
                    }
                }
            }
            _ => { /* empty */ }
        }, 
        _ => { /* empty */ }
    }
    Ok(())
}

/// #### 한국어 </br>
/// 플레이어의 키보드 입력 이벤트를 처리합니다. </br>
/// 
/// #### English (Translation) </br>
/// Handles the player's keyboard input events. </br>
/// 
fn handle_player_keyboard_events(this: &mut InGameScene, shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    use crate::nodes::path;

    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let control = &settings.control;
    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                // (한국어) 사용자가 `ESC`키를 눌렀을 경우.
                // (English Translation) When the user presses the `ESC` key.
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    // (한국어) 일시정지 사운드를 재생합니다.
                    // (English Translation) Play pause sound. 
                    let source = asset_bundle.get(path::PAUSE_SOUND_PATH)?
                        .read(&SoundDecoder)?;
                    let sink = sound::play_sound(settings.effect_volume, source, stream)?;
                    thread::spawn(move || {
                        sink.sleep_until_end();
                        sink.detach();
                    });

                    // (한국어) 선택했던 ui의 색상을 원래대로 되돌립니다. 
                    // (English Translation) Returns the color of the selected ui to its original color.
                    let mut guard = FOCUSED_MENU_BTN.lock().expect("Failed to access variable.");
                    if let Some(ui_color) = guard.take() {
                        this.menu_button.update(queue, |data| {
                            data.color = (ui_color, data.color.w).into();
                        });
                    }

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state. 
                    this.timer = 0.0;
                    this.state = InGameState::EnterPause; 
                    this.player.control_state = PlayerControlState::Idle;
                }

                // (한국어) 사용자가 `위쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Up` key.
                if control.up.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == PlayerControlState::Down {
                        return Ok(());
                    }
                    this.player.control_state = PlayerControlState::Up;
                }

                // (한국어) 사용자가 `위쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Up` key.
                if control.up.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == PlayerControlState::Up {
                    this.player.control_state = PlayerControlState::Idle;
                }


                // (한국어) 사용자가 `아래쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Down` key.
                if control.down.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == PlayerControlState::Up {
                        return Ok(());
                    }
                    this.player.control_state = PlayerControlState::Down;
                }

                // (한국어) 사용자가 `아래쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Down` key.
                if control.down.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == PlayerControlState::Down {
                    this.player.control_state = PlayerControlState::Idle;
                }


                // (한국어) 사용자가 `왼쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Left` key.
                if control.left.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == PlayerControlState::Right {
                        return Ok(());
                    }
                    this.player.control_state = PlayerControlState::Left;
                }

                // (한국어) 사용자가 `왼쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Left` key.
                if control.left.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == PlayerControlState::Left {
                    this.player.control_state = PlayerControlState::Idle;
                }


                // (한국어) 사용자가 `오른쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Right` key.
                if control.right.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == PlayerControlState::Left {
                        return Ok(());
                    }
                    this.player.control_state = PlayerControlState::Right;
                }

                // (한국어) 사용자가 `오른쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Right` key.
                if control.right.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == PlayerControlState::Right {
                    this.player.control_state = PlayerControlState::Idle;
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }
    Ok(())
}

/// #### 한국어 </br>
/// 남은 시간을 표시하는 사용자 인터페이스를 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Update the user interface to display time remaining. </br>
/// 
fn update_remaining_time(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.remaining_time = (this.remaining_time - elapsed_time).max(0.0);
    
    // (한국어) 사용자 인터페이스를 새로 생성합니다. 
    // (English Translation) Create a new user interface. 
    let min = (this.remaining_time / 60.0) as u32;
    let sec = (this.remaining_time % 60.0) as u32;
    this.remaining_timer_text.change(
        &format!("{}:{:0>2}", min, sec), 
        device, 
        queue, 
        &text_brush.tex_sampler, 
        &text_brush.texture_layout
    );

    if this.remaining_time <= 0.0 {
        audio.voice.stop();

        this.player.face_timer = 0.0;
        this.player.face_state = PlayerFaceState::Hit;
        this.player.sprite.update(queue, |instances| {
            instances[0].texture_index = PlayerFaceState::Hit as u32;
        });

        this.boss.face_timer = 0.0;
        this.boss.face_state = BossFaceState::Smile;
        this.boss.sprite.update(queue, |instances| {
            instances[0].texture_index = BossFaceState::Smile as u32;
        });

        this.timer = 0.0;
        this.state = InGameState::WaitForFinish;
    }
    
    Ok(())
}

/// #### 한국어 </br>
/// 소유한 타일을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates owned tiles. </br>
/// 
fn update_owned_tiles(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 0.4;

    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();

    let mut next = VecDeque::with_capacity(this.owned_tiles.capacity());
    while let Some((mut timer, tiles)) = this.owned_tiles.pop_front() {
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Updates the timer.
        timer += elapsed_time;

        // (한국어) 타일의 투명도를 갱신합니다.
        // (English Translation) Updates the transparency of the tile. 
        let delta = interpolation::f64::smooth_step(timer, DURATION) as f32;
        let alpha = 1.0 - 1.0 * delta;

        for &(row, col) in tiles.iter() {
            this.table.tiles[row][col].color.w = alpha;
        }

        tile_brush.update(queue, |instances| {
            for &(row, col) in tiles.iter() {
                instances[row * this.table.num_cols + col].color = this.table.tiles[row][col].color;
            }
        });


        if timer < DURATION {
            next.push_back((timer, tiles));
        }
    }

    this.owned_tiles = next;
    Ok(())
}

/// #### 한국어 </br>
/// 잃어버린 체력 하트 오브젝트를 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates lost health heart objects. </br>
/// 
fn update_lost_hearts(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 0.4;

    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    let mut next = VecDeque::with_capacity(player::MAX_PLAYER_HEARTS);
    while let Some((mut timer, heart)) = this.lost_hearts.pop_front() {
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Updates the timer.
        timer += elapsed_time;

        // (한국어) 하트의 크기를 갱신합니다.
        // (English Translation) Update the size of heart.
        let delta = interpolation::f64::smooth_step(timer, DURATION) as f32;
        let scale = 1.0 - 1.0 * delta;
        heart.update(queue, |data| {
            data.local_scale = (scale, scale, scale).into();
        });

        if timer < DURATION {
            next.push_back((timer, heart));
        }
    }

    this.lost_hearts = next;
    Ok(())
}

/// #### 한국어 </br>
/// 플레이어를 갱신하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This function updates the player. </br>
/// 
fn player_update(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();
    let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();

    player::update_player_face(elapsed_time, queue, &mut this.player);
    player::update_player_game_state(elapsed_time, queue, &mut this.player);

    player::translation_player(
        elapsed_time, 
        &this.table, 
        &mut this.player, 
        &queue
    );

    if let Some(flag) = player::check_current_pos(
        &mut this.table, 
        &mut this.player, 
        tile_brush, 
        queue
    ) {
        if flag {
            table::update_owned_tiles(
                queue, 
                tile_brush, 
                &mut this.table, 
                &mut this.player.path, 
                &mut this.num_owned_tiles, 
                &mut this.owned_tiles
            );

            // (한국어) 결과 점수의 인덱스를 갱신합니다.
            // (English Translation) Update the index of the resulting score.
            let percent = this.num_owned_tiles as f32 / this.num_total_tiles as f32 * 100.0;
            if percent < 20.0 {
                this.result_star_index = 0;
            } else if 20.0 <= percent && percent < 50.0 {
                for text in this.result_condition_texts[0..=0].iter() {
                    text.update(queue, |data| {
                        data.color = (255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0, 0.0).into();
                    });
                }
                this.result_star_index = 1;
            } else if 50.0 <= percent && percent < 80.0 {
                for text in this.result_condition_texts[0..=1].iter() {
                    text.update(queue, |data| {
                        data.color = (255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0, 0.0).into();
                    });
                }
                this.result_star_index = 2;
            } else if 80.0 <= percent  && percent < 100.0 {
                for text in this.result_condition_texts[0..=2].iter() {
                    text.update(queue, |data| {
                        data.color = (255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0, 0.0).into();
                    });
                }
                this.result_star_index = 3;
            } else {
                for text in this.result_condition_texts[0..=2].iter() {
                    text.update(queue, |data| {
                        data.color = (255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0, 0.0).into();
                    });
                }
                this.result_star_index = 4;
            }


            // (한국어) 퍼센트 인터페이스를 갱신합니다.
            // (English Translation) Updates the percent interface. 
            this.percent_timer = 0.0;

            // (한국어) 플레이어의 표정을 웃는 표정으로 변경합니다.
            // (English Translation) Changes the player's face to a smiley face. 
            this.player.face_timer = 0.0;
            this.player.face_state = PlayerFaceState::Smile;
            this.player.sprite.update(queue, |instances| {
                instances[0].texture_index = PlayerFaceState::Smile as u32;
            });

            // (한국어) 무작위로 캐릭터 목소리를 재생합니다.
            // (English Translation) Plays character voices randomly. 
            if rand::thread_rng().gen_bool(0.3) {
                play_random_character_voice(
                    &this.player_smile_sounds, 
                    &audio.voice, 
                    asset_bundle
                )?;
            }
        } else {
            // (한국어) 플레이어의 라이프 카운트를 감소시킵니다.
            // (English Translation) Decreases the player's life count.
            let remaining_life = decrease_player_life_count(
                &mut this.owned_hearts, 
                &mut this.lost_hearts
            );

            if remaining_life == 0 {
                audio.voice.stop();

                tile_brush.update(queue, |instances| {
                    for &(r, c) in this.player.path.iter() {
                        instances[r * this.table.num_cols + c].color = this.table.tiles[r][c].color;
                    }
                });
                while let Some((r, c)) = this.player.path.pop_front() {
                    this.table.tiles[r][c].visited = false;
                }

                this.player.face_timer = 0.0;
                this.player.face_state = PlayerFaceState::Hit;
                this.player.sprite.update(queue, |instances| {
                    instances[0].texture_index = PlayerFaceState::Hit as u32;
                });
    
                this.boss.face_timer = 0.0;
                this.boss.face_state = BossFaceState::Smile;
                this.boss.sprite.update(queue, |instances| {
                    instances[0].texture_index = BossFaceState::Smile as u32;
                });

                this.timer = 0.0;
                this.state = InGameState::WaitForFinish;
            } else {
                // (한국어) 플레이어를 스폰위치로 이동시키고, 타일을 원래 상태로 되돌립니다.
                // (English Translation) Moves the player to the spawn position and returns the tile to its original state. 
                player::restore(
                    queue, 
                    &mut this.table, 
                    &mut this.boss, 
                    &mut this.player, 
                    tile_brush
                );

                play_random_character_voice(
                    &this.player_damage_sounds, 
                    &audio.voice, 
                    asset_bundle
                )?;
            }
        }
    };

    player::set_player_next_position(
        &this.table, 
        &mut this.player
    );

    Ok(())
}

/// #### 한국어 </br>
/// 플레이어가 차지한 영역의 비율을 보여주는 텍스트를 갱신하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This function updates text showing the percentage of area occupied by the player. </br>
/// 
fn update_percent_text(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.percent_timer += elapsed_time;

    let per = this.num_owned_tiles as f32 /  this.num_total_tiles as f32 * 100.0;
    let s = 1.0 + 0.5 - 0.5 * interpolation::f64::smooth_step(this.percent_timer, 0.25) as f32;
    this.percent.change(
        &format!("{}%", per.floor() as u32), 
        device, 
        queue, 
        &text_brush.tex_sampler, 
        &text_brush.texture_layout
    );
    this.percent.update(queue, |data| {
        data.scale = (s, s, s).into();
    });

    // (한국어) 플레이어가 모든 타일을 차지한 경우 다음 장면 상태로 변경합니다.
    // (English Translation) When a player occupies all tiles, they change to the next scene state. 
    if per >= 100.0 {
        audio.voice.stop();

        this.player.face_timer = 0.0;
        this.player.face_state = PlayerFaceState::Smile;
        this.player.sprite.update(queue, |instances| {
            instances[0].texture_index = PlayerFaceState::Smile as u32;
        });

        this.boss.face_timer = 0.0;
        this.boss.face_state = BossFaceState::Embarrass;
        this.boss.sprite.update(queue, |instances| {
            instances[0].texture_index = BossFaceState::Embarrass as u32;
        });

        this.timer = 0.0;
        this.state = InGameState::WaitForFinish;
    }

    Ok(())
}

/// #### 한국어  </br>
/// 발사된 총알들을 갱신하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This function updates the bullets fired bullets. </br>
/// 
fn update_bullets(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    bullet::update_bullets(
        shared.get::<Arc<wgpu::Queue>>().unwrap(), 
        &this.table, 
        &this.enemy_bullet, 
        elapsed_time
    );

    Ok(())
}

/// #### 한국어 </br>
/// 보스를 갱신하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a function that updates the boss. </br>
/// 
#[inline]
fn update_boss(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    boss::update_boss_face(elapsed_time, queue, &mut this.boss);
    boss::update_boss(this, shared, total_time, elapsed_time)?;

    Ok(())
}

/// #### 한국어 </br>
/// 모든 충돌을 처리합니다. </br>
/// 
/// #### English (Translation) </br>
/// Handles all collision. </br>
/// 
fn handles_collision(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();
    let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();

    // (한국어) 발사된 총알들을 가져옵니다.
    // (English Translation) Take the fired bullets.
    let mut enemy_bullets: Vec<_> = {
        let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
        instances.drain(..).collect()
    };

    // <1>
    if is_player_collide(&this.boss, &mut this.player, &mut enemy_bullets) {
        let remaining_life = decrease_player_life_count(
            &mut this.owned_hearts, 
            &mut this.lost_hearts
        );

        if remaining_life == 0 {
            audio.voice.stop();

            tile_brush.update(queue, |instances| {
                for &(r, c) in this.player.path.iter() {
                    instances[r * this.table.num_cols + c].color = this.table.tiles[r][c].color;
                }
            });
            while let Some((r, c)) = this.player.path.pop_front() {
                this.table.tiles[r][c].visited = false;
            }

            this.player.face_timer = 0.0;
            this.player.face_state = PlayerFaceState::Hit;
            this.player.sprite.update(queue, |instances| {
                instances[0].texture_index = PlayerFaceState::Hit as u32;
            });

            this.boss.face_timer = 0.0;
            this.boss.face_state = BossFaceState::Smile;
            this.boss.sprite.update(queue, |instances| {
                instances[0].texture_index = BossFaceState::Smile as u32;
            });

            this.timer = 0.0;
            this.state = InGameState::WaitForFinish;
        } else {
            // (한국어) 플레이어를 스폰위치로 이동시키고, 타일을 원래 상태로 되돌립니다.
            // (English Translation) Moves the player to the spawn position and returns the tile to its original state. 
            player::restore(
                queue, 
                &mut this.table, 
                &mut this.boss, 
                &mut this.player, 
                tile_brush
            );

            play_random_character_voice(
                &this.player_damage_sounds, 
                &audio.voice, 
                asset_bundle
            )?;
        }
    }


    // (한국어) 변경된 사항을 적용합니다.
    // (English Translation) Apply changes.
    {
        let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
        instances.append(&mut enemy_bullets);
    }
    this.enemy_bullet.update(queue, |_| { });

    Ok(())
}

/// #### 한국어 </br>
/// 플레이어가 적이나 적의 총알과 충돌한 경우 `true`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Returns `true` if the player collided with an enemy or an enemy bullet. </br>
/// 
fn is_player_collide(
    boss: &Boss, 
    player: &mut Player, 
    enemy_bullets: &mut Vec<BulletData>
) -> bool {
    let mut is_collide = false;
    if player.game_state != PlayerGameState::Invincibility {
        let player_collider = player.collider();

        // (한국어) 1. 플레이어와 보스와의 충돌을 확인합니다.
        // (English Translation) 1. Check the collision between the player and the boss. 
        let boss_collider = boss.collider();
        is_collide |= player_collider.test(&boss_collider);

        // (한국어) 2. 플레이어와 적의 총알과의 충돌을 확인합니다.
        // (English Translation) 2. Check for collisions between player and enemy bullets.
        let mut next_bullets = Vec::with_capacity(enemy_bullets.capacity());
        while let Some(bullet) = enemy_bullets.pop() {
            if player_collider.test(&bullet.collider()) {
                is_collide |= true;
                continue;
            }
            next_bullets.push(bullet);
        }
        enemy_bullets.append(&mut next_bullets);
    }
    return is_collide;
}

/// #### 한국어 </br>
/// 플레이어의 라이프 카운트를 감소시키고 남은 라이프 카운트를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Decrease the player's life count and return the remaining life count. </br>
/// 
fn decrease_player_life_count(
    owned_hearts: &mut VecDeque<UiObject>, 
    lost_hearts: &mut VecDeque<(f64, UiObject)>
) -> usize {
    // (한국어) 플레이어의 라이프 카운트를 감소시킵니다.
    // (English Translation) Decreases the player's life count.
    if let Some(heart) = owned_hearts.pop_back() {
        lost_hearts.push_back((0.0, heart));
    }

    return owned_hearts.len();
}

/// #### 한국어 </br>
/// 주어진 캐릭터 음성을 무작위로 선택하여 재생합니다. </br>
/// 이미 재생중인 음성이 있는 경우 생략합니다. </br>
/// 
/// #### English (Translation) </br>
/// Randomly selects and plays a given character's voice. </br>
/// If there is already audio playing, it will be omitted. </br>
/// 
fn play_random_character_voice(
    voices: &Vec<&'static str>, 
    voice_sink: &Sink, 
    asset_bundle: &AssetBundle
) -> AppResult<()> {
    if voice_sink.empty() {
        let rel_path = voices.choose(&mut rand::thread_rng()).unwrap();
        let source = asset_bundle.get(rel_path)?
            .read(&SoundDecoder)?;
        voice_sink.append(source);
    }

    Ok(())
}
