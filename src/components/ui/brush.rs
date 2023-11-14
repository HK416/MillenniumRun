use glam::Mat4;

use crate::{
    assets::bundle::AssetBundle, 
    system::error::AppResult
};

use super::{
    pipeline::Pipeline, 
    objects::UiObject
};



/// #### 한국어 </br>
/// 유저 인터페이스를 화면에 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// A tool for drawing user interface on the screen. </br>
/// 
#[derive(Debug)]
pub struct UiBrush {
    pipeline: Pipeline,
}

impl UiBrush {
    pub fn new(
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        multiview: Option<std::num::NonZeroU32>,
        asset_bundle: &AssetBundle,
        ortho: Mat4,
    ) -> AppResult<Self> {
        // (한국어) 유저 인터페이스 렌더링 파이프라인을 생성합니다.
        // (English Translation) Create a user interface rendering pipeline.
        let pipeline = Pipeline::new(
            device,
            render_format,
            depth_stencil,
            multisample,
            multiview,
            asset_bundle,
            ortho
        )?;

        Ok(Self { pipeline })
    }

    /// #### 한국어 </br>
    /// 렌더링 파이프라인의 바인드 그룹 레이아웃을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the bind group layout from the rendering pipeline. </br>
    /// 
    #[inline]
    pub fn ref_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.pipeline.texture_bind_group_layout
    }

    /// #### 한국어 </br>
    /// 주어진 유저 인터페이스 오브젝트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given user interface objects on the screen. </br>
    /// 
    #[inline]
    pub fn draw<'pass>(
        &'pass self,
        objects: &'pass [&'pass dyn UiObject],
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        self.pipeline.bind(rpass);
        for object in objects {
            object.bind(rpass);
            object.draw(rpass);
        }
    }
}