mod bind_group_layout;
mod buffer;
// mod mesh;
mod pipeline_layout;
mod pipeline;
mod shader;
mod texture_view;
mod texture;


pub(super) use self::{
    bind_group_layout::BindGroupLayoutPool,
    buffer::BufferPool,
    pipeline_layout::PipelineLayoutPool,
    pipeline::RenderPipelinePool,
    shader::ShaderModulePool,
    texture_view::TextureViewPool,
    texture::TexturePool,
};

pub use self::{
    bind_group_layout::BindGroupLayoutObj,
    buffer::BufferObj,
    pipeline_layout::PipelineLayoutObj,
    pipeline::RenderPipelineObj,
    shader::ShaderModuleObj,
    texture_view::TextureViewObj,
    texture::TextureObj,
};



pub mod utils {
    use crate::{
        panic_msg,
        app::abort::{PanicMsg, AppResult},
        render::{
            identifier::IDHandle,
            objects::{
                BindGroupLayoutObj,
                BindGroupLayoutPool,
                BufferObj,
                BufferPool,
                PipelineLayoutObj,
                PipelineLayoutPool,
                RenderPipelineObj,
                RenderPipelinePool,
                ShaderModuleObj,
                ShaderModulePool,
                TextureViewObj,
                TextureViewPool,
                TextureObj,
                TexturePool,
            },
        },
    };

    const ERR_TITLE_NOT_FOUND: &'static str = "Failed to get rendering object";


    /// #### 한국어 </br>
    /// `BindGroupLayoutObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `BindGroupLayoutObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that brrows `BindGroupLayoutObj`. </br>
    /// Returns `PanicMsg` if the `BindGroupLayoutObj` is not found in the pool. </br>
    /// 
    pub fn ref_bind_group_layout_obj<'a>(
        id: &IDHandle,
        pool: &'a BindGroupLayoutPool,
    ) -> AppResult<&'a BindGroupLayoutObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered bind group layout found!"
        ))
    }

    /// #### 한국어 </br>
    /// `BufferObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `BufferObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation)
    /// This is a function that brrows `BufferObj`. </br>
    /// Returns `PanicMsg` if the `BufferObj` is not found in the pool. </br>
    /// 
    pub fn ref_buffer_obj<'a>(
        id: &IDHandle,
        pool: &'a BufferPool
    ) -> AppResult<&'a BufferObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered buffer found."
        ))
    }

    /// #### 한국어 </br>
    /// `PipelineLayoutObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `PipelineLayoutObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that borrows `PipelineLayoutObj`. </br>
    /// Returns `PanicMsg` if the `PipelineLayoutObj` is not found in the pool. </br>
    /// 
    pub fn ref_pipeline_layout_obj<'a>(
        id: &IDHandle,
        pool: &'a PipelineLayoutPool,
    ) -> AppResult<&'a PipelineLayoutObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered pipeline layout found!"
        ))
    }

    /// #### 한국어 </br>
    /// `RenderPipelineObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `RenderPipelineObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that borrows `RenderPipelineObj`. </br>
    /// Returns `PanicMsg` if the `RenderPipelineObj` is not found in the pool. </br>
    /// 
    pub fn ref_render_pipeline_obj<'a>(
        id: &IDHandle,
        pool: &'a RenderPipelinePool,
    ) -> AppResult<&'a RenderPipelineObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered render pipeline found!"
        ))
    }

    /// #### 한국어 </br>
    /// `ShaderModuleObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `ShaderModuleObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that borrows `ShaderModuleObj`. </br>
    /// Returns `PanicMsg` if the `ShaderModuleObj` is not found in the pool. </br>
    /// 
    pub fn ref_shader_module_obj<'a>(
        id: &IDHandle,
        pool: &'a ShaderModulePool,
    ) -> AppResult<&'a ShaderModuleObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered shader module found!"
        ))
    }

    /// #### 한국어 </br>
    /// `TextureViewObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `TextureViewObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that borrows `TextureViewObj`. </br>
    /// Returns `PanicMsg` if the `TextureViewObj` is not found in the pool. </br>
    /// 
    pub fn ref_texture_view_obj<'a>(
        id: &IDHandle, 
        pool: &'a TextureViewPool
    ) -> AppResult<&'a TextureViewObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered texture view found!"
        ))
    }

    /// #### 한국어 </br>
    /// `TextureObj`를 빌려오는 함수입니다. </br>
    /// 풀에서 `TextureObj`를 찾을 수 없는 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a function that borrows `TextureObj`. </br>
    /// Returns `PanicMsg` if the `TextureObj` is not found in the pool. </br>
    /// 
    pub fn ref_texture_obj<'a>(
        id: &IDHandle,
        pool: &'a TexturePool,
    ) -> AppResult<&'a TextureObj> {
        pool.get(id).ok_or_else(|| panic_msg!(
            ERR_TITLE_NOT_FOUND, "No registered texture found!"
        ))
    }
}
