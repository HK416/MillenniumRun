use crate::{
    assets::interface::AssetDecoder,
    system::error::AppResult,
};


/// #### 한국어 </br>
#[derive(Debug)]
pub struct WgslDecoder<'a> {
    label: Option<&'a str>,
    device: &'a wgpu::Device,
}

impl<'a> WgslDecoder<'a> {
    #[inline]
    pub const fn new(
        label: Option<&'a str>, 
        device: &'a wgpu::Device
    ) -> Self {
        Self { label, device }
    }
}

impl<'a> AssetDecoder for WgslDecoder<'a> {
    type Output = wgpu::ShaderModule;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        Ok(
            self.device.create_shader_module(wgpu::ShaderModuleDescriptor { 
                label: self.label, 
                source: wgpu::ShaderSource::Wgsl(
                    String::from_utf8_lossy(buf)
                )
            })
        )
    }
}
