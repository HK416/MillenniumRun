use std::sync::Arc;

use crate::{
    game_err,
    components::{ui::UiBrush, camera::GameCamera},
    nodes::intro::{IntroScene, state::IntroState},
    render::depth::DepthBuffer,
    system::{
        error::{AppResult, GameError},
        shared::Shared,
    },
};



/// #### 한국어 </br>
/// `intro` 게임 장면의 `WaitLoading` 상태일 때 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function when the `intro` game scene is in the `WaitLoading` state. </br>
/// 
pub fn update(this: &mut IntroScene, _shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    // (한국어) 
    // 다음 장면의 게임 에셋 로드가 완료되었을 경우 다음 상태로 변경합니다.
    // 
    // (English Translation) 
    // If the game asset loading for the next scene is complete,
    // it changes to the next state.
    // 
    if this.loading.as_ref().unwrap().is_finished() {
        this.loading.take().unwrap().join().unwrap()?;
        this.state = IntroState::FadeOut;
        this.timer = 0.0;
        return Ok(());
    }

    Ok(())
}



/// #### 한국어 </br>
/// `intro` 게임 장면의 `WaitLoading` 상태일 때 그리기 함수 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a drawing function when the `intro` game scene is in the `WaitLoading` state. </br>
/// 
pub fn draw(this: &IntroScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass(IntroScene(WaitLoading(Ui)))"),
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
        ui_brush.draw(&mut rpass, [&this.logo].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
