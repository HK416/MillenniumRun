use crate::{
    assets::bundle::AssetBundle, 
    components::{
        ui::{
            brush::UiBrush, 
            objects::{UiObject, UiObjectBuilder}, 
        },
        anchor::Anchor, 
        margin::Margin,
    },
    nodes::path,
    render::texture::DdsTextureDecoder, 
    system::error::AppResult, 
};

pub fn create_background(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    ui_brush: &UiBrush, 
    asset_bundle: &AssetBundle
) -> AppResult<UiObject> {
    // (한국어) 텍스처를 로드하고, 생성합니다.
    // (English Translation) Load and create texture.
    let texture = asset_bundle.get(path::UI_BACKGROUND_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("UiBackground"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 768,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 사용한 에셋을 정리합니다.
    // (English Translation) Release used assets.
    asset_bundle.release(path::UI_BACKGROUND_TEXTURE_PATH);

    // (한국어) 사용자 인터페이스 배경을 생성합니다.
    // (English Translation) Create a user interface background.
    Ok(UiObjectBuilder::new(
        Some("UiBackground"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(1.0, 0.0, 0.0, 1.0))
    .with_margin(Margin::new(0, 0, 0, 0))
    .with_color((1.0, 1.0, 1.0, 1.0).into())
    .with_translation((0.0, 0.0, 0.75).into())
    .build(device))
}
