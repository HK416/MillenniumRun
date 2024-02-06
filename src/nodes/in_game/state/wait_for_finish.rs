use std::sync::Arc;
use std::collections::VecDeque;

use rodio::Source;
use winit::event::Event;

use crate::{
    game_err, 
    assets::bundle::AssetBundle, 
    components::{
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        camera::GameCamera, 
        bullet::{self, BulletBrush}, 
        player::{self, Actor}, 
        table::TileBrush, 
        user::Settings, 
        interpolation, 
        sound, 
        save::{SaveData, SaveEncoder}, 
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

const DURATION: f64 = 1.0;


pub fn handle_events(_this: &mut InGameScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    use crate::nodes::path;

    {
        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared objects to use.
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
        let settings = shared.get::<Settings>().unwrap();
        
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Updates the timer.
        this.timer += elapsed_time;
        let scale = 1.0 - 1.0 * interpolation::f64::smooth_step(this.timer, DURATION) as f32;
        
        // (한국어) 배경 음악 소리를 줄입니다.
        // (English Translation) Reduce the background music sound.
        audio.background.set_volume(settings.background_volume.norm() * scale);
        
        // (한국어) 적이 발사한 총알들을 갱신합니다.
        // (English Translation) Updates the bullets fired by the enemy.
        this.enemy_bullet.update(queue, |instances| {
            for instance in instances.iter_mut() {
                instance.color.w = scale;
            }
        });
        bullet::update_bullets(
            queue, 
            &this.table, 
            &this.enemy_bullet, 
            elapsed_time
        );
        
        update_owned_tiles(this, shared, total_time, elapsed_time)?;
        update_lost_hearts(this, shared, total_time, elapsed_time)?;
        
        update_percent_text(this, shared, total_time, elapsed_time)?;
    }

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to the next state. 
    if this.timer >= DURATION {
        // (한국어) 세이브 파일에 결과를 저장합니다.
        // (English Translation) Save the results in a save file.
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();
        let save = shared.get_mut::<SaveData>().unwrap();
        let updated = match this.player.actor {
            Actor::Aris => { 
                if save.stage_aris < this.num_owned_tiles as u16 {
                    save.stage_aris = this.num_owned_tiles as u16;
                    true
                } else {
                    false
                }
            }, 
            Actor::Momoi => {
                if save.stage_momoi < this.num_owned_tiles as u16 {
                    save.stage_momoi = this.num_owned_tiles as u16;
                    true
                } else {
                    false
                }
            }, 
            Actor::Midori => { 
                if save.stage_midori < this.num_owned_tiles as u16 {
                    save.stage_midori = this.num_owned_tiles as u16;
                    true
                } else {
                    false
                }
            }, 
            Actor::Yuzu => { 
                if save.stage_yuzu < this.num_owned_tiles as u16 {
                    save.stage_yuzu = this.num_owned_tiles as u16;
                    true
                } else {
                    false
                }
            }
        };
        if updated {
            asset_bundle.get(path::SAVE_PATH)?
                .write(&SaveEncoder, save)?;
        }

        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared object to use.
        let audio = shared.get::<Arc<utils::InGameAudio>>().unwrap();
        let settings = shared.get::<Settings>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();

        audio.background.stop();
        audio.background.set_volume(settings.background_volume.norm());
        audio.voice.stop();

        if this.owned_hearts.len() == 0 {
            let source = asset_bundle.get(path::THEME27_SOUND_PATH)?
                .read(&sound::SoundDecoder)?
                .amplify(0.5)
                .repeat_infinite();
            audio.background.append(source);
        } else {
            let source = asset_bundle.get(path::THEME23_SOUND_PATH)?
                .read(&sound::SoundDecoder)?
                .amplify(0.5)
                .repeat_infinite();
            audio.background.append(source);
        }

        this.timer = 0.0;
        this.state = InGameState::DisappearRun;
    }
    
    Ok(())
}

pub fn draw(this: &InGameScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
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
                label: Some("RenderPass(InGameScene(EnterFinish(Background)))"),
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
                label: Some("RenderPass(InGameScene(EnterFinish(Ui)))"),
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
                label: Some("RenderPass(InGameScene(EnterFinish(Sprite)))"),
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

fn update_percent_text(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.percent_timer += elapsed_time;

    let per = this.num_owned_tiles as f32 / this.num_total_tiles as f32 * 100.0;
    let s = 1.0 + 0.5 - 0.5 * interpolation::f64::smooth_step(this.percent_timer, 0.25) as f32;
    this.percent.change(
        &format!("{}%", per.floor() as u32), 
        device, 
        queue, 
        &text_brush
    );
    this.percent.update(queue, |data| data.scale = (s, s, s).into());

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
