//! #### 한국어 </br>
//! 개발을 위해 `Debug`모드일 때 포함되는 게임 장면 상태입니다. </br>
//! 오브젝트의 위치를 확인하는 용도로 사용됩니다. </br>
//! 
//! #### English (Translation) </br>
//! This is the game scene state included when in `Debug` mode for development. </br>
//! It is used to check the position of the object. </br>
//! 

use std::sync::{Arc, Mutex};

use winit::{
    event::{Event, WindowEvent},
    keyboard::{PhysicalKey, KeyCode},
};

use crate::{
    game_err,
    components::{
        sprite::brush::SpriteBrush,
        transform::{Transform, Projection},
        camera::GameCamera,
    },
    nodes::{
        consts, 
        title::{
            TitleScene, 
            state::TitleState
        }
    },
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};


static PREV: Mutex<Option<(Transform, Projection)>> = Mutex::new(None);

pub fn handle_events(this: &mut TitleScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(code) = event.physical_key {
                if !event.repeat && event.state.is_pressed() && code == KeyCode::F4 {
                    // (한국어) 사용할 공유 객체 가져오기.
                    // (English Translation) Get shared object to use.
                    let queue = shared.pop::<Arc<wgpu::Queue>>().unwrap();
                    let mut camera = shared.pop::<GameCamera>().unwrap();

                    // (한국어) 이전 카메라 데이터를 가져옵니다.
                    // (English Translation) Get previous camera data.
                    let (transform, projection) = PREV.lock()
                        .expect("Failed to access previous data.")
                        .take()
                        .unwrap();

                    // (한국어) 카메라를 설정 합니다.
                    // (English Translation) Set up the camera.
                    camera.projection = projection;
                    camera.transform = transform;
                    camera.update_buffer(&queue);

                    // (한국어) 사용을 완료한 에셋을 반환합니다.
                    // (English Translation) Returns assets that have been used.
                    shared.push(camera);
                    shared.push(queue);

                    this.elapsed_time = 0.0;
                    this.state = TitleState::Menu;
                    return Ok(());
                }
            },
            _ => { }
        },
        _ => { }
    }
    Ok(())
}

pub fn update(_this: &mut TitleScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    if PREV.lock().expect("Failed to access previouse data.").is_none() {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let queue = shared.pop::<Arc<wgpu::Queue>>().unwrap();
        let mut camera = shared.pop::<GameCamera>().unwrap();

        // (한국어) 카메라 데이터를 가져옵니다.
        // (English Translation) Get the camera data.
        *PREV.lock().expect("Failed to access previous data.") = Some((
            camera.transform,
            camera.projection
        ));

        // (한국어) 카메라를 설정 합니다.
        // (English Translation) Set up the camera.
        camera.projection = Projection::new_ortho(
            9.0 * consts::PIXEL_PER_METER, 
            -16.0 * consts::PIXEL_PER_METER, 
            -9.0 * consts::PIXEL_PER_METER, 
            16.0 * consts::PIXEL_PER_METER, 
            0.0 * consts::PIXEL_PER_METER, 
            1000.0 * consts::PIXEL_PER_METER 
        );
        camera.transform.set_position((
            0.0 * consts::PIXEL_PER_METER,
            0.0 * consts::PIXEL_PER_METER,
            0.0 * consts::PIXEL_PER_METER
        ).into());
        camera.update_buffer(&queue);

        // (한국어) 사용을 완료한 에셋을 반환합니다.
        // (English Translation) Returns assets that have been used.
        shared.push(camera);
        shared.push(queue);
    }

    Ok(())
}

pub fn draw(this: &TitleScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
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
            label: Some("RenderPass(TitleScene(DevState))"),
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

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
