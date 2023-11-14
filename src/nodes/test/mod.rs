use std::sync::Arc;

use ab_glyph::FontArc;
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
    assets::bundle::AssetBundle,
    components::text::{
        brush::TextBrush,
        section::{
            Align,
            Section, 
            SectionBuilder,
        },
    },
    nodes::{
        consts,
        path,
    },
    render::{
        depth::DepthBuffer,
        shader::WgslDecoder
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



#[derive(Debug, Default)]
pub struct TestScene {
    module: Option<wgpu::ShaderModule>,
    pipeline: Option<wgpu::RenderPipeline>,
    hello_en: Option<Section>,
    hello_jp: Option<Section>,
    hello_kr: Option<Section>,
    total_time_txt: Option<Section>,
}

impl SceneNode for TestScene { 
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let font = shared.get::<FontArc>().unwrap();
        let config = shared.get::<wgpu::SurfaceConfiguration>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();

        // (한국어) 텍스트 생성하기.
        // (English Translation) Create a text.
        let text0 = SectionBuilder::new(font, 64.0, "Hello!")
            .with_align(Align::Center((0.0, 3.0 * consts::PIXEL_PER_METER).into()))
            .build(device);
        let text1 = SectionBuilder::new(font, 64.0, "こんにちは!")
            .with_align(Align::Center((0.0, -3.0 * consts::PIXEL_PER_METER).into()))
            .build(device);
        let text2 = SectionBuilder::new(font, 64.0, "안녕하세요!")
            .build(device);

        // (한국어) 쉐이더 모듈 생성하기.
        // (English Translation) Create shader module.
        let module = asset_bundle.get(path::TEST_SHADER_PATH)?
            .read(&WgslDecoder::new(Some("Test Shader Module"), &device))?;

        // (한국어) 렌더링 파이프라인 생성하기.
        // (English Translation) Create rendering pipeline.
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("test render pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare:wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
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
        self.hello_en = Some(text0.into());
        self.hello_jp = Some(text1.into());
        self.hello_kr = Some(text2.into());

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
    
    fn update(&mut self, shared: &mut Shared, total_time: f64, _: f64) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let font = shared.get::<FontArc>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();

        let total_time_text = SectionBuilder::new(font, 24.0, &format!("Total Time {:.2}", total_time))
            .with_align(Align::BottomLeft((-8.0 * consts::PIXEL_PER_METER, 4.5 * consts::PIXEL_PER_METER).into()))
            .build(device);

        self.total_time_txt = Some(total_time_text.into());

        std::thread::sleep(std::time::Duration::from_millis(8));
        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let mut brush = shared.pop::<TextBrush>().unwrap();
        let font = shared.get::<FontArc>().unwrap();
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let sections = [
            self.hello_en.as_ref().unwrap(),
            self.hello_jp.as_ref().unwrap(),
            self.hello_kr.as_ref().unwrap(),
            self.total_time_txt.as_ref().unwrap(),
        ];

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
                label: Some("object render pass"),
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

            rpass.set_pipeline(self.pipeline.as_ref().unwrap());
            rpass.draw(0..3, 0..1);
        }
        let command_buffer0 = encoder.finish();

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("interface render pass"),
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

            brush.update_texture(font, device, queue, &sections);
            brush.draw(&sections, &mut rpass);
        }
        let command_buffer1 = encoder.finish();

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit([command_buffer0, command_buffer1]);
        frame.present();

        shared.push(brush);

        Ok(())
    }
}
