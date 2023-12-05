//! #### 한국어  </br>
//! 사용자가 언어를 선택했을 경우 애니메이션을 재생한 후 다음 게임 장면으로 변경합니다. </br>
//! 
//! #### English (Translation) </br>
//! If the user selects a language, play the animation 
//! and then change to the next game scene. </br>
//! 
use std::sync::Arc;

use winit::event::Event;

use crate::{
    game_err,
    components::{
        ui::{objects::UiObject, brush::UiBrush, UserInterface}, 
        text::{section::d2::Section2d, brush::TextBrush, Section}, 
        camera::GameCamera,
    },
    nodes::{
        first_time::{
            MAX_BUTTON_SCALE,
            INIT_BUTTON_SCALE,
            FirstTimeSetupScene,
        },
        intro::IntroScene,
    },
    scene::state::SceneState,
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    }, render::depth::DepthBuffer, 
};

const TOTAL_DURATION: f64 = 1.0;
const ANIMATION_TIME: f64 = 0.6;



pub fn handle_events(_this: &mut FirstTimeSetupScene, _shared: &mut Shared, _event: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

pub fn update(this: &mut FirstTimeSetupScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 경과 시간을 갱신합니다.
    // (English Translation) Updates the elapsed time.
    this.elapsed_time += elapsed_time;
    let delta = smooth_step(this.elapsed_time, ANIMATION_TIME);

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    
    // (한국어) 버튼의 알파값을 갱신합니다.
    // (English Translation) Updates the alpha value of the button 
    let alpha = 1.0 - 1.0 * delta;
    for (ui, text) in this.buttons.values_mut() {
        update_ui_alpha(ui, queue, alpha);
        update_text_alpha(text, queue, alpha);
    }

    // (한국어) 버튼의 크기를 갱신합니다.
    // (English Translation) Updates the scale value of the button.
    let scale = INIT_BUTTON_SCALE + (MAX_BUTTON_SCALE - INIT_BUTTON_SCALE) * delta;
    if let Some((ui, text)) = this.buttons.get_mut(&this.language) {
        ui.data.transform.x_axis.x = scale.x;
        ui.data.transform.y_axis.y = scale.y;
        ui.data.transform.z_axis.z = scale.z;
        ui.update_buffer(queue);
        
        text.data.transform.x_axis.x = scale.x;
        text.data.transform.y_axis.y = scale.y;
        text.data.transform.z_axis.z = scale.z;
        text.update_buffer(queue);
    }

    // (한국어) 지속 시간보다 클 경우 다음 게임 장면으로 변경합니다.
    // (English Translation) Changes to the next game scene if it is greater than the duration. 
    if this.elapsed_time >= TOTAL_DURATION {
        *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(IntroScene::default()));
        this.elapsed_time = 0.0;
        return Ok(());
    }

    Ok(())
}

pub fn draw(this: &FirstTimeSetupScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let text_brush = shared.get::<TextBrush>().unwrap();
    let ui_brush = shared.get::<UiBrush>().unwrap();
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
            label: Some("RenderPass(FirstTimeSetupScene(Wait(Ui))))"),
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

        // (한국어) 카메라 바인딩.
        // (English Translation) Bind the camera.
        camera.bind(&mut rpass);

        // (한국어) 유저 인터페이스 오브젝트 그리기.
        // (English Translation) Drawing user interface objects.
        ui_brush.draw(&mut rpass, this.buttons.values().map(|(ui, _)| ui as &dyn UserInterface));

        // // (한국어) 텍스트 그리기.
        // // (English Translation) Drawing texts.
        text_brush.draw_2d(&mut rpass, this.buttons.values().map(|(_, text)| text as &dyn Section));
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
/// 사용자 인터페이스 색상을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the color of the user interface. </br>
/// 
#[inline]
fn update_ui_alpha(ui: &mut UiObject, queue: &wgpu::Queue, alpha: f32) {
    ui.data.color.w = alpha;
    ui.update_buffer(queue);
}


/// #### 한국어 </br>
/// 텍스트의 색상을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the color of the text. </br>
/// 
#[inline]
fn update_text_alpha(text: &mut Section2d, queue: &wgpu::Queue, alpha: f32) {
    text.data.color.w = alpha;
    text.update_buffer(queue);
}
