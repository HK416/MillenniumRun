use crate::{
    assets::bundle::AssetBundle, 
    components::{
        ui::{UiBrush, UiObject, UiObjectBuilder}, 
        anchor::Anchor, 
        margin::Margin, 
        player::Actor,
    },
    render::texture::DdsTextureDecoder, 
    system::error::AppResult, 
};



pub fn create_background(
    actor: Actor, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    ui_brush: &UiBrush, 
    asset_bundle: &AssetBundle
) -> AppResult<Vec<UiObject>> {
    use crate::nodes::path;

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
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용한 에셋을 정리합니다.
    // (English Translation) Release used assets.
    asset_bundle.release(path::UI_BACKGROUND_TEXTURE_PATH);

    // (한국어) 사용자 인터페이스 배경을 생성합니다.
    // (English Translation) Create a user interface background.
    let background = UiObjectBuilder::new(
        Some("UiBackground"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(1.0, 0.0, 0.0, 1.0))
    .with_margin(Margin::new(0, 0, 0, 0))
    .with_color((1.0, 1.0, 1.0, 1.0).into())
    .with_global_translation((0.0, 0.0, 0.75).into())
    .build(device);


    // FIXME
    // (한국어) 텍스처를 로드하고, 생성합니다.
    // (English Translation) Load and create texture.
    let rel_path = match actor {
        _ => path::TEMP_STAGE_TEXTURE_PATH,
    };
    let texture = asset_bundle.get(rel_path)?
        .read(&DdsTextureDecoder {
            name: Some("Image"),
            size: wgpu::Extent3d {
                width: 2048,
                height: 2048,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 12,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용한 에셋을 정리합니다. 
    // (English Translation) Release used assets.
    asset_bundle.release(rel_path);

    // (한국어) 이미지를 생성합니다.
    // (English Translation) Create a image.
    let image = UiObjectBuilder::new(
        Some("Image"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(
        0.9166666667 + 0.004166666666, 
        0.0625 + 0.003125, 
        0.08333333333 + 0.004166666666, 
        0.6875 + 0.003125
    ))
    .with_margin(Margin::new(0, 0, 0, 0))
    .with_color((1.0, 1.0, 1.0, 1.0).into())
    .with_global_translation((0.0, 0.0, 0.75).into())
    .build(device);

    return Ok(vec![
        background, 
        image, 
    ])
}
