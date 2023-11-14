use std::collections::HashMap;

use wgpu::util::DeviceExt;
use ab_glyph::{Font, ScaleFont};



/// #### 한국어 </br>
/// 텍스트 렌더링에 사용되는 텍스처 집합 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a set of textures used for text rendering. </br>
/// 
#[derive(Debug)]
pub struct TextureMap {
    pub bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: HashMap<char, wgpu::BindGroup>,
}

impl TextureMap {
    pub fn new(device: &wgpu::Device) -> Self {
        // (한국어) 바인드 그룹 레이아웃을 생성합니다.
        // (English Translation) Create a bind group layout.
        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Text - Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                            multisampled: false 
                        },
                        count: None,
                    }
                ]
            }
        );

        // (한국어) 바인드 그룹을 생성합니다.
        // (English Translation) Create a bind group.
        let bind_groups = HashMap::new();

        Self { bind_group_layout, bind_groups }
    }

    /// #### 한국어 </br>
    /// 텍스처 집합을 갱신합니다. </br>
    /// 주어진 문자에 대한 텍스처가 존재하지 않는 경우 새로운 텍스처를 생성합니다. </br>
    /// 
    /// #### English (Translation) </br> 
    /// Updates the texture set. </br>
    /// If a texture does not exist for a given character, it creates a new texture. </br>
    /// 
    pub fn update<F: Font, SF: ScaleFont<F>>(
        &mut self,
        ch: char,
        font: &SF,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // (한국어) 미리 생성한 텍스처가 존재하지 않을 경우 텍스처를 생성합니다.
        // (English Translation) If a pre-generated texture does not exist, create a texture.
        if !self.bind_groups.contains_key(&ch) {
            let glyph = font.scaled_glyph(ch);
            if let Some(outline) = font.outline_glyph(glyph) {
                let bound = outline.px_bounds();
                let width = bound.width().ceil() as u32;
                let height = bound.height().ceil() as u32;
                let mut data = vec![0; (width * height) as usize];
                outline.draw(|x, y, v| data[(width * y + x) as usize] = (v * 255.0) as u8);
    
                let texture = device.create_texture_with_data(
                    queue, 
                    &wgpu::TextureDescriptor {
                        label: Some(format!("Text({}) - Texture", ch).as_str()),
                        size: wgpu::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::R8Unorm,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[]
                    }, 
                    &data
                );

                let bind_group = device.create_bind_group(
                    &wgpu::BindGroupDescriptor { 
                        label: Some(format!("Text({}) - Bind Group", ch).as_str()), 
                        layout: &self.bind_group_layout, 
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(
                                    &texture.create_view(&wgpu::TextureViewDescriptor::default())
                                )
                            }
                        ] 
                    }
                );
    
                self.bind_groups.insert(ch, bind_group);
            }
        }
    }

    /// #### 한국어 </br>
    /// 주어진 문자의 텍스처 바인드 그룹을 빌려옵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows the texture bind group for the given character. </br>
    /// 
    #[inline]
    pub fn ref_bind_group<'pass>(&'pass self, ch: char) -> Option<&'pass wgpu::BindGroup>  {
        self.bind_groups.get(&ch)
    }
}



/// #### 한국어 </br>
/// 텍스트 렌더링에 사용되는 텍스처 샘플러 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a texture sampler used for text rendering. </br>
///  
#[derive(Debug)]
pub struct TextureSampler {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl TextureSampler {
    pub fn new(device: &wgpu::Device) -> Self {
        // (한국어) 텍스처 샘플러를 생성합니다.
        // (English Translation) Create a texture sampler.
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Text - Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        // (한국어) 바인드 그룹 레이아웃을 생성합니다.
        // (English Translation) Create a bind group layout.
        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Text - Sampler Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ]
            }
        );

        // (한국어) 바인드 그룹을 생성합니다.
        // (English Translation) Create a bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Text - Sampler Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            }
        );

        Self { bind_group_layout, bind_group }
    }
}
