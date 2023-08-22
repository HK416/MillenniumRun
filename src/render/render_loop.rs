use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};

use winit::window::Window;

use crate::{
    panic_err, 
    app::{GameRenderEvent, AppCmd, PanicErr},
};



/// #### 한국어
/// 게임 랜더링 루프 함수입니다.
/// 
/// #### English (Translation)
/// This is the game rendering loop function.
/// 
pub fn game_render_loop(
    window: Arc<Window>,
    event_receiver: Receiver<GameRenderEvent>,
    message_sender: Sender<AppCmd>,
    running_flag: Arc<AtomicBool>,
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
) {
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let swapchain_alpha_mode = swapchain_capabilities.alpha_modes[0];
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: swapchain_alpha_mode,
        view_formats: vec![]
    };
    surface.configure(&device, &config);

    log::info!("Run :: Game render loop.");
    'render_loop: while running_flag.load(MemOrdering::Acquire) {
        while let Ok(event) = event_receiver.try_recv() {
            match event {
                GameRenderEvent::ApplicationTerminate => {
                    break 'render_loop;
                },
                GameRenderEvent::WindowResized => {
                    config.width = window.inner_size().width; 
                    config.height = window.inner_size().height;
                    surface.configure(&device, &config);
                },
            }
        }


        let frame = success(
            surface.get_current_texture(),
            "Failed to get next image",
            &running_flag,
            &message_sender
        );
        let view = frame.texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None }
        );

        {
            let mut _rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        }
                    })],
                    depth_stencil_attachment: None,
                }
            );
        }
        queue.submit(Some(encoder.finish()));
        frame.present();
    }
    log::info!("End :: Game render loop.");
}


#[inline]
fn success<T, E: ToString>(
    result: Result<T, E>,
    err_title: &str,
    running_flag: &Arc<AtomicBool>,
    message_sender: &Sender<AppCmd>,
) -> T {
    result.unwrap_or_else(|e| {
        let e = panic_err!(err_title, "{}", e.to_string());
        running_flag.store(false, MemOrdering::Release);
        message_sender.send(AppCmd::PanicError(e.clone())).unwrap();
        panic!("{}", e.display());
    })
}


