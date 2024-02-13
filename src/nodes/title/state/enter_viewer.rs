use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err, 
    components::{
        ui::UiBrush, 
        text::TextBrush, 
        sprite::SpriteBrush, 
        camera::GameCamera, 
        player::Actor, 
        interpolation, 
    }, 
    render::depth::DepthBuffer, 
    nodes::title::{
        TitleScene, 
        state::TitleState,
    },
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared, 
    },
};

const DURATION: f64 = 0.5;



pub fn handle_events(_this: &mut TitleScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut TitleScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.timer += elapsed_time;

    // (한국어) 인터페이스의 투명도를 갱신합니다. 
    // (English Translation) Updates the transparency of the interface.
    let actor = shared.get::<Actor>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let delta = interpolation::f64::smooth_step(this.timer, DURATION) as f32;
    let transparency = 1.0 - 1.0 * delta;
    for (sprite, _) in this.sprites.iter() {
        sprite.update(queue, |instances| instances[0].color.w = transparency);
    }
    this.sprites[*actor as usize].0.update(queue, |instances| instances[0].color.w = transparency);
    this.stage_window.update(queue, |data| data.color.w = transparency);
    this.stage_enter_button.0.update(queue, |data| data.color.w = transparency);
    this.stage_enter_button.1.update(queue, |data| data.color.w = transparency);
    this.stage_images[actor].0.update(queue, |data| data.color.w = transparency);
    this.stage_images[actor].1.update(queue, |data| data.color.w = transparency);
    this.stage_images[actor].2.update(queue, |data| data.color.w = transparency);


    let transparency = 1.0 * delta;
    this.stage_viewer_images[actor].update(queue, |data| data.color.w = transparency);


    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) changes to the next state if it is greater than the duration.
    if this.timer >= DURATION {
        this.timer = 0.0;
        this.state = TitleState::Viewer;
    }

    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use. 
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let actor = shared.get::<Actor>().unwrap();


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
            label: Some("RenderPass(TitleScene(EnterPreview(Background)))"),
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

        let iter = [&this.background].into_iter()
            .chain(this.sprites.iter().map(|(it, _)| it));
        sprite_brush.draw(&mut rpass, iter);
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(EnterPreview(Ui)))"),
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

        // (한국어) 스테이지 윈도우 창을 그립니다. 
        // (English Translation) Drawing the stage window.
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.stage_window, 
            &this.stage_enter_button.0, 
            &this.stage_images[&actor].0, 
            &this.stage_images[&actor].1, 
        ].into_iter());
        text_brush.draw(&mut rpass, [
            &this.stage_enter_button.1, 
            &this.stage_images[&actor].2,
        ].into_iter());
        ui_brush.draw(&mut rpass, [&this.stage_viewer_images[&actor]].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
