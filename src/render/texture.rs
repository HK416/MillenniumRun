use crate::{
    game_err,
    assets::interface::AssetDecoder,
    system::error::{AppResult, GameError},
};


#[derive(Debug, Clone, Copy)]
pub struct ImageDecoder<'a> {
    label: Option<&'a str>,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> ImageDecoder<'a> {
    #[inline]
    pub const fn new(label: Option<&'a str>, device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> Self {
        Self { label, device, queue }
    }
}

impl<'a> AssetDecoder for ImageDecoder<'a> {
    type Output = wgpu::Texture;

    #[inline]
    fn decode(&self, buf: &[u8]) -> AppResult<Self::Output> {
        use std::io::Cursor;
        use image::{EncodableLayout, io::Reader};
        use wgpu::util::DeviceExt;

        let img = Reader::new(Cursor::new(buf))
            .with_guessed_format()
            .map_err(|err| game_err!(
                "Image decoding failed",
                "Image decoding failed for the following reasons: {}",
                err.to_string()
            ))?
            .decode()
            .map_err(|err| game_err!(
                "Image decoding failed",
                "Image decoding failed for the following reasons: {}",
                err.to_string()
            ))?;

        let texture = self.device.create_texture_with_data(
            self.queue, 
            &wgpu::TextureDescriptor {
                label: self.label,
                size: wgpu::Extent3d {
                    width: img.width(),
                    height: img.height(),
                    depth_or_array_layers: 1
                },
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[]
            }, 
            img.to_rgba8().as_bytes()
        );

        Ok(texture)
    }
}
