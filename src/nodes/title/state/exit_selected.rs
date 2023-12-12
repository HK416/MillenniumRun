use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err,
    components::{
        sprite::brush::SpriteBrush,  
        ui::{brush::UiBrush, objects::UiObject}, 
        text::{brush::TextBrush, section::d2::Section2d}, 
        transform::{Orthographic, Projection}, 
        camera::GameCamera, 
    },
    nodes::title::{
        DEF_PROJECTION,
        FOCUSED_PROJECTIONS,
        ty, state::TitleState, 
        TitleScene, 
    }, 
    system::{
        error::{AppResult, GameError},
        event::AppEvent, 
        shared::Shared, 
    }, render::depth::DepthBuffer, 
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
    let tag = shared.pop::<ty::SpriteButtonTags>().unwrap();
    let queue = shared.pop::<Arc<wgpu::Queue>>().unwrap();
    let mut camera = shared.pop::<GameCamera>().unwrap();

    // (한국어) 경과한 시간을 갱신합니다.
    // (English Translation) Updates the elapsed time.
    this.elapsed_time += elapsed_time;

    // (한국어) 카메라의 투영 행렬을 갱신합니다.
    // (English Translation) Update the camera's projection matrix.
    let delta = smooth_step(this.elapsed_time, DURATION);
    let ortho = Orthographic {
        top: FOCUSED_PROJECTIONS[tag as usize].top + (DEF_PROJECTION.top - FOCUSED_PROJECTIONS[tag as usize].top) * delta,
        left: FOCUSED_PROJECTIONS[tag as usize].left + (DEF_PROJECTION.left - FOCUSED_PROJECTIONS[tag as usize].left) * delta,
        bottom: FOCUSED_PROJECTIONS[tag as usize].bottom + (DEF_PROJECTION.bottom - FOCUSED_PROJECTIONS[tag as usize].bottom) * delta,
        right: FOCUSED_PROJECTIONS[tag as usize].right + (DEF_PROJECTION.right - FOCUSED_PROJECTIONS[tag as usize].right) * delta,
        near: FOCUSED_PROJECTIONS[tag as usize].near + (DEF_PROJECTION.near - FOCUSED_PROJECTIONS[tag as usize].near) * delta,
        far: FOCUSED_PROJECTIONS[tag as usize].far + (DEF_PROJECTION.far - FOCUSED_PROJECTIONS[tag as usize].far) * delta,
    };

    camera.projection = Projection::Orthographic(ortho);
    camera.update_buffer(&queue);

    // (한국어) 스테이지 윈도우 알파 값을 갱신합니다.
    // (English Translation) Updates the stage window alpha value.
    let alpha = 1.0 - 1.0 * delta;
    update_ui_alpha(this.stage_window.iter_mut(), &queue, alpha);

    // (한국어) 사용 완료한 공유 객체를 반환합니다.
    // (English Translation) Returns a shared object that has been used.
    shared.push(camera);
    shared.push(queue);
    shared.push(tag);

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) changes to the next state if it is greater than the duration.
    if this.elapsed_time >= DURATION {
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
    let camera = shared.get::<GameCamera>().unwrap();

    
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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

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
        this.background.draw(sprite_brush, &mut rpass);

        // (한국어) 스프라이트 오브젝트들 그리기.
        // (English Translation) Drawing sprite objects.
        this.sprite.draw(sprite_brush, &mut rpass);
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
        this.system.draw(ui_brush, text_brush, &mut rpass);
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
        this.stage_window.draw(ui_brush, text_brush, &mut rpass);
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
where Iter: Iterator<Item = &'a mut (Arc<UiObject>, Vec<Arc<Section2d>>)> {
    for (ui, texts) in iter {
        ui.update_buffer(queue, |data| {
            data.color.w = alpha;
        });
        for text in texts.iter_mut() {
            text.update_section(queue, |data| {
                data.color.w = alpha;
            })
        }
    }
}
