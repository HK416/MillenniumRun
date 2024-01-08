use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};

use winit::dpi::PhysicalPosition; 
use glam::{Mat4, Vec4, Vec3, Quat};
use bytemuck::{Pod, Zeroable, offset_of};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        collider2d::Collider2d, 
        anchor::Anchor, 
        margin::Margin, 
        camera::GameCamera, 
    },
    render::shader::WgslDecoder, 
    system::error::AppResult, 
};



/// #### 한국어 </br>
/// 사용자 인터페이스를 렌더링하는데 사용되는 인스턴스 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains instance data used to render the user interface. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct InstanceData {
    pub local: Mat4,
    pub global: Mat4, 
    pub anchor: Anchor,
    pub margin: Margin, 
    pub color: Vec4, 
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self { 
            local: Mat4::IDENTITY, 
            global: Mat4::IDENTITY, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4::new(1.0, 1.0, 1.0, 1.0), 
        }
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스를 렌더링하는데 사용되는 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data used to render the user interface. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiData {
    pub local_scale: Vec3, 
    pub local_rotation: Quat, 
    pub local_translation: Vec3,
    pub global_scale: Vec3, 
    pub global_rotation: Quat, 
    pub global_translation: Vec3,
    pub anchor: Anchor,
    pub margin: Margin, 
    pub color: Vec4, 
}

impl UiData {
    #[inline]
    fn to_instance(&self) -> InstanceData {
        InstanceData { 
            local: Mat4::from_scale_rotation_translation(
                self.local_scale, 
                self.local_rotation, 
                self.local_translation
            ), 
            global: Mat4::from_scale_rotation_translation(
                self.global_scale, 
                self.global_rotation, 
                self.global_translation
            ), 
            anchor: self.anchor, 
            margin: self.margin, 
            color: self.color 
        }
    }
}

impl Default for UiData {
    #[inline]
    fn default() -> Self {
        Self { 
            local_scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            local_rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            local_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            global_scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            global_rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            global_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
        }
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 오브젝트를 생성하는 빌더 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a builder that creates user interface objects. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct UiObjectBuilder<'a> {
    pub name: Option<&'a str>, 
    pub anchor: Anchor, 
    pub margin: Margin, 
    pub color: Vec4, 
    pub local_scale: Vec3,
    pub local_rotation: Quat, 
    pub local_translation: Vec3, 
    pub global_scale: Vec3, 
    pub global_rotation: Quat, 
    pub global_translation: Vec3, 
    pub texture_index: u32, 
    pub tex_sampler: &'a wgpu::Sampler, 
    pub texture_view: &'a wgpu::TextureView, 
    pub ui_brush: &'a UiBrush, 
}

#[allow(dead_code)]
impl<'a> UiObjectBuilder<'a> {
    #[inline]
    pub fn new(
        name: Option<&'a str>, 
        tex_sampler: &'a wgpu::Sampler, 
        texture_view: &'a wgpu::TextureView, 
        ui_brush: &'a UiBrush
    ) -> Self {
        Self {
            name, 
            anchor: Anchor::default(), 
            margin: Margin::default(), 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            local_scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            local_rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            local_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            global_scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            global_rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, 
            global_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            texture_index: 0, 
            tex_sampler, 
            texture_view, 
            ui_brush, 
        }
    }

    #[inline]
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        return self;
    }

    #[inline]
    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        return self;
    }

    #[inline]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        return self;
    }

    #[inline]
    pub fn with_global_scale(mut self, scale: Vec3) -> Self {
        self.global_scale = scale;
        return self;
    }

    #[inline]
    pub fn with_global_rotation(mut self, rotation: Quat) -> Self {
        self.global_rotation = rotation.normalize();
        return self;
    }

    #[inline]
    pub fn with_global_translation(mut self, translation: Vec3) -> Self {
        self.global_translation = translation;
        return self;
    }

    #[inline]
    pub fn with_local_scale(mut self, scale: Vec3) -> Self {
        self.local_scale = scale;
        return self;
    }

    #[inline]
    pub fn with_local_rotation(mut self, rotation: Quat) -> Self {
        self.local_rotation = rotation.normalize();
        return self;
    }

    #[inline]
    pub fn with_local_translation(mut self, translation: Vec3) -> Self {
        self.local_translation = translation;
        return self;
    }

    #[inline]
    pub fn with_texture_index(mut self, texture_index: u32) -> Self {
        self.texture_index = texture_index;
        return self;
    }

    #[inline]
    pub fn build(self, device: &wgpu::Device) -> UiObject {
        UiObject::new(self, device)
    }
}



/// #### 한국어 </br>
/// 사용자 인터페이스 오브젝트 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a user interface object. </br>
/// 
#[derive(Debug)]
pub struct UiObject {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub data: Mutex<UiData>,
}

