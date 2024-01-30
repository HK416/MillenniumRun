use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};

use glam::{Mat4, Quat, Vec4, Vec3, Vec2};
use bytemuck::{Pod, Zeroable, offset_of};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        collider2d::{Collider2d, shape::OBB}, 
        table::Table, 
    }, 
    render::shader::WgslDecoder, 
    system::error::AppResult, 
};



/// #### 한국어 </br>
/// 총알 객체를 렌더링하는데 사용되는 정점 입력 데이터 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an vertex input data structure used to render bullet objects. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
struct VertexInput {
    transform: Mat4, 
    color: Vec4, 
    size: Vec2, 
}

impl Default for VertexInput {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Mat4::IDENTITY, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 총알 객체를 렌더링하는데 사용되는 정점 입력 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains vertex input data used for rendering bullet objects. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instance {
    pub speed: f32, 
    pub timer: f64, 
    pub life_time: f64, 
    pub size: Vec2, 
    pub color: Vec4, 
    pub scale: Vec3, 
    pub direction: Vec3, 
    pub translation: Vec3, 
    pub box_size: Vec2, 
}

impl Instance {
    #[inline]
    fn to_data(&self) -> VertexInput {
        VertexInput { 
            transform: Mat4::from_scale_rotation_translation(
                self.scale, 
                Quat::from_rotation_arc(
                    Vec3::X, 
                    self.direction
                ), 
                self.translation
            ), 
            color: self.color, 
            size: self.size 
        }
    }

    #[inline]
    pub fn collider(&self) -> OBB {
        OBB { 
            x: self.translation.x, 
            y: self.translation.y, 
            width: self.box_size.x, 
            height: self.box_size.y, 
            radian: Vec3::X.angle_between(self.direction), 
        }
    }
}

impl Default for Instance {
    #[inline]
    fn default() -> Self {
        Self { 
            speed: 0.0, 
            timer: 0.0, 
            life_time: 0.0, 
            size: Vec2 { x: 0.0, y: 0.0 }, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
            direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            box_size: Vec2 { x: 0.0, y: 0.0 }, 
        }
    }
}


/// #### 한국어 </br>
/// 총알의 데이터 버퍼를 포함하고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the bullet's data buffer. </br>
/// 
#[derive(Debug)]
pub struct Bullet {
    buffer: wgpu::Buffer, 
    bind_group: wgpu::BindGroup, 
    pub instances: Mutex<Vec<Instance>>, 
    capacity: usize, 
}

impl Bullet {
    pub fn with_capacity(
        device: &wgpu::Device, 
        tex_sampler: &wgpu::Sampler, 
        texture_view: &wgpu::TextureView, 
        bullet_brush: &BulletBrush, 
        capacity: usize, 
    ) -> Self {
        // (한국어) 인스턴스 데이터 버퍼를 생성합니다.
        // (English Translation) Create a instance data buffer. 
        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex(InstanceData(Bullet))"), 
                mapped_at_creation: false, 
                size: (size_of::<VertexInput>() * capacity) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
            },
        );

        // (한국어) 텍스처 이미지 바인드 그룹을 생성합니다.
        // (English Translation) Create a texture image bind group. 
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(Texture(Bullet))"), 
                layout: &bullet_brush.texture_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::TextureView(
                            texture_view
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1, 
                        resource: wgpu::BindingResource::Sampler(
                            tex_sampler
                        ),
                    },
                ],
            },
        );

        Self {
            buffer, 
            bind_group, 
            instances: Vec::with_capacity(capacity).into(),
            capacity, 
        }
    }

    /// #### 한국어 </br>
    /// 인터페이스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation) </br>
    /// Updates the interface data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, Vec<Instance>>) {
        let mut guard = self.instances.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        let data: Vec<VertexInput> = guard.iter().map(|it| it.to_data()).collect();
        let length = self.capacity.min(data.len());
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data[0..length]));
    }

    fn draw<'pass>(&'pass self, rpass: &mut wgpu::RenderPass<'pass>) {
        let guard = self.instances.lock().expect("Failed to access variable.");
        let num_instance = self.capacity.min(guard.len());
        if num_instance == 0 {
            return;
        }

        rpass.set_bind_group(1, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.buffer.slice(..));
        rpass.draw(0..4, 0..num_instance as u32);
    }
}



