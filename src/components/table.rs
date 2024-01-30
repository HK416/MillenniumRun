use std::mem::size_of;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::VecDeque;

use rand::seq::SliceRandom;
use glam::{Mat4, Vec4, Vec3, Vec2};
use bytemuck::{Pod, Zeroable, offset_of};

use crate::{
    assets::bundle::AssetBundle,
    components::{
        collider2d::shape::AABB, 
        transform::Transform, 
    },
    render::shader::WgslDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 타일의 인스턴스 데이터를 담고있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains the instance data of the tile. </br>
/// 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Transform, 
    pub texcoord_top: f32, 
    pub texcoord_left: f32, 
    pub texcoord_bottom: f32, 
    pub texcoord_right: f32, 
    pub color: Vec4, 
    pub size: Vec2, 
}

impl Default for InstanceData {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: Transform::default(), 
            texcoord_top: 0.0, 
            texcoord_left: 0.0, 
            texcoord_bottom: 1.0, 
            texcoord_right: 1.0, 
            color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, 
            size: Vec2 { x: 0.0, y: 0.0 } 
        }
    }
}



/// #### 한국어 </br>
/// 타일 스프라이트를 그리는 도구 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a tool for drawing tile sprites. </br> 
/// 
#[derive(Debug)]
pub struct TileBrush {
    pipeline: wgpu::RenderPipeline, 
    instance_buffer: wgpu::Buffer, 
    pub instances: Mutex<Vec<InstanceData>>, 
}

impl TileBrush {
    pub fn new(
        device: &wgpu::Device, 
        camera_layout: &wgpu::BindGroupLayout, 
        render_format: wgpu::TextureFormat, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState, 
        multiview: Option<std::num::NonZeroU32>, 
        asset_bundle: &AssetBundle, 
        capacity: usize, 
    ) -> AppResult<Arc<Self>> {
        let module = create_shader_module(device, asset_bundle)?;
        let bind_group_layouts = &[camera_layout];
        let pipeline = create_pipeline(
            device, 
            &module, 
            bind_group_layouts, 
            render_format, 
            depth_stencil, 
            multisample, 
            multiview
        );

        let instances = vec![InstanceData::default(); capacity];
        let instance_buffer = create_instance_buffer(device, &instances);

        Ok(Self { 
            pipeline, 
            instance_buffer, 
            instances: instances.into() 
        }.into())
    }

    /// #### 한국어 </br>
    /// 인스턴스 데이터 버퍼를 갱신합니다. </br>
    /// 버퍼의 내용이 바로 갱신되지 않습니다. (상세: [wgpu::Queue]) </br>
    /// 
    /// #### English (Translation)
    /// Updates the instance data buffer. </br>
    /// The contents of the buffer are not updated immediately. (see also: [wgpu::Queue]) </br>
    /// 
    pub fn update<F>(&self, queue: &wgpu::Queue, mapping_func: F)
    where F: Fn(&mut MutexGuard<'_, Vec<InstanceData>>) {
        let mut guard = self.instances.lock().expect("Failed to access variable.");
        mapping_func(&mut guard);
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&*guard));
    }

    #[inline]
    pub fn draw<'pass>(
        &'pass self, 
        rpass: &mut wgpu::RenderPass<'pass>
    ) {
        let guard = self.instances.lock().expect("Failed to access variable.");
        let num_instances = guard.len() as u32;
        if num_instances == 0 {
            return;
        }

        rpass.set_pipeline(&self.pipeline);
        rpass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        rpass.draw(0..4, 0..num_instances);
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
    let module = asset_bundle.get(path::TILE_SPRITE_SHADER_PATH)?
        .read(&WgslDecoder { name: Some("TileSprite"), device })?;
    asset_bundle.release(path::TILE_SPRITE_SHADER_PATH);
    return Ok(module);
}


