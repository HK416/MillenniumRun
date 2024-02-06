use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err, 
    components::{
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        table::TileBrush, 
        bullet::BulletBrush, 
        camera::GameCamera, 
        interpolation, 
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
    },
};

const DURATION: f64 = 0.2;



pub fn handle_events(_this: &mut InGameScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    this.timer += elapsed_time;

    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use. 
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 인터페이스를 갱신합니다.
    // (English Translation) Updates the interfaces.
    let delta = interpolation::f64::smooth_step(this.timer, DURATION) as f32;
    
    let alpha = 1.0 - 1.0 * delta;
    this.pause_text.update(queue, |data| data.color.w = alpha);
    for (ui, text) in this.pause_buttons.values() {
        ui.update(queue, |data| data.color.w = alpha);
        text.update(queue, |data| data.color.w = alpha);
    }

    let scale = 1.0 * delta;
    let iter = [
            &this.setting_return_button.0, 
        ].into_iter()
        .chain(this.setting_windows.iter())
        .chain(this.setting_languages.values().map(|(it, _)| it))
        .chain(this.setting_resolutions.values().map(|(it, _)| it))
        .chain(this.setting_volume_background.values().map(|(it, _)| it))
        .chain(this.setting_volume_bar.values());
    for ui in iter {
        ui.update(queue, |data| {
            data.global_scale = (scale, scale, scale).into() 
        });
    }

    let iter = [
            &this.setting_return_button.1, 
        ].into_iter()
        .chain(this.setting_titles.iter())
        .chain(this.setting_languages.values().map(|(_, it)| it))
        .chain(this.setting_resolutions.values().map(|(_, it)| it))
        .chain(this.setting_volume_background.values().map(|(_, it)| it));
    for text in iter {
        text.update(queue, |data| {
            data.scale = (scale, scale, scale).into()
        });
    }

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state if it is greater than the duration. 
    if this.timer >= DURATION {
        this.state = InGameState::Setting;
        this.timer = 0.0;
    }
    
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
                label: Some("RenderPass(InGameScene(EnterSetting(Background)))"), 
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
                label: Some("RenderPass(InGameScene(EnterSetting(Sprite)))"),
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
                label: Some("RenderPass(InGameScene(EnterSetting(Foreground)))"),
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
                label: Some("RenderPass(InGameScene(EnterSetting(PauseUI)))"),
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
        
        ui_brush.draw(&mut rpass, this.pause_buttons.values().map(|(it, _)| it));
        
        let iter = [&this.pause_text].into_iter()
            .chain(this.pause_buttons.values().map(|(_, it)| it));
        text_brush.draw(&mut rpass, iter);
    }

    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(InGameScene(EnterSetting(SettingUI)))"),
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