impl UiObject {
    fn new<'a>(builder: UiObjectBuilder<'a>, device: &wgpu::Device) -> Self {
        use wgpu::util::DeviceExt;

        // (한국어) 라벨 데이터를 생성합니다.
        // (English Translation) Create a label data.
        let label = format!("UiObject({})", builder.name.unwrap_or("Unknown"));

        // (한국어) 사용자 인터페이스 데이터 버퍼를 생성합니다.
        // (English Translation) Create a user interface data buffer.
        let data = UiData {
            local_scale: builder.local_scale, 
            local_rotation: builder.local_rotation, 
            local_translation: builder.local_translation,
            global_scale: builder.global_scale, 
            global_rotation: builder.global_rotation, 
            global_translation: builder.global_translation,
            anchor: builder.anchor, 
            margin: builder.margin, 
            color: builder.color, 
        };
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertex(InstanceData({}))", label)),
                contents: bytemuck::bytes_of(&data.to_instance()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group.
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(&format!("BingGroup(Texture({}))", label)),
                layout: &builder.ui_brush.texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(builder.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(builder.tex_sampler),
                    },
                ],
            }
        );

        Self { 
            buffer, 
            bind_group, 
            data: data.into(), 
        }
    }

    /// #### 한국어 </br>
    /// 사용자 인터페이스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the user interface data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    #[inline]
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F) 
    where F: Fn(&mut MutexGuard<'_, UiData>) {
        let mut guard = self.data.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&guard.to_instance()));
    }

    #[inline]
    fn bind<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    #[inline]
    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        rpass.draw(0..4, 0..1);
    }
}

impl Collider2d<(&PhysicalPosition<f64>, &GameCamera)> for UiObject {
    fn test(&self, other: &(&PhysicalPosition<f64>, &GameCamera)) -> bool {
        let (pos, view, scale) = {
            let guard = other.1.data.lock().expect("Failed to access variable.");
            (other.0, guard.viewport, guard.scale_factor)
        };
        
        let guard = self.data.lock().expect("Failed to access variable.");
        let anchor = guard.anchor;
        let margin = guard.margin;

        let top = view.y + anchor.top() * view.height + margin.top() as f32 * scale;
        let left = view.x + anchor.left() * view.width + margin.left() as f32 * scale;
        let bottom = view.y + anchor.bottom() * view.height + margin.bottom() as f32 * scale;
        let right = view.x + anchor.right() * view.width + margin.right() as f32 * scale;

        let x = pos.x as f32;
        let y = pos.y as f32;

        return left <= x && x <= right
        && bottom <= y && y <= top;
    }
}


#[derive(Debug)]
pub struct UiBrush {
    pipeline: wgpu::RenderPipeline,
    pub texture_layout: wgpu::BindGroupLayout, 
}

impl UiBrush {
    pub fn new(
        device: &wgpu::Device, 
        camera_layout: &wgpu::BindGroupLayout, 
        render_format: wgpu::TextureFormat, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState, 
        multiview: Option<std::num::NonZeroU32>, 
        asset_bundle: &AssetBundle
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        let texture_layout = create_texture_layout(device);
        let bind_group_layouts = &[camera_layout, &texture_layout];
        let pipeline = create_render_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format, 
            depth_stencil, 
            multisample, 
            multiview
        );

        Ok(Self { 
            pipeline,
            texture_layout,
        }.into())
    }

    /// #### 한국어 </br>
    /// 주어진 사용자 인터페이스 오브젝트들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given user interface objects on the screen. </br>
    /// 
    pub fn draw<'pass, Iter>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>, iter: Iter) 
    where Iter: Iterator<Item = &'pass UiObject> {
        rpass.set_pipeline(&self.pipeline);
        for ui in iter {
            ui.bind(rpass);
            ui.draw(rpass);
        }
    }
}


/// #### 한국어 </br>
/// 사용자 인터페이스의 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a shader module for the user interface. </br>
/// 
fn create_shader_module(
    device: &wgpu::Device,
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::UI_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Ui"), device })?;
    asset_bundle.release(path::UI_SHADER_PATH);
    return Ok(module);
}


/// #### 한국어 </br>
/// 사용자 인터페이스 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface bind group layout. </br>
/// 
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(UserInterface)))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { 
                            filterable: true 
                        }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering
                    ),
                    count: None
                },
            ],
        },
    )
}



/// #### 한국어 </br>
/// 사용자 인터페이스 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface rendering pipeline. </br>
/// 
fn create_render_pipeline(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    render_format: wgpu::TextureFormat,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 사용자 인터페이스 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a user interface rendering pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Ui)"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    // (한국어) 사용자 인터페이스 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a user interface rendering pipeline.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Ui)"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<InstanceData>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, local) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, local) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, local) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, local) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, global) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, global) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, global) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: (offset_of!(InstanceData, global) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 8,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(InstanceData, anchor) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 9,
                                format: wgpu::VertexFormat::Sint32x4,
                                offset: offset_of!(InstanceData, margin) as wgpu::BufferAddress,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 10,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: offset_of!(InstanceData, color) as wgpu::BufferAddress,
                            },
                        ]
                    },
                ]
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: "fs_main",
                targets: &[
                    Some(wgpu::ColorTargetState {
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        format: render_format,
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ],
            }),
            multiview
        }
    )
}