/// #### 한국어 </br>
/// 타일 스프라이트의 렌더링 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a rendering pipeline for tile sprite.
/// 
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
            label: Some("PipelineLayout(Tile)"), 
            bind_group_layouts, 
            push_constant_ranges: &[], 
        }, 
    );

    // (한국어) 렌더링 파이프라인을 생성합니다.
    // (English Translation) Create a rendering pipeline. 
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: Some("RenderPipeline(Tile)"), 
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
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, x_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 1, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, y_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 2, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, z_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute { 
                                shader_location: 3, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: (offset_of!(InstanceData, transform) + offset_of!(Mat4, w_axis)) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 4, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_top) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 5, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_left) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 6, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_bottom) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 7, 
                                format: wgpu::VertexFormat::Float32, 
                                offset: offset_of!(InstanceData, texcoord_right) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 8, 
                                format: wgpu::VertexFormat::Float32x4, 
                                offset: offset_of!(InstanceData, color) as wgpu::BufferAddress, 
                            }, 
                            wgpu::VertexAttribute {
                                shader_location: 9, 
                                format: wgpu::VertexFormat::Float32x2, 
                                offset: offset_of!(InstanceData, size) as wgpu::BufferAddress, 
                            }, 
                        ],
                    },
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, 
                strip_index_format: Some(wgpu::IndexFormat::Uint16), 
                front_face: wgpu::FrontFace::Cw, 
                cull_mode: Some(wgpu::Face::Back), 
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
/// 타일 스프라이트의 인스턴스 버퍼를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a instance buffer for tile sprite. </br>
/// 
#[inline]
fn create_instance_buffer(
    device: &wgpu::Device, 
    instances: &[InstanceData]
) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("VertexBuffer(Instance(TileSprite))"), 
            contents: bytemuck::cast_slice(instances), 
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
        }, 
    )
}



/// #### 한국어 </br>
/// 타일의 데이터를 담고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the data of the tile. </br>
/// 
#[derive(Debug)]
pub struct Tile {
    pub visited: bool, 
    pub color: Vec4, 
    pub transform: Transform, 
}


/// #### 한국어 </br>
/// 타일 집합의 데이터를 담고 있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains the data of the tile set. </br>
/// 
#[derive(Debug)]
pub struct Table {
    pub tiles: Vec<Vec<Tile>>, 
    pub player_spawn_pos: (usize, usize), 
    pub half_spawn_area: usize, 
    pub boss_spawn_pos: (usize, usize), 
    pub num_rows: usize, 
    pub num_cols: usize, 
    pub edge_color: Vec4, 
    pub fill_color: Vec4,
    pub line_color: Vec4,  
    pub origin: Vec3, 
    pub size: Vec2, 
    pub aabb: AABB
}

