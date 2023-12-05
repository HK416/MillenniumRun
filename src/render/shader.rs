use crate::{
    assets::interface::AssetDecoder,
    system::error::AppResult,
};


/// #### 한국어 </br>
/// `wgsl` 쉐이더 파일로부터 쉐이더 모듈을 만드는 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder that creates shader modules from `wgsl` shader files. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct WgslDecoder<'a> {
    pub name: Option<&'a str>,
    pub device: &'a wgpu::Device,
}

impl<'a> AssetDecoder for WgslDecoder<'a> {
    type Output = wgpu::ShaderModule;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        // (한국어) 쉐이더 모듈을 생성합니다.
        // (English Translation) Create a shader module.
        Ok(self.device.create_shader_module(wgpu::ShaderModuleDescriptor { 
                label: Some(&format!("ShaderModule({})", match self.name {
                    Some(name) => name,
                    None => "Unknown",
                })),
                source: wgpu::ShaderSource::Wgsl(
                    String::from_utf8_lossy(buf)
                )
            }
        ))
    }
}
