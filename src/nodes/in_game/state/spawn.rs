use std::sync::Arc;
use std::collections::VecDeque;

use rodio::Sink;
use winit::event::Event;

use crate::{
    game_err, 
    assets::bundle::AssetBundle, 
    components::{
        ui::UiBrush,  
        text::TextBrush, 
        sprite::SpriteBrush, 
        table::TileBrush, 
        camera::GameCamera, 
        interpolation, 
        sound, 
    }, 
    nodes::in_game::{
        InGameScene, 
        state::InGameState, 
    }, 
    render::depth::DepthBuffer, 
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared, 
    } 
};

const DURATION: f64 = 0.3;


pub fn handle_events(_this: &mut InGameScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.timer += elapsed_time;

    // (한국어) 사용할 공유 객체를 가져옵니다.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();

    // (한국어) 플레이어의 크기를 갱신합니다.
    // (English Translation) Updates the player's scale.
    let s = scale_interpolation(this.timer, DURATION) as f32;
    this.player.sprite.update(queue, |instances| {
        instances[0].scale = (s, s, s).into();
    });
    for face in this.player_faces.values() {
        face.update(queue, |data| {
            data.local_scale = (s, s, s).into();
        });
    }

    // (한국어) 플레이어 체력 인터페이스의 크기를 갱신합니다. 
    // (English Translation) Update the scale of the player health interface. 
    let s = interpolation::f64::smooth_step(this.timer, DURATION) as f32;
    for ui in this.owned_hearts.iter() {
        ui.update(queue, |data| {
            data.local_scale = (s, s, s).into();
        });
    }

    // (한국어) 보스의 크기를 갱신합니다.
    // (English Translation) Updates the boss's scale.
    let s = scale_interpolation(this.timer, DURATION) as f32;
    this.boss.sprite.update(queue, |instances| {
        instances[0].scale = (s, s, s).into();
    });
    for face in this.boss_faces.values() {
        face.update(queue, |data| {
            data.local_scale = (s, s, s).into();
        });
    }
    
    // (한국어) 타일을 갱신합니다.
    // (English Translation) Updates the tiles. 
    let mut next = VecDeque::new();
    while let Some((mut timer, tiles)) = this.owned_tiles.pop_front() {
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Updates the timer. 
        timer += elapsed_time;

        // (한국어) 타일의 알파 값을 갱신합니다.
        // (English Translation) Updates the alpha value of the tile.
        let alpha = 1.0 - 1.0 * interpolation::f64::smooth_step(timer, DURATION) as f32;
        for &(row, col) in tiles.iter() {
            this.table.tiles[row][col].color.w = alpha;
        }

        // (한국어) 타일의 변경된 내용을 적용합니다.
        // (English Translation) Apply changes to the tile. 
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


    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to the next state.
    if this.timer >= DURATION {
        this.timer = 0.0;
        this.state = InGameState::Ready;


        // (한국어) 플레이어 시작 목소리를 재생합니다.
        // (English Translation) Play the player startup voice.
        if let Some((_, voice)) = shared.get::<(Sink, Sink)>() {
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(this.player_startup_sound)?.read(&sound::SoundDecoder)?;
            voice.append(source);
        }
    }

    Ok(())
}

pub fn draw(this: &InGameScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use. 
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let tile_brush = shared.get::<Arc<TileBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
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

    // (한국어) 프레임 버퍼의 텍스처 뷰를 생성합니다.
    // (English Translation) Creates a texture view of the framebuffer.
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Spawn(Background)))"),
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
                    }
                ), 
                timestamp_writes: None,
                occlusion_query_set: None, 
            }
        );

        // (한국어) 카메라를 바인드 합니다.
        // (English Translation) Bind the camera. 
        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [
            &this.background, 
            &this.stage_images[this.result_star_index.min(3)], 
        ].into_iter());
        tile_brush.draw(&mut rpass);
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Spawn(Ui)))"),
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
        ui_brush.draw(
            &mut rpass, 
            [
                &this.menu_button, 
                &this.remaining_timer_bg, 
                &this.player_faces[&this.player.face_state], 
                &this.boss_faces[&this.boss.face_state], 
            ].into_iter()
        );
        ui_brush.draw(&mut rpass, this.owned_hearts.iter());
        ui_brush.draw(&mut rpass, this.lost_hearts.iter().map(|(_, it)| it));
        text_brush.draw(&mut rpass, [&this.remaining_timer_text, &this.percent].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(Spawn(Sprite)))"),
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
        sprite_brush.draw(&mut rpass, [&this.player.sprite, &this.boss.sprite].into_iter());
    }


    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}

#[inline]
fn scale_interpolation(val: f64, max: f64) -> f64 {
    debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
    let t = (val / max).clamp(0.0, 1.0);
    return t.powf(1.0 - 3.0 * t * t)
}
