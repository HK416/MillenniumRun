use std::thread;
use std::sync::Arc;
use std::collections::VecDeque;

use rand::prelude::*;
use rodio::OutputStreamHandle;
use winit::{
    event::{Event, WindowEvent, MouseButton},
    keyboard::{PhysicalKey, KeyCode},
    dpi::PhysicalPosition, 
};

use crate::{
    game_err,
    assets::bundle::AssetBundle, 
    components::{
        text::TextBrush,
        ui::UiBrush,
        camera::GameCamera,
        sprite::SpriteBrush,
        user::Settings,
        table::TileBrush,
        bullet::BulletBrush, 
        player::{self, Actor, ControlState}, 
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



pub fn handle_events(this: &mut InGameScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_player_mouse_events(this, shared, &event)?;
    handle_player_keyboard_events(this, shared, &event)?;
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    update_percent_text(this, shared, total_time, elapsed_time)?;
    update_timer(this, shared, total_time, elapsed_time)?;
    update_lost_hearts(this, shared, total_time, elapsed_time)?;
    player_update(this, shared, total_time, elapsed_time)?;
    update_owned_tiles(this, shared, total_time, elapsed_time)?;

    update_bullets(this, shared, total_time, elapsed_time)?;

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
                &this.stage_image, 
                &this.player_faces[&this.player.face_state], 
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
        sprite_brush.draw(&mut rpass, [&this.player.sprite].into_iter());
        bullet_brush.draw(&mut rpass, [&this.player_bullet].into_iter());
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
fn handle_player_mouse_events(this: &mut InGameScene, _shared: &mut Shared, event: &Event<AppEvent>) -> AppResult<()> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::MouseInput { state, button, .. } => 
            if MouseButton::Left == *button && state.is_pressed() {
                this.mouse_pressed = true;
            } else if MouseButton::Left == *button && !state.is_pressed() {
                this.mouse_pressed = false;
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
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let settings = shared.get::<Settings>().unwrap();
    let control = &settings.control;
    
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                // (한국어) 사용자가 `ESC`키를 눌렀을 경우.
                // (English Translation) When the user presses the `ESC` key.
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    sound::play_click_sound(shared)?;

                    // TODO!

                    // (한국어) 다음 게임 장면 상태로 변경합니다.
                    // (English Translation) Change to the next game scene state. 
                    this.timer = 0.0;
                    this.state = InGameState::EnterPause; 
                }

                // (한국어) 사용자가 `위쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Up` key.
                if control.up.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == ControlState::Down {
                        return Ok(());
                    }
                    this.keyboard_pressed = true;
                    this.player.control_state = ControlState::Up;
                }

                // (한국어) 사용자가 `위쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Up` key.
                if control.up.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Up {
                    this.keyboard_pressed = false;

                    if this.player.path.is_empty() {
                        this.player.control_state = ControlState::Idle;
                    }
                }


                // (한국어) 사용자가 `아래쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Down` key.
                if control.down.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == ControlState::Up {
                        return Ok(());
                    }
                    this.keyboard_pressed = true;
                    this.player.control_state = ControlState::Down;
                }

                // (한국어) 사용자가 `아래쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Down` key.
                if control.down.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Down {
                    this.keyboard_pressed = false;

                    if this.player.path.is_empty() {
                        this.player.control_state = ControlState::Idle;
                    }
                }


                // (한국어) 사용자가 `왼쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Left` key.
                if control.left.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == ControlState::Right {
                        return Ok(());
                    }
                    this.keyboard_pressed = true;
                    this.player.control_state = ControlState::Left;
                }

                // (한국어) 사용자가 `왼쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Left` key.
                if control.left.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Left {
                    this.keyboard_pressed = false; 

                    if this.player.path.is_empty() { 
                        this.player.control_state = ControlState::Idle;
                    }
                }


                // (한국어) 사용자가 `오른쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Right` key.
                if control.right.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == ControlState::Left {
                        return Ok(());
                    }
                    this.keyboard_pressed = true;
                    this.player.control_state = ControlState::Right;
                }

                // (한국어) 사용자가 `오른쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Right` key.
                if control.right.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Right {
                    this.keyboard_pressed = false; 

                    if this.player.path.is_empty() {
                        this.player.control_state = ControlState::Idle;
                    }
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
fn update_timer(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();

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

    player::update_player_face(elapsed_time, queue, &mut this.player);

    player::translation_player(
        elapsed_time, 
        &this.table, 
        &mut this.player, 
        &queue
    );

    if let Some(flag) = player::check_current_pos(
        &mut this.table, 
        &mut this.player, 
        this.keyboard_pressed, 
        &mut this.num_owned_tiles,
        &mut this.owned_tiles, 
        &mut this.owned_hearts, 
        &mut this.lost_hearts, 
        tile_brush, 
        queue
    ) {
        if flag {

            // (한국어) 퍼센트 인터페이스를 갱신합니다.
            // (English Translation) Updates the percent interface. 
            this.percent_timer = 0.0;
            
            // (한국어) 무작위로 캐릭터 목소리를 재생합니다.
            // (English Translation) Plays character voices randomly. 
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.3) && audio.voice.empty() {
                let rel_path = this.player_smile_sounds.choose(&mut rng).unwrap();
                let source = asset_bundle.get(rel_path)?
                .read(&SoundDecoder)?;
                audio.voice.append(source)
            }
        } else {
            // (한국어) 무작위로 캐릭터 목소리를 재생합니다.
            // (English Translation) Plays character voices randomly. 
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
            let mut rng = rand::thread_rng();
            if audio.voice.empty() {
                let rel_path = this.player_damage_sounds.choose(&mut rng).unwrap();
                let source = asset_bundle.get(rel_path)?
                .read(&SoundDecoder)?;
                audio.voice.append(source)
            }
        }
    };

    player::set_player_next_position(
        &this.table, 
        &mut this.player
    );

    Ok(())
}

fn update_percent_text(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.percent_timer += elapsed_time;

    let per = this.num_owned_tiles as f32 /  this.num_total_tiles as f32 * 100.0;
    let s = 1.0 + 0.5 - 0.5 * interpolation::f64::smooth_step(this.percent_timer, 0.25) as f32;
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
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
    Ok(())
}


fn update_bullets(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use. 
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();

    let cursor_pos_world = camera.to_world_coordinates(cursor_pos).into();
    
    if this.mouse_pressed && this.player.try_fire() {
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let settings = shared.get::<Settings>().unwrap();
        let stream = shared.get::<OutputStreamHandle>().unwrap();
        let source = asset_bundle.get(this.player_fire_sound)?
            .read(&sound::SoundDecoder)?;
        let sink = sound::play_sound(settings.effect_volume, source, stream)?;
        thread::spawn(move || {
            sink.sleep_until_end();
            sink.detach();
        });
    }

    player::update_player_bullet(
        queue, 
        &this.table, 
        &this.player_bullet, 
        &mut this.player, 
        elapsed_time, 
        cursor_pos_world
    );

    Ok(())
}