impl Table {
    pub fn new(
        num_rows: usize, 
        num_cols: usize, 
        half_spawn_area: usize, 
        edge_color: Vec4, 
        fill_color: Vec4, 
        line_color: Vec4, 
        origin: Vec3, 
        size: Vec2, 
        queue: &wgpu::Queue, 
        tile_brush: &TileBrush
    ) -> Self {
        debug_assert!(0 < half_spawn_area, "The given \'spawn_half_area\' must be greater than 0!");
        debug_assert!(num_rows > 8 * half_spawn_area, "The number of rows given must be greater than \'8 * spawn_half_area\'!");
        debug_assert!(num_cols > 8 * half_spawn_area, "The number of cols given must be greater than \'8 * spawn_half_area\'!");
    
        // (한국어) 주어진 위치와 크기로 타일을 생성합니다.
        // (English Translation) Creates a tile with a given position and size. 
        let mut tiles = Vec::with_capacity(num_rows);
        for row in 0..num_rows {
            let mut lines = Vec::with_capacity(num_cols);
            for col in 0..num_cols {
                let x = self::position(origin.x, size.x, col);
                let y = self::position(origin.y, size.y, row);

                lines.push(Tile {
                    visited: false, 
                    transform: Mat4::from_translation(Vec3::new(x, y, origin.y)).into(), 
                    color: if row == 0 || row == num_rows - 1 || col == 0 || col == num_cols - 1 {
                        edge_color
                    } else {
                        fill_color
                    },
                });
            }
            tiles.push(lines);
        }

        // (한국어) 타일의 변경된 내용을 갱신합니다.
        // (English Translation) Updates changes made to the tile. 
        tile_brush.update(queue, |instances| {
            for row in 0..num_rows {
                for col in 0..num_cols {
                    instances[row * num_cols + col].transform = tiles[row][col].transform;
                    instances[row * num_cols + col].color = tiles[row][col].color;
                    instances[row * num_cols + col].size = size;
                }
            }
        });


        // (한국어) 플레이어의 스폰 위치를 설정합니다.
        // (English Translation) Set the player's spawn position. 
        let nr = num_rows / 4;
        let nc = num_cols / 4;
        let mut spawns = vec![
            ((1 * nr, 1 * nc), (3 * nr, 3 * nc)), 
            ((1 * nr, 2 * nc), (3 * nr, 2 * nc)), 
            ((1 * nr, 3 * nc), (3 * nr, 1 * nc)),
            ((2 * nr, 1 * nc), (2 * nr, 3 * nc)), 
            ((2 * nr, 3 * nc), (2 * nr, 1 * nc)),
            ((3 * nr, 1 * nc), (1 * nr, 3 * nc)), 
            ((3 * nr, 2 * nc), (1 * nr, 2 * nc)), 
            ((3 * nr, 3 * nc), (1 * nr, 1 * nc)),
        ];
        spawns.shuffle(&mut rand::thread_rng());
        let (player_spawn_pos, boss_spawn_pos) = spawns.pop().unwrap();


        // (한국어) 타일의 바운드 박스를 생성합니다.
        // (English Translation) Creates a bounding box for the tile.
        let width = num_cols as f32 * size.x;
        let height = num_rows as f32 * size.y;
        let x = origin.x + 0.5 * width;
        let y = origin.y + 0.5 * height;
        let aabb = AABB { x, y, width, height };

        Self { 
            tiles, 
            player_spawn_pos, 
            half_spawn_area, 
            boss_spawn_pos, 
            num_rows, 
            num_cols, 
            edge_color, 
            fill_color, 
            line_color, 
            origin, 
            size, 
            aabb, 
        }
    }
}



/// #### 한국어 </br>
/// 타일의 위치를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Returns the position of the tile. </br>
/// 
#[inline]
pub fn position(pos: f32, size: f32, index: usize) -> f32 {
    pos + 0.5 * size + size * index as f32
}

/// #### 한국어 </br>
/// 플레이어가 소유한 타일을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates tiles owned by the player. </br>
/// 
pub fn update_owned_tiles(
    queue: &wgpu::Queue, 
    tile_brush: &TileBrush, 
    table: &mut Table, 
    path: &mut VecDeque<(usize, usize)>, 
    num_owned_tiles: &mut u32, 
    owned_tiles: &mut VecDeque<(f64, Vec<(usize, usize)>)>, 
) {
    // (한국어) 안쪽 영역의 타일들을 구한다.
    // (English Translation) Finds the tiles in the inner area. 
    let mut inside_tiles = search_inside_tiles(
        table.num_rows, 
        table.num_cols, 
        &table.tiles, 
        &path, 
    );

    // (한국어) 선분 영역의 타일들을 구한다.
    // (English Translation) Finds the tiles in edge area.
    let mut edge_tiles = search_edge_tiles(
        table.num_rows, 
        table.num_cols, 
        &table.tiles,
        &path, 
        &inside_tiles
    );

    // (한국어) 안쪽 영역 타일에 경로를 포함시킵니다.
    // (English Translation) Include the path in the inner area tile.
    while let Some(path) = path.pop_front() {
        inside_tiles.push(path);
    }

    tile_brush.update(queue, |instances| {
        // (한국어) 선분 영역의 타일을 갱신합니다.
        // (English Translation) Updates the tiles in the edge area.
        for &(r, c) in edge_tiles.iter() {
            instances[r * table.num_cols + c].color = table.edge_color;
        }
        
        // (한국어) 안쪽 영역의 타일을 갱신합니다.
        // (English Translation) Updates the tiles in the inner area.
        for &(r, c) in inside_tiles.iter() {
            instances[r * table.num_cols + c].color = table.fill_color;
        }
    });

    // (한국어) 타일을 재설정 합니다.
    // (English Translation) Reset the tile.
    while let Some((r, c)) = edge_tiles.pop() {
        table.tiles[r][c].color = table.edge_color;
    }
    for &(r, c) in inside_tiles.iter() {
        table.tiles[r][c].color = table.fill_color;
        table.tiles[r][c].visited = true;
    }

    // (한국어) 소유 타일 목록에 추가합니다.
    // (English Translation) Add it to the list of owned tiles. 
    *num_owned_tiles += inside_tiles.len() as u32;
    owned_tiles.push_back((0.0, inside_tiles));
}

