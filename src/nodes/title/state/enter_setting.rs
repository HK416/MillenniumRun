use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err,
    components::{
        text2d::{brush::Text2dBrush, section::Section2d},
        ui::{brush::UiBrush, objects::UiObject},
        camera::GameCamera,
        lights::PointLights, 
        sprite::SpriteBrush,
    },
    nodes::title::{
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
/// `EnterSetting` 상태의 지속 시간입니다. </br>
/// 
/// #### English (Translation) </br>
/// Duration of the `EnterSetting` state. </br>
/// 
const DURATION: f64 = 0.2;


pub fn handle_events(_this: &mut TitleScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut TitleScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 경과 시간을 갱신합니다.
    // (English Translation) Updates the elapsed time.
    this.elapsed_time += elapsed_time;


    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 배율을 시간에 따라 갱신합니다.
    // (English Translation) Updates the scale over time.
    let delta = smooth_step(this.elapsed_time, DURATION);
    let alpha = 1.0 - 1.0 * delta;
    let scale = 1.0 * delta;
    update_ui_alpha(this.menu_buttons.iter_mut(), queue, alpha);
    update_ui_scale(this.setting_window.iter_mut(), queue, scale);

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state if it is greater than the duration. 
    if this.elapsed_time >= DURATION {
        this.state = TitleState::Setting;
        this.elapsed_time = 0.0;
        return Ok(());
    }

    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let point_lights = shared.get::<Arc<PointLights>>().unwrap();
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
    let text_brush = shared.get::<Arc<Text2dBrush>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(EnterSettingState(Background)))"),
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
        
        // (한국어) 배경 오브젝트 그리기.
        // (English Translation) Drawing background objects.
        sprite_brush.draw(point_lights, &mut rpass, [this.background.as_ref()].into_iter());
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(SettingState(Ui)))"),
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
        
        // (한국어) 사용자 인터페이스 그리기.
        // (English Translation) Drawing the user interface.
        ui_brush.draw(
            &mut rpass, 
            this.menu_buttons.iter()
            .map(|(ui, _)| ui.as_ref())
        );
        text_brush.draw(
            &mut rpass, 
            this.menu_buttons.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it.as_ref())
        );
    }

    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(TitleScene(SettingState(Ui)))"),
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
        
        // (한국어) 사용자 인터페이스 그리기.
        // (English Translation) Drawing the user interface.
        ui_brush.draw(
            &mut rpass, 
            this.setting_window.iter()
            .map(|(ui, _)| ui.as_ref())
        );
        text_brush.draw(
            &mut rpass, 
            this.setting_window.iter()
            .map(|(_, it)| it)
            .flatten()
            .map(|it| it.as_ref())
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
/// 사용자 인터페이스 객체의 크기 값을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the scale value of the user interface object.
/// 
fn update_ui_scale<'a, Iter>(iter: Iter, queue: &wgpu::Queue, scale: f32) 
where Iter: Iterator<Item = &'a mut (Arc<UiObject>, Vec<Arc<Section2d>>)> {
    for (ui, sections) in iter {
        ui.update(queue, |data| {
            data.transform.x_axis.x = scale;
            data.transform.y_axis.y = scale;
            data.transform.z_axis.z = scale;
        });
        for section in sections.iter_mut() {
            section.update(queue, |data| {
                data.transform.x_axis.x = scale;
                data.transform.y_axis.y = scale;
                data.transform.z_axis.z = scale;
            });
        }
    }
}

/// #### 한국어 </br>
/// 사용자 인터페이스 객체의 알파 값을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the alpha value of the user interface object.
/// 
fn update_ui_alpha<'a, Iter>(iter: Iter, queue: &wgpu::Queue, alpha: f32) 
where Iter: Iterator<Item = &'a mut (Arc<UiObject>, Vec<Arc<Section2d>>)> {
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
