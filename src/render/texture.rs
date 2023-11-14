use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{
        AppResult,
        GameError,
    },
};



#[derive(Debug)]
pub struct DdsImageDecoder<'a> {
    label: Option<&'a str>,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> DdsImageDecoder<'a> {
    #[inline]
    pub const fn new(
        label: Option<&'a str>,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue
    ) -> Self {
        Self { label, device, queue }
    }
}

impl<'a> AssetDecoder for DdsImageDecoder<'a> {
    type Output = wgpu::Texture;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        use ddsfile::Dds;
        use wgpu::util::DeviceExt;
        
        let dds = Dds::read(buf)
            .map_err(|err| game_err!(
                "Texture creation failed",
                "Texture creation failed for the following reasons: {}",
                err.to_string()
            ))?;
        
        Ok(self.device.create_texture_with_data(
            self.queue, 
            &wgpu::TextureDescriptor {
                label: self.label,
                size: wgpu::Extent3d {
                    width: dds.get_width(),
                    height: dds.get_height(),
                    depth_or_array_layers: dds.get_depth()
                },
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Bc3RgbaUnorm,
                mip_level_count: dds.get_num_mipmap_levels(),
                sample_count: 1,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[]
            }, 
            &dds.data
        ))
    }
}
