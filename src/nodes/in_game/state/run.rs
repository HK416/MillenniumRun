use std::sync::Arc;
use std::collections::VecDeque;

use winit::{
    event::{Event, WindowEvent},
    keyboard::PhysicalKey,
};

use crate::{
    game_err,
    components::{
        text2d::{brush::Text2dBrush, font::FontSet, section::Section2dBuilder},
        ui::UiBrush,
        camera::GameCamera,
        lights::PointLights,
        sprite::SpriteBrush,
        user::Settings,
        map::TileBrush,
        player::{self, ControlState}, anchor::Anchor, 
    },
    nodes::{path, in_game::{self, InGameScene}},
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};



pub fn handle_events(this: &mut InGameScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    handle_player_keyboard_events(this, shared, &event)?;
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    update_timer(this, shared, total_time, elapsed_time)?;
    player_update(this, shared, total_time, elapsed_time)?;
    update_owned_tiles(this, shared, total_time, elapsed_time)?;
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
    let point_lights = shared.get::<Arc<PointLights>>().unwrap();
    let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();

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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

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

        // (한국어) 배경을 화면에 그립니다.
        // (English Translation) Draws a background to the screen. 
        ui_brush.draw(&mut rpass, this.background.iter());
        text_brush.draw(&mut rpass, [&this.timer_ui].into_iter());
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

        // (한국어) 스프라이트를 화면에 그립니다.
        // (English Translation) Draws a sprite to the screen. 
        tile_brush.draw(point_lights, &mut rpass);
        sprite_brush.draw(point_lights, &mut rpass, [&this.player.sprite].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

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

                // (한국어) 사용자가 `위쪽`키를 눌렀을 경우.
                // (English Translation) When the user presses the `Up` key.
                if control.up.to_keycode() == code && event.state.is_pressed() && !event.repeat {
                    if !this.player.path.is_empty() && this.player.control_state == ControlState::Down {
                        return Ok(());
                    }
                    this.player.keyboard_pressed = true;
                    this.player.control_state = ControlState::Up;
                }

                // (한국어) 사용자가 `위쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Up` key.
                if control.up.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Up {
                    this.player.keyboard_pressed = false;

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
                    this.player.keyboard_pressed = true;
                    this.player.control_state = ControlState::Down;
                }

                // (한국어) 사용자가 `아래쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Down` key.
                if control.down.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Down {
                    this.player.keyboard_pressed = false;

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
                    this.player.keyboard_pressed = true;
                    this.player.control_state = ControlState::Left;
                }

                // (한국어) 사용자가 `왼쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Left` key.
                if control.left.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Left {
                    this.player.keyboard_pressed = false; 

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
                    this.player.keyboard_pressed = true;
                    this.player.control_state = ControlState::Right;
                }

                // (한국어) 사용자가 `오른쪽`키를 떼었을 경우.
                // (English Translation) When the user releases the `Right` key.
                if control.right.to_keycode() == code && !event.state.is_pressed() && !event.repeat 
                && this.player.control_state == ControlState::Right {
                    this.player.keyboard_pressed = false; 

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




fn update_timer(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get the shared object to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let font_set = shared.get::<FontSet>().unwrap();
    let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap();
    let nexon_lv2_gothic_medium = font_set.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
        .expect("Registered font not found!");

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.remaining_time = (this.remaining_time - elapsed_time).max(0.0);
    let min = ((this.remaining_time / 60.0) as u32).min(9);
    let sec0 = ((this.remaining_time % 60.0) / 10.0) as u32; 
    let sec1 = (this.remaining_time % 10.0) as u32;

    this.timer_ui = Section2dBuilder::new(
        Some("Timer"), 
        &nexon_lv2_gothic_medium, 
        &format!("{}:{}{}", min, sec0, sec1), 
        text_brush
    )
    .with_anchor(Anchor::new(0.75, 0.85, 0.55, 0.85))
    .build(device, queue);

    Ok(())
}


/// #### 한국어 </br>
/// 소유한 타일을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates owned tiles. </br>
/// 
fn update_owned_tiles(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 0.3;

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
        let delta = smooth_step(timer, DURATION);
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
        this.table.origin.x, 
        this.table.origin.y, 
        this.table.size.x, 
        this.table.size.y, 
        &mut this.player, 
        &queue
    );

    player::check_current_pos(
        this.table.origin.x, 
        this.table.origin.y, 
        this.table.size.x, 
        this.table.size.y, 
        in_game::NUM_TILE_ROWS, 
        in_game::NUM_TILE_COLS, 
        in_game::LINE_COLOR, 
        in_game::EDGE_COLOR, 
        in_game::FILL_COLOR, 
        tile_brush, 
        &mut this.table.tiles, 
        &mut this.player, 
        &mut this.num_owned_tiles, 
        &mut this.owned_tiles, 
        queue
    );

    player::set_player_next_position(
        this.table.num_rows, 
        this.table.num_cols, 
        &mut this.player
    );

    Ok(())
}


#[inline]
fn smooth_step(elapsed_time: f64, duration: f64) -> f32 {
    let t = (elapsed_time / duration).clamp(0.0, 1.0) as f32;
    return 3.0 * t * t - 2.0 * t * t * t;
}
