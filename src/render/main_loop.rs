use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::collections::VecDeque;

use winit::window::Window;

use crate::{
    panic_msg, 
    app::{
        abort::PanicMsg,
        message::{
            send,
            success,
            GameRenderEvent,
        },
        running_flag::RunningFlag,
    },
    render::{
        objects::{
            BindGroupLayoutPool,
            BufferPool,
            PipelineLayoutPool,
            RenderPipelinePool,
            ShaderModulePool,
            TextureViewPool,
            TexturePool,
            utils::{
                ref_buffer_obj,
                ref_render_pipeline_obj,
            },
        },
        message::{
            CommandResult,
            RenderCommand,
        },
        task::DrawCommand,
    },
};



/// #### 한국어 </br>
/// 게임 랜더링 루프 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the game rendering loop function. </br>
/// 
pub fn game_render_loop(
    window: Arc<Window>,
    event_receiver: Receiver<GameRenderEvent>,
    render_cmd_receiver: Receiver<(Sender<CommandResult>, RenderCommand)>,
    instance: Arc<wgpu::Instance>,
    surface: Arc<wgpu::Surface>,
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
) {
    // (한국어) 렌더 오브젝트 풀을 생성합니다.
    // (English Translation) Create a render object pool.
    let mut bind_group_layout_pool = BindGroupLayoutPool::new();
    let mut buffer_pool = BufferPool::new();
    let mut pipeline_layout_pool = PipelineLayoutPool::new();
    let mut render_pipeline_pool = RenderPipelinePool::new();
    let mut shader_module_pool = ShaderModulePool::new();
    let mut texture_view_pool = TextureViewPool::new();
    let mut texture_pool = TexturePool::new();

    // (한국어) 렌더 제출 큐를 생성합니다.
    // (English Translation) Create a render submit queue.
    let mut submit_queue = VecDeque::new();

    // (한국어) wgpu 프레임 버퍼를 설정합니다.
    // (English Translation) Set the wgpu framebuffer.
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
    'render_loop: while RunningFlag::is_running() {
        // (한국어) 어플리케이션 이벤트를 처리합니다.
        // (English Translation) Handles application events.
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

        while let Ok((sender, cmd)) = render_cmd_receiver.try_recv() {
            let result = match cmd {
                RenderCommand::CreateBindGroupLayout(ci) => {
                    CommandResult::Return(ci.build(&device, &mut bind_group_layout_pool))
                },
                RenderCommand::CreateBuffer(ci) => {
                    CommandResult::Return(ci.build(&device, &mut buffer_pool))
                },
                RenderCommand::CreateBufferWithData(ci) => {
                    CommandResult::Return(ci.build(&device, &mut buffer_pool))
                }
                RenderCommand::CreatePipelineLayout(ci) => {
                    CommandResult::Return(success(ci.build(
                        &device, 
                        &bind_group_layout_pool, 
                        &mut pipeline_layout_pool
                    )))
                }
                RenderCommand::CreateShaderModule(ci) => {
                    CommandResult::Return(ci.build(&device, &mut shader_module_pool))
                },
                RenderCommand::CreateRenderPipeline(ci) => {
                    CommandResult::Return(success(ci.build(
                        &device, 
                        &pipeline_layout_pool, 
                        &shader_module_pool, 
                        &mut render_pipeline_pool
                    )))
                },
                RenderCommand::Copy(batch) => {
                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    for desc in batch {
                        success(desc.copy(&buffer_pool, &texture_pool, &mut encoder));
                    }
                    queue.submit(Some(encoder.finish()));
                    CommandResult::Finish
                },
                RenderCommand::QuerySwapchainFormat => {
                    CommandResult::QueryTextureFormat(swapchain_format)
                }
                RenderCommand::Submit(passes) => {
                    submit_queue.push_back(passes);
                    CommandResult::Finish
                },
            };
            success(send(result, &sender));
        }


        // (한국어) 다음 프레임을 가져옵니다.
        // (English Translation) Get the next frame.
        let frame = success(surface
            .get_current_texture()
            .map_err(|e| panic_msg!(
                "Failed to get next frame",
                "Failed to get the next frame for the following reasons: {}",
                e.to_string()
            ))
        );

        // (한국어) 프레임 버퍼의 텍스쳐 뷰를 생성합니다.
        // (English Translation) Creates a texture view of the frame buffer.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        // (한국어) 맨 마지막 제출만 사용합니다.
        // (English Translation) Only the very last submission is used.
        while submit_queue.len() > 1 {
            submit_queue.pop_front();
        }

        match submit_queue.front() {
            Some(passes) => {
                for pass in passes {
                    let mut rpass = success(pass.desc.begin(&view, &texture_view_pool, &mut encoder));
                    for command in pass.commands.iter() {
                        match command {
                            DrawCommand::SetPipeline { pipeline } => {
                                rpass.set_pipeline(
                                    success(ref_render_pipeline_obj(pipeline, &render_pipeline_pool)).as_ref()
                                );
                            },
                            DrawCommand::SetIndexBuffer { format, buffer, buffer_range } => {
                                rpass.set_index_buffer(
                                    success(ref_buffer_obj(buffer, &buffer_pool)).as_ref().slice(buffer_range.clone()), 
                                    format.clone()
                                );
                            },
                            DrawCommand::SetVertexBuffer { slot, buffer, buffer_range } => {
                                rpass.set_vertex_buffer(
                                    slot.clone(), 
                                    success(ref_buffer_obj(buffer, &buffer_pool)).as_ref().slice(buffer_range.clone())
                                );
                            },
                            DrawCommand::Draw { vertices, instances } => {
                                rpass.draw(vertices.clone(), instances.clone());
                            }
                            DrawCommand::DrawIndexed { indices, base_vertex, instances } => {
                                rpass.draw_indexed(indices.clone(), base_vertex.clone(), instances.clone());
                            },
                        }
                    }
                }
            },
            None => {
                let mut _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                    label: Some("Default render pass"), 
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })], 
                    depth_stencil_attachment: None,
                });

                
            }
        };

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
    log::info!("End :: Game render loop.");
}