/// #### 한국어 </br>
/// 선분 안쪽 타일들을 찾는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a function that finds tiles inside a edge. </br>
/// 
fn search_inside_tiles(
    max_rows: usize, 
    max_cols: usize, 
    tiles: &Vec<Vec<Tile>>,
    path: &VecDeque<(usize, usize)>,
) -> Vec<(usize, usize)> {
    // (한국어) 타일의 캐시입니다.
    // (English Translation) This is the cache of tiles.
    let mut inside = vec![vec![false; max_cols]; max_rows];
    let mut outside = vec![vec![false; max_cols]; max_rows];

    for &(r, c) in path.iter() {
        // (한국어) 한 위치의 탐색 가능한 영역을 담습니다.
        // (English Translation) Contains the navigable area of a position. 
        let mut begins = Vec::with_capacity(8);
        if r > 0 && !tiles[r - 1][c].visited { begins.push((r - 1, c)); }
        if r + 1 < max_rows && !tiles[r + 1][c].visited { begins.push((r + 1, c)); }
        if c > 0 && !tiles[r][c - 1].visited { begins.push((r, c - 1)); }
        if c + 1 < max_cols && !tiles[r][c + 1].visited { begins.push((r, c + 1)); }
        if r > 0 && c > 0 && !tiles[r - 1][c - 1].visited { begins.push((r - 1, c - 1)); }
        if r > 0 && c + 1 < max_cols && !tiles[r - 1][c + 1].visited { begins.push((r - 1, c + 1)); }
        if r + 1 < max_rows && c > 0 && !tiles[r + 1][c - 1].visited { begins.push((r + 1, c - 1)); }
        if r + 1 < max_rows && c + 1 < max_cols && !tiles[r + 1][c + 1].visited { begins.push((r + 1, c + 1)); }
        
        // (한국어) 깊이 우선 탐색으로 인접한 영역을 찾습니다.
        // (English Translation) Find adjacent regions using `DFS`.
        'check: while let Some(pos) = begins.pop() {
            let mut is_inside = true;
            let mut stack = VecDeque::with_capacity(max_rows);
            let mut visited = vec![vec![false; max_cols]; max_rows];
            stack.push_back(pos);

            'dfs: while let Some((r, c)) = stack.pop_back() {
                // (한국어) 타일이 캐시에 속해 있는 경우 탐색할 필요가 없음.
                // (English Translation) No need to seek if the tile is included in the cache.
                if outside[r][c] || inside[r][c] {
                    continue 'check;
                }

                // (한국어) 깊이 우선 탐색에서 중복되는 탐색 영역을 제거함.
                // (English Translation) Removal of overlapping search areas in `DFS`.
                if visited[r][c] {
                    continue 'dfs;
                }

                // (한국어) 경계에 속하지 않으므로 외부 영역에 해당함.
                // (English Translation) Since it does not belong to the boundary, it is an external area.
                if r == 0 || r + 1 == max_rows || c == 0 || c + 1 == max_cols {
                    is_inside = false;
                }

                visited[r][c] = true;

                if r > 0 && !tiles[r - 1][c].visited && !visited[r - 1][c] { 
                    stack.push_back((r - 1, c)); 
                }

                if r + 1 < max_rows && !tiles[r + 1][c].visited && !visited[r + 1][c] { 
                    stack.push_back((r + 1, c)); 
                }

                if c > 0 && !tiles[r][c - 1].visited && !visited[r][c - 1] { 
                    stack.push_back((r, c - 1));
                }

                if c + 1 < max_cols && !tiles[r][c + 1].visited && !visited[r][c + 1] { 
                    stack.push_back((r, c + 1)); 
                }

                if r > 0 && c > 0 
                && !tiles[r - 1][c - 1].visited && !visited[r - 1][c - 1] { 
                    stack.push_back((r - 1, c - 1)); 
                }

                if r > 0 && c + 1 < max_cols 
                && !tiles[r - 1][c + 1].visited && !visited[r - 1][c + 1] { 
                    stack.push_back((r - 1, c + 1)); 
                }

                if r + 1 < max_rows && c > 0 
                && !tiles[r + 1][c - 1].visited && !visited[r + 1][c - 1] { 
                    stack.push_back((r + 1, c - 1)); 
                }

                if r + 1 < max_rows && c + 1 < max_cols 
                && !tiles[r + 1][c + 1].visited && !visited[r + 1][c + 1] { 
                    stack.push_back((r + 1, c + 1)); 
                }
            }

            if is_inside {
                for r in 0..max_rows {
                    for c in 0..max_cols {
                        inside[r][c] |= visited[r][c];
                    }
                }
            } else {
                for r in 0..max_rows {
                    for c in 0..max_cols {
                        outside[r][c] |= visited[r][c];
                    }
                }
            }
        }
    }

    return inside.into_iter()
    .enumerate()
    .map(|(r, rows)| {
        rows.into_iter()
            .enumerate()
            .filter_map(|(c, flag)| {
                flag.then_some((r, c))
            })
            .collect::<Vec<_>>()
    })
    .flatten()
    .collect();
}

