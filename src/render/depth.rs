use winit::window::Window;



/// #### 한국어 </br>
/// 깊이 테스트에 사용되는 깊이 버퍼 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a depth buffer used for depth testing. </br>
/// 
#[derive(Debug)]
pub struct DepthBuffer {
    texture_view: wgpu::TextureView,
}

impl DepthBuffer {
    pub fn new(window: &Window, device: &wgpu::Device) -> Self {
        // (한국어) 깊이 버퍼 텍스처를 생성합니다.
        // (English Translation) Create a depth buffer texture. 
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Depth Buffer"),
                size: wgpu::Extent3d {
                    width: window.inner_size().width,
                    height: window.inner_size().height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[]
            }
        );

        // (한국어) 깊이 버퍼 텍스처 뷰를 생성합니다.
        // (English Translation) Create a depth buffer texture view.
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        Self { texture_view }
    }

    #[inline]
    pub fn view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}
