use std::sync::Arc;

use crate::{
    game_err,
    components::{ui::{UserInterface, brush::UiBrush}, camera::GameCamera},
    nodes::{intro::IntroScene, title::EnterTitleScene},
    render::depth::DepthBuffer,
    scene::state::SceneState,
    system::{
        error::{AppResult, GameError},
        shared::Shared,
    },
};

/// #### 한국어 </br>
/// `FadeOut` 상태의 지속 시간입니다. </br>
/// 
/// #### English (Translation) </br>
/// Duration of the `FadeOut` state. </br>
/// 
const DURATION: f64 = 0.5;



/// #### 한국어 </br>
/// `intro` 게임 장면의 `FadeOut` 상태일 때 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function when the `intro` game scene is in the `FadeOut` state. </br>
/// 
pub fn update(this: &mut IntroScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 경과 시간을 갱신합니다.
    // (English Translation) Updates the elapsed time.
    this.elapsed_time += elapsed_time;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

    // (한국어) 로고 이미지의 알파 값을 시간에 따라 갱신합니다.
    // (English Translation) Updates the alpha value of the logo image over time.
    let delta_time = (this.elapsed_time / DURATION).min(1.0) as f32;
    let alpha = 1.0 - 1.0 * delta_time;
    for ui in this.logo_images.iter_mut() {
        ui.update_buffer(queue, |data| {
            data.color.w = alpha;
        });
    }

    // (한국어) 지속 시간보다 클 경우 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state if it is greater than the duration.
    if this.elapsed_time >= DURATION {
        *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(EnterTitleScene::default()));
        return Ok(());
    }

    Ok(())
}



/// #### 한국어 </br>
/// `intro` 게임 장면의 `FadeOut` 상태일 때 그리기 함수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a drawing function when the `intro` game scene is in the `FadeOut` state. </br>
/// 
pub fn draw(this: &IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
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
            label: Some("RenderPass(IntroScene(FadeOut(Ui)))"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                view: &view, 
                resolve_target: None, 
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
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
        ui_brush.draw(&mut rpass, this.logo_images.iter().map(|it| it as &dyn UserInterface));
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
