use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err,
    components::{
        ui::{UiBrush, UiObject}, 
        text::{TextBrush, Text},  
        camera::GameCamera, 
        transform::Projection, 
        sprite::SpriteBrush,  
    },
    nodes::title::{
        utils,
        TitleScene, 
        state::TitleState, 
    }, 
    render::depth::DepthBuffer, 
    system::{
        error::{AppResult, GameError},
        event::AppEvent, 
        shared::Shared, 
    }, 
};



/// #### 한국어 </br>
/// `ExitSelected`상태의 지속 시간입니다. </br>
/// 
/// #### English (Translation) </br>
/// Duration of the `ExitSelected` state. </br>
/// 
const DURATION: f64 = 0.5;


pub fn handle_events(_this: &mut TitleScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut TitleScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let sprite = shared.get::<utils::Sprites>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 경과한 시간을 갱신합니다.
    // (English Translation) Updates the elapsed time.
    this.elapsed_time += elapsed_time;

    // (한국어) 카메라의 투영 행렬을 갱신합니다.
    // (English Translation) Update the camera's projection matrix.
    let delta = smooth_step(this.elapsed_time, DURATION);
    camera.update(queue, |data| {
        data.projection = Projection::new_ortho(
            match sprite {
                utils::Sprites::Aris => utils::STAGE_ARIS_TOP + (utils::STAGE_TOP - utils::STAGE_ARIS_TOP) * delta,
                utils::Sprites::Momoi => utils::STAGE_MOMOI_TOP + (utils::STAGE_TOP - utils::STAGE_MOMOI_TOP) * delta,
                utils::Sprites::Midori => utils::STAGE_MIDORI_TOP + (utils::STAGE_TOP - utils::STAGE_MIDORI_TOP) * delta,
                utils::Sprites::Yuzu => utils::STAGE_YUZU_TOP + (utils::STAGE_TOP - utils::STAGE_YUZU_TOP) * delta,
            }, 
            match sprite {
                utils::Sprites::Aris => utils::STAGE_ARIS_LEFT + (utils::STAGE_LEFT - utils::STAGE_ARIS_LEFT) * delta,
                utils::Sprites::Momoi => utils::STAGE_MOMOI_LEFT + (utils::STAGE_LEFT - utils::STAGE_MOMOI_LEFT) * delta,
                utils::Sprites::Midori => utils::STAGE_MIDORI_LEFT + (utils::STAGE_LEFT - utils::STAGE_MIDORI_LEFT) * delta,
                utils::Sprites::Yuzu => utils::STAGE_YUZU_LEFT + (utils::STAGE_LEFT - utils::STAGE_YUZU_LEFT) * delta,
            }, 
            match sprite {
                utils::Sprites::Aris => utils::STAGE_ARIS_BOTTOM + (utils::STAGE_BOTTOM - utils::STAGE_ARIS_BOTTOM) * delta,
                utils::Sprites::Momoi => utils::STAGE_MOMOI_BOTTOM + (utils::STAGE_BOTTOM - utils::STAGE_MOMOI_BOTTOM) * delta,
                utils::Sprites::Midori => utils::STAGE_MIDORI_BOTTOM + (utils::STAGE_BOTTOM - utils::STAGE_MIDORI_BOTTOM) * delta,
                utils::Sprites::Yuzu => utils::STAGE_YUZU_BOTTOM + (utils::STAGE_BOTTOM - utils::STAGE_YUZU_BOTTOM) * delta,
            }, 
            match sprite {
                utils::Sprites::Aris => utils::STAGE_ARIS_RIGHT + (utils::STAGE_RIGHT - utils::STAGE_ARIS_RIGHT) * delta,
                utils::Sprites::Momoi => utils::STAGE_MOMOI_RIGHT + (utils::STAGE_RIGHT - utils::STAGE_MOMOI_RIGHT) * delta,
                utils::Sprites::Midori => utils::STAGE_MIDORI_RIGHT + (utils::STAGE_RIGHT - utils::STAGE_MIDORI_RIGHT) * delta,
                utils::Sprites::Yuzu => utils::STAGE_YUZU_RIGHT + (utils::STAGE_RIGHT - utils::STAGE_YUZU_RIGHT) * delta,
            }, 
            0.0,
            1000.0
        );
    });

    // (한국어) 스테이지 윈도우 알파 값을 갱신합니다.
    // (English Translation) Updates the stage window alpha value.
    let alpha = 1.0 - 1.0 * delta;
    update_ui_alpha(this.stage_window.iter_mut(), &queue, alpha);

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) changes to the next state if it is greater than the duration.
    if this.elapsed_time >= DURATION {
        shared.pop::<utils::Sprites>().unwrap();
        this.state = TitleState::Stage;
        this.elapsed_time = 0.0;
        return Ok(());
    }

    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
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
            label: Some("RenderPass(TitleScene(ExitSelected(Background)))"),
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
            label: Some("RenderPass(TitleScene(ExitSelected(Sprites)))"),
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
        // (English Translation) Drawing the sprite objects.
        sprite_brush.draw(&mut rpass, this.sprites.iter().map(|(it, _)| it));
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(ExitSelected(Ui)))"),
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

        // (한국어) 버튼 그리기.
        // (English Translation) Drawing the buttons.
        ui_brush.draw(
            &mut rpass, 
            this.system_buttons.iter()
            .map(|(it, _)| it)
        );
        text_brush.draw(
            &mut rpass, 
            this.system_buttons.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it)
        );
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(EnterSelected(Ui)))"),
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
        ui_brush.draw(
            &mut rpass, 
            this.stage_window.iter()
            .map(|(it, _)| it)
        );
        text_brush.draw(
            &mut rpass, 
            this.stage_window.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it)
        );
    }
    
    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}


#[inline]
fn smooth_step(elapsed_time: f64, duration: f64) -> f32 {
    let t = (elapsed_time / duration).clamp(0.0, 1.0) as f32;
    return 3.0 * t * t - 2.0 * t * t * t;
}


/// #### 한국어 </br>
/// 사용자 인터페이스 객체의 알파 값을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the alpha value of the user interface object. </br>
/// 
fn update_ui_alpha<'a, Iter>(iter: Iter, queue: &wgpu::Queue, alpha: f32) 
where Iter: Iterator<Item = &'a mut (UiObject, Vec<Text>)> {
    for (ui, sections) in iter {
        ui.update(queue, |data| {
            data.color.w = alpha;
        });
        for section in sections.iter_mut() {
            section.update(queue, |data| {
                data.color.w = alpha;
            })
        }
    }
}