/// #### 한국어 </br>
/// 선분 타일들을 찾는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a function that finds edge tiles. </br>
/// 
fn search_edge_tiles(
    max_rows: usize, 
    max_cols: usize, 
    tiles: &Vec<Vec<Tile>>,
    path: &VecDeque<(usize, usize)>,
    inside_tiles: &Vec<(usize, usize)>, 
) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; max_cols]; max_rows];
    for &(r, c) in inside_tiles.iter() {
        visited[r][c] = true;
    }
    for &(r, c) in path.iter() {
        visited[r][c] = true;
    }

    let mut edge_tiles = Vec::with_capacity(path.len() * 2);
    for &(r, c) in path.iter() {
        if r > 0 && !visited[r - 1][c] && !tiles[r - 1][c].visited {
            edge_tiles.push((r - 1, c));
            visited[r - 1][c] = true;
        }

        if r + 1 < max_rows && !visited[r + 1][c] && !tiles[r + 1][c].visited {
            edge_tiles.push((r + 1, c));
            visited[r + 1][c] = true;
        }

        if c > 0 && !visited[r][c - 1] && !tiles[r][c - 1].visited {
            edge_tiles.push((r, c - 1));
            visited[r][c - 1] = true;
        }

        if c + 1 < max_cols && !visited[r][c + 1] && !tiles[r][c + 1].visited {
            edge_tiles.push((r, c + 1));
            visited[r][c + 1] = true;
        }

        if r > 0 && c > 0 && !visited[r - 1][c - 1] && !tiles[r - 1][c - 1].visited {
            edge_tiles.push((r - 1, c - 1));
            visited[r - 1][c - 1] = true;
        }

        if r > 0 && c + 1 < max_cols && !visited[r - 1][c + 1] && !tiles[r - 1][c + 1].visited {
            edge_tiles.push((r - 1, c + 1));
            visited[r - 1][c + 1] = true;
        }

        if r + 1 < max_rows && c > 0 && !visited[r + 1][c - 1] && !tiles[r + 1][c - 1].visited {
            edge_tiles.push((r + 1, c - 1));
            visited[r + 1][c - 1] = true;
        }

        if r + 1 < max_rows && c + 1 < max_cols && !visited[r + 1][c + 1] && !tiles[r + 1][c + 1].visited {
            edge_tiles.push((r + 1, c + 1));
            visited[r + 1][c + 1] = true;
        }
    }
    
    return edge_tiles;
}