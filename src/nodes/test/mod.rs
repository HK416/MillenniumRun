use std::sync::Arc;

use winit::{
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
    event::{
        Event,
        WindowEvent,
        ElementState,
    }
};

use crate::{
    game_err,
    assets::{
        bundle::AssetBundle,
        interface::AssetDecoder,
    },
    scene::{
        node::SceneNode,
        state::SceneState,
    },
    system::{
        error::{
            AppResult,
            GameError,
        },
        event::AppEvent,
        shared::Shared,
    },
};


#[derive(Debug)]
pub struct ShaderDecoder;

impl AssetDecoder for ShaderDecoder {
    type Output = String;

    #[inline]
    fn decode(buf: &[u8]) -> AppResult<Self::Output> {
        Ok(String::from_utf8_lossy(buf).into_owned())
    }
}



#[derive(Debug, Default)]
pub struct TestScene {
    module: Option<wgpu::ShaderModule>,
    pipeline: Option<wgpu::RenderPipeline>,
}

impl SceneNode for TestScene { 
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();

        // (한국어) 쉐이더 파일 불러오기.
        // (English Translation) Load shader file.
        let handle = asset_bundle.get("test.wgsl");

        // (한국어) 쉐이더 모듈 생성하기.
        // (English Translation) Create shader module.
        log::info!("Create shader module.");
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("test.wgsl"),
            source: wgpu::ShaderSource::Wgsl(
                pollster::block_on(handle)?
                    .read::<String, ShaderDecoder>()?
                    .into()
            )
        });

        // (한국어) 렌더링 파이프라인 생성하기.
        // (English Translation) Create rendering pipeline.
        log::info!("Create rendering pipeline.");
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("test render pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::default(),
                })],
            }),
            multiview: None,
        });

        self.module = Some(module);
        self.pipeline = Some(pipeline);

        Ok(())
    }

    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        if code == KeyCode::Escape && event.state == ElementState::Released {
                            *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
                        }
                    }
                },
                _ => { /* pass */ }
            },
            _ => { /* pass */ }
        }

        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();

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
                label: Some("draw render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(self.pipeline.as_ref().unwrap());
            rpass.draw(0..3, 0..1);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