/// #### 한국어 </br>
/// 총알 객체를 그리는 도구입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a tool for drawing bullet objects. </br>
/// 
#[derive(Debug)]
pub struct BulletBrush {
    pipeline: wgpu::RenderPipeline, 
    pub texture_layout: wgpu::BindGroupLayout, 
}

impl BulletBrush {
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
        let pipeline = create_pipeline(
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
            texture_layout
        }.into())
    }

    /// #### 한국어 </br>
    /// 주어진 총알 객체들을 화면에 그립니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Draws the given bullet objects on the screen. </br>
    /// 
    pub fn draw<'pass, I>(
        &'pass self, 
        rpass: &mut wgpu::RenderPass<'pass>, 
        iter: I
    ) where I: Iterator<Item = &'pass Bullet> {
        rpass.set_pipeline(&self.pipeline);
        for bullet in iter {
            bullet.draw(rpass);
        }
    }
}



/// #### 한국어 </br>
/// 쉐이더 파일에서 쉐이더 모듈을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a shader module from the shader file. </br>
/// 
#[inline]
fn create_shader_module(
    device: &wgpu::Device, 
    asset_bundle: &AssetBundle
) -> AppResult<wgpu::ShaderModule> {
    use crate::nodes::path;
    let module = asset_bundle.get(path::BULLET_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("Bullet"), device })?;
    asset_bundle.release(path::BULLET_SHADER_PATH);
    return Ok(module);
}

/// #### 한국어 </br>
/// 텍스처 바인드 그룹 레이아웃을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a texture bind group layout. </br>
/// 
#[inline]
fn create_texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout(Texture(Bullet))"), 
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
                    count: None, 
                },
            ],
        },
    )
}

fn create_pipeline(
    device: &wgpu::Device, 
    module: &wgpu::ShaderModule, 
    bind_group_layouts: &[&wgpu::BindGroupLayout], 
    render_format: wgpu::TextureFormat, 
    depth_stencil: Option<wgpu::DepthStencilState>, 
    multisample: wgpu::MultisampleState, 
    multiview: Option<std::num::NonZeroU32>
) -> wgpu::RenderPipeline {
    // (한국어) 렌더링 파이프라인 레이아웃을 생성합니다.
    // (English Translation) Create a rendering pipeline layout. 
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout(Bullet)"), 
            bind_group_layouts, 
            push_constant_ranges: &[], 
        }
    );

    // (한국어) 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a rendering pipeline.
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Bullet)"), 
            layout: Some(&pipeline_layout), 
            vertex: wgpu::VertexState {
                module, 
                entry_point: "vs_main", 
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<VertexInput>() as wgpu::BufferAddress, 
                        step_mode: wgpu::VertexStepMode::Instance, 
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress, 
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress, 
                            },
                            wgpu::VertexAttribute {
                                shader_location: 2, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress, 
                            },
                            wgpu::VertexAttribute {
                                shader_location: 3, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(VertexInput, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress, 
                            },
                            wgpu::VertexAttribute {
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: offset_of!(VertexInput, color) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x2, 
                                offset: offset_of!(VertexInput, size) as wgpu::BufferAddress, 
                            },
                        ],
                    },
                ],
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
                    }),
                ],
            }),
            multiview, 
        },
    )
}

/// #### 한국어 </br>
/// 총알을 갱신하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the bullets. </br>
/// 
pub fn update_bullets(
    queue: &wgpu::Queue, 
    table: &Table, 
    bullet: &Bullet, 
    elapsed_time: f64
) {
    bullet.update(queue, |instances| {
        let mut next = Vec::with_capacity(instances.capacity());
        while let Some(mut bullet) = instances.pop() {
            // (한국어) 총알의 타이머를 갱신합니다. 
            // (English Translation) Updates the bullet's timer. 
            bullet.timer += elapsed_time;

            // (한국어) 총알이 생명주기를 초과한 경우 건너뜁니다. 
            // (English Translation) If the bullet has exceeded its life cycle, it is skipped. 
            if bullet.timer >= bullet.life_time {
                continue;
            }

            // (한국어) 총알이 타일을 벗어난 경우 건너뜁니다.
            // (English Translation) If the bullet leaves the tile, it is skipped.
            if !table.aabb.test(&bullet.collider()) {
                continue;
            }

            // (한국어) 총알의 위치를 갱신합니다. 
            // (English Translation) Updates the bullet's position. 
            let distance = bullet.direction.normalize() * bullet.speed;
            bullet.translation += distance;

            next.push(bullet);
        }

        instances.append(&mut next);
    })
}