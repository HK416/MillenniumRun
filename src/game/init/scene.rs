use async_std::task;

use crate::{
    panic_msg,
    app::{
        abort::{PanicMsg, AppResult},
        message::GameLogicEvent,
    },
    assets::bundle::AssetBundle,
    game::assets::shader::ShaderDecoder,
    logic::{
        resource::Resources,
        scene::{GameScene, NextScene}
    },
    render::{
        descriptor::{
            ShaderModuleDesc,
            ShaderSourceDesc,
            VertexStateDesc,
            FragmentStateDesc,
            RenderPipelineDesc,
            RenderPassDesc,
            RenderPassColorAttachmentDesc,
        },
        identifier::IDHandle,
        message::{RenderCommand, RenderCommandChannel},
        task::{SubmitRenderPass, DrawCommand, RenderSubmitChannel}
    },
};



#[derive(Debug)]
pub struct InitScene {
    test_shader: Option<IDHandle>,
    test_pipeline: Option<IDHandle>,
}

impl InitScene {
    pub fn new() -> Self {  
        Self { 
            test_shader: None,
            test_pipeline: None,
        }
    }
}

impl GameScene for InitScene {
    fn enter(&mut self, res: &mut Resources) -> AppResult<()> {
        log::debug!("InitScene::enter");
        
        // (한국어) 사용할 리소스 가져오기.
        let asset_bundle = res.get::<AssetBundle>()
            .ok_or_else(|| panic_msg!(
                "Failed to find resource", "AssetBundle is not registered!"
            ))?;
        
        // (한국어) 테스트 쉐이더 에셋 불러오기.
        let h_shader = asset_bundle.get("test.wgsl");

        // (한국어) 쉐이더 모듈 렌더 오브젝트 생성하기.
        let future = RenderCommandChannel::push(
            RenderCommand::CreateShaderModule(
                ShaderModuleDesc { 
                    label: Some("test shader".to_string()), 
                    source: ShaderSourceDesc::Wgsl(task::block_on(h_shader)?.read::<String, ShaderDecoder>()?)
                }
            )
        );
        let module = future.get_wait()
            .return_or_else(|| panic_msg!(
                "Render object creation failed", "Shader module object creation failed..."
            ))?;

        // (한국어) 스왑체인 텍스쳐 형식 가져오기.
        let future = RenderCommandChannel::push(RenderCommand::QuerySwapchainFormat);
        let swapchain_format = future.get_wait()
            .texture_format_or_else(|| panic_msg!(
                "Texture format query failed", "Failed to get swapchain texture format."
            ))?;

        // (한국어) 렌더 파이프라인 오브젝트 생성하기.
        let future = RenderCommandChannel::push(
            RenderCommand::CreateRenderPipeline(
                RenderPipelineDesc { 
                    label: Some("test pipeline".to_string()), 
                    layout: None, 
                    vertex: VertexStateDesc { 
                        module: module.clone(), 
                        entry_point: "vs_main".to_string(), 
                        buffers: vec![] 
                    }, 
                    primitive: wgpu::PrimitiveState::default(), 
                    depth_stencil: None, 
                    multisample: wgpu::MultisampleState::default(), 
                    fragment: Some(FragmentStateDesc { 
                        module: module.clone(), 
                        entry_point: "fs_main".to_string(), 
                        targets: vec![Some(wgpu::ColorTargetState { 
                            format: swapchain_format, 
                            blend: None, 
                            write_mask: wgpu::ColorWrites::default(),
                        })]
                    }), 
                    multiview: None,
                }
            )
        );
        let pipeline = future.get_wait()
            .return_or_else(|| panic_msg!(
                "Render object creation failed", "Render pipeline object creation failed..."
            ))?;


        self.test_shader = Some(module);
        self.test_pipeline = Some(pipeline);
        
        Ok(())
    }

    fn exit(&mut self, _res: &mut Resources) -> AppResult<()> {
        log::debug!("InitScene::exit");
        Ok(())
    }
    
    fn handle_events(&mut self, _event: &GameLogicEvent) -> AppResult<()> {
        Ok(())
    }

    fn update(&mut self, _elapsed_time_sec: f64) -> AppResult<()> {
        Ok(())
    }

    fn render_submit(&self, _res: &mut Resources) -> AppResult<()> {
        RenderSubmitChannel::upload(vec![
                SubmitRenderPass {
                    desc: RenderPassDesc { 
                        label: None, 
                        color_attachments: vec![Some(
                            RenderPassColorAttachmentDesc { 
                                view: None, 
                                resolve_target: None, 
                                ops: wgpu::Operations { 
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                                    store: true 
                                }
                            }
                        )], 
                        depth_stencil_attachments: None 
                    },
                    commands: vec![
                        DrawCommand::SetPipeline { pipeline: self.test_pipeline.clone().unwrap() },
                        DrawCommand::Draw { vertices: 0..3, instances: 0..1 },
                    ]
                }
            ]
        );

        Ok(())
    }
    
    fn next(&self) -> NextScene {
        NextScene::Keep
    }
}
