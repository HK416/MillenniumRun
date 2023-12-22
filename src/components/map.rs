use std::sync::{Arc, Mutex, MutexGuard};
use std::num::NonZeroUsize;

use glam::{Mat4, Vec4, Vec3, Vec2};

use crate::{
    assets::bundle::AssetBundle,
    components::{
        transform::Transform, 
        sprite::{InstanceData, Sprite, SpriteBrush}, 
    },
    nodes::path, 
    render::texture::DdsTextureDecoder,
    system::error::AppResult,
};


/// #### 한국어 </br>
/// 타일 스프라이트의 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of tile's sprite states. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpriteState {
    TopLeft = 0,
    TopMid = 1,
    TopRight = 2,
    MidLeft = 3,
    MidMid = 4,
    MidRight = 5,
    BottomLeft = 6,
    BottomMid = 7,
    BottomRight = 8,
    Center = 9,
}


#[derive(Debug)]
pub struct Tile {
    pub visible: bool,
    pub state: SpriteState,
    pub transform: Transform,
}


#[derive(Debug)]
pub struct TileMap {
    pub sprite: Sprite,

    pub tiles: Vec<Vec<Tile>>,
    pub num_rows: usize, 
    pub num_cols: usize, 

    pub origin: Vec3, 
    pub size: Vec2, 
}

impl TileMap {
    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue,
        tex_sampler: &wgpu::Sampler, 
        sprite_brush: &SpriteBrush, 
        asset_bundle: &AssetBundle, 
        rows: NonZeroUsize, 
        cols: NonZeroUsize, 
        color: Vec4, 
        origin: Vec3, 
        size: Vec2
    ) -> AppResult<Arc<Self>> {
        assert!(size.x > 0.0 && size.y > 0.0, "The given size must be greater than zero!");

        // (한국어) 타일을 생성합니다.
        // (English Translation) Create a tile.
        let mut tiles = Vec::with_capacity(rows.get());
        for row in 0..rows.get() {
            let mut lines = Vec::with_capacity(cols.get());
            for col in 0..cols.get() {
                let x = Self::position(origin.x, size.x, col);
                let y = Self::position(origin.y, size.y, row);
                let z = origin.z;

                lines.push(Tile {
                    visible: true, 
                    state: if row == 0 && col == 0 {
                        SpriteState::BottomLeft
                    } else if row == 0 && col == cols.get() - 1 {
                        SpriteState::BottomRight
                    } else if row == rows.get() - 1 && col == 0 {
                        SpriteState::TopLeft
                    } else if row == rows.get() - 1 && col == cols.get() - 1 {
                        SpriteState::TopRight
                    } else if row == 0 {
                        SpriteState::BottomMid
                    } else if row == rows.get() - 1 {
                        SpriteState::TopMid
                    } else if col == 0 {
                        SpriteState::MidLeft
                    } else if col == cols.get() - 1 {
                        SpriteState::MidRight
                    }else {
                        SpriteState::MidMid
                    },
                    transform: Mat4::from_translation((x, y, z).into()).into()
                })
            }
            tiles.push(lines);
        }


        // (한국어) 타일 텍스처를 생성하고, 텍스처 뷰를 생성합니다.
        // (English Translation) Create a tile texture and create a texture view. 
        let texture = asset_bundle.get(path::TILE_TEXTURE_PATH)?
            .read(&DdsTextureDecoder {
                name: Some("Tile"), 
                size: wgpu::Extent3d {
                    width: 128, 
                    height: 128, 
                    depth_or_array_layers: 10, 
                }, 
                dimension: wgpu::TextureDimension::D2, 
                format: wgpu::TextureFormat::Bgra8Unorm, 
                mip_level_count: 8, 
                sample_count: 1, 
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
                view_formats: &[], 
                device, 
                queue, 
            })?;
        let texture_view = texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: Some("TextureView(Tile)"),
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            },
        );

        // (한국어) 사용한 에셋을 정리합니다.
        // (English Translation) Release used assets.
        asset_bundle.release(path::TILE_TEXTURE_PATH);


        // (한국어) 타일 스프라이트를 생성합니다.
        // (English Translation) Create a tile sprite.
        let data: Vec<InstanceData> = tiles.iter().flatten()
            .map(|tile| InstanceData {
                transform: tile.transform, 
                color, 
                size, 
                texture_index: tile.state as u32, 
            })
            .collect();
        let sprite = Sprite::new(
            device, 
            tex_sampler, 
            &texture_view, 
            sprite_brush, 
            data
        );

        Ok(Self {
            sprite, 
            tiles: tiles.into(), 
            num_rows: rows.get(), 
            num_cols: cols.get(), 
            origin, 
            size, 
        }.into())
    }

    #[inline]
    pub fn position(origin: f32, size: f32, index: usize) -> f32 {
        origin + 0.5 * size + size * index as f32
    }
}