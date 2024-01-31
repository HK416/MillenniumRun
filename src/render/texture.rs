use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{AppResult, GameError},
};


/// #### 한국어 </br>
/// `dds` 이미지 파일로부터 텍스처를 만드는 디코더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a decoder that creates texture from `dds` image files. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct DdsTextureDecoder<'a> {
    pub name: Option<&'a str>,
    pub size: wgpu::Extent3d,
    pub dimension: wgpu::TextureDimension,
    pub format: wgpu::TextureFormat,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub usage: wgpu::TextureUsages,
    pub view_formats: &'a [wgpu::TextureFormat],
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

impl<'a> AssetDecoder for DdsTextureDecoder<'a> {
    type Output = wgpu::Texture;

    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        use ddsfile::Dds;
        use wgpu::util::DeviceExt;

        let dds = Dds::read(buf)
            .map_err(|err| game_err!(
                "Image decoding failed",
                "Image decoding failed for the following reasons: {}",
                err.to_string()
            ))?;

        let texture = self.device.create_texture_with_data(
            self.queue, 
            &wgpu::TextureDescriptor {
                label: Some(&format!("Texture({})", self.name.unwrap_or("Unknown"))),
                size: self.size,
                dimension: self.dimension,
                format: self.format,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                usage: self.usage,
                view_formats: self.view_formats,
            }, 
            wgpu::util::TextureDataOrder::LayerMajor,
            &dds.data
        );

        Ok(texture)
    }
}
