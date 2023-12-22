use std::sync::Arc;

use glam::{Vec2, Mat4};

use crate::{
    assets::bundle::AssetBundle,
    components::{
        map::TileMap,
        sprite::{InstanceData, Sprite, SpriteBrush}
    },
    render::texture::DdsTextureDecoder,
    system::error::AppResult,
};



/// #### 한국어 </br>
/// 사용자가 선택 가능한 캐릭터의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of characters that the user can select. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Actor {
    #[default]
    Aris = 0,
    Momoi = 1,
    Midori = 2,
    Yuzu = 3,
}


/// #### 한국어 </br>
/// 플레이어 스프라이트 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of player's sprite states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpriteState {
    #[default]
    Idle = 0, 
    Hit = 1, 
    Smile = 2, 
}



/// #### 한국어 </br>
/// 플레이어의 조작 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of player's control states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlState {
    /// #### 한국어 </br>
    /// 사용자가 아무 것도 조작하고 있지 않는 상태 입니다. </br>
    /// 다음 위치로 이동하기 위한 입력을 받습니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a state in which the user is not operating anything. </br>
    /// Receives input to move to the next location. </br>
    /// 
    #[default]
    Idle = 0, 

    /// #### 한국어 </br>
    /// 사용자 입력을 받아 플레이어가 이동하고 있는 상태를 나타냅니다. </br>
    /// 일정 시간 이후 다시 `Idle` 상태로 변경됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is a state in which the player is moveing based on user input. </br>
    /// After a certain period of time, it changes back to `Idle` state. </br>
    /// 
    Move = 1, 
}



#[derive(Debug)]
pub struct Player {
    pub timer: f64,
    pub life_count: u32, 
    pub control_state: ControlState,

    map: Arc<TileMap>,
    row: usize,
    col: usize,

    pub sprite: Sprite,
}

impl Player {
    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        tex_sampler: &wgpu::Sampler, 
        sprite_brush: &SpriteBrush, 
        asset_bundle: &AssetBundle, 
        actor: Actor, 
        map: Arc<TileMap>,
        row: usize,
        col: usize,
        depth: f32,
        size: Vec2
    ) -> AppResult<Self> {
        use crate::nodes::path;

        assert!(row < map.num_rows && col < map.num_cols, "The given row and column are out of range!");
        assert!(size.x > 0.0 && size.y > 0.0, "The given size must be greater than zero!");

        let rel_path = match actor {
            Actor::Aris => path::ARIS_PLAYER_TEXTURE_PATH,
            Actor::Momoi => path::MOMOI_PLAYER_TEXTURE_PATH,
            Actor::Midori => path::MIDORI_PLAYER_TEXTURE_PATH,
            Actor::Yuzu => path::YUZU_PLAYER_TEXTURE_PATH
        };

        // (한국어) 텍스처를 생성하고, 텍스처 뷰를 생성합니다.
        // (English Translation) Create a texture and create a texture view.
        let texture = asset_bundle.get(rel_path)?
            .read(&DdsTextureDecoder {
                name: Some("Player"),
                size: wgpu::Extent3d {
                    width: 256,
                    height: 256,
                    depth_or_array_layers: 3,
                },
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Bgra8Unorm,
                mip_level_count: 9,
                sample_count: 1,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
                device,
                queue
            })?;
        let texture_view = texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: Some("TextureView(Player)"),
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            }
        );

        // (한국어) 사용한 에셋을 정리합니다.
        // (English Translation) Release used assets.
        asset_bundle.release(rel_path);

        // (한국어) 플레이어 스프라이트를 생성합니다.
        // (English Translation) Create a player sprite.
        let x = TileMap::position(map.origin.x, map.size.x, col);
        let y = TileMap::position(map.origin.y, map.size.y, row);
        let z = depth;
        let instance = vec![InstanceData {
            transform: Mat4::from_translation((x, y, z).into()).into(),
            size,
            ..Default::default()
        }];
        let sprite = Sprite::new(
            device, 
            tex_sampler, 
            &texture_view, 
            sprite_brush, 
            instance
        );

        Ok(Self {
            timer: 0.0, 
            life_count: 3, 
            control_state: ControlState::Idle, 
            map, 
            row, 
            col, 
            sprite, 
        })
    }
}
