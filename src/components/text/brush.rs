use glam::{Mat4, Vec4};
use ab_glyph::{
    Font, 
    FontArc, 
};

use crate::{
    assets::bundle::AssetBundle, 
    system::error::AppResult
};

use super::{
    pipeline::Pipeline, 
    section::Section,
};



/// #### 한국어 </br>
/// 텍스트를 화면에 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// A tool for drawing text on the screen. </br>
/// 
#[derive(Debug)]
pub struct TextBrush {
    pipeline: Pipeline,
}

impl TextBrush {
    pub fn new(
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        multiview: Option<std::num::NonZeroU32>,
        asset_bundle: &AssetBundle,
        ortho: Mat4,
    ) -> AppResult<Self> {
        // (한국어) 텍스트 렌더링 파이프라인을 생성합니다.
        // (English Translation) Create a text rendering pipeline.
        let pipeline = Pipeline::new(
            device,
            render_format,
            depth_stencil,
            multisample,
            multiview,
            asset_bundle,
            ortho,
        )?;

        Ok(Self { pipeline })
    }

    /// #### 한국어 </br>
    /// 텍스트의 유니폼 버퍼를 갱신합니다. </br>
    ///
    /// #### English (Translation) </br>
    /// Updates the uniform buffer of the text. </br>
    /// 
    #[inline]
    pub fn update_uniform(&self, queue: &wgpu::Queue, ortho: Mat4) {
        self.pipeline.update_uniform(queue, ortho);
    }

    /// #### 한국어 </br>
    /// 텍스트의 텍스처를 갱신합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the textureof the text. </br>
    /// 
    #[inline]
    pub fn update_texture(
        &mut self,
        font: &FontArc,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sections: &[&Section],
    ) {
        let font = font.as_scaled(128.0);
        for section in sections {
            self.pipeline.update_texture(&font, section, device, queue);
        }
    }

    /// #### 한국어 </br>
    /// 텍스트를 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws text to the screen. </br>
    /// 
    #[inline]
    pub fn draw<'pass>(
        &'pass self, 
        sections: &'pass [&'pass Section],
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        self.pipeline.bind(rpass);
        for section in sections {
            self.pipeline.draw(section, rpass);
        }
    }
}
