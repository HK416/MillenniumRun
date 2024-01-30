use std::f32::consts::PI;
use std::collections::VecDeque;

use glam::Vec3;

use crate::components::{
    collider2d::shape::AABB, 
    sprite::{Sprite, SpriteBrush, Instance as SpriteData}, 
    table::{self, Table, TileBrush},
    boss::{Boss, BossFaceState},
};

pub const MAX_PLAYER_HEARTS: usize = 5;



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

impl From<usize> for Actor {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Aris, 
            1 => Self::Momoi, 
            2 => Self::Midori, 
            3 => Self::Yuzu,
            _ => panic!("Index out of range!")
        }
    }
}



/// #### 한국어 </br>
/// 플레이어 스프라이트 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of player's sprite states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerFaceState {
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
pub enum PlayerControlState {
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
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}

/// #### 한국어 </br>
/// 플레이어의 게임 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of player's game states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerGameState {
    #[default]
    Empty = 0,
    Invincibility = 1, 
}



/// #### 한국어 </br>
/// 플레이어 데이터를 담고있는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that contains player data. </br>
/// 
#[derive(Debug)]
pub struct Player {
    pub actor: Actor, 

    pub face_timer: f64, 
    pub face_state: PlayerFaceState, 

    pub moving_timer: f64, 
    pub control_state: PlayerControlState,

    pub game_timer: f64,
    pub game_state: PlayerGameState, 

    pub depth: f32, 
    pub curr: (usize, usize),
    pub next: Option<(usize, usize)>,
    pub path: VecDeque<(usize, usize)>,

    pub sprite: Sprite,
}

impl Player {
    pub fn new(
        actor: Actor, 
        row: usize, 
        col: usize, 
        depth: f32, 
        table: &Table, 
        device: &wgpu::Device, 
        tex_sampler: &wgpu::Sampler, 
        texture_view: &wgpu::TextureView, 
        sprite_brush: &SpriteBrush
    ) -> Self {
        let x = table::position(table.origin.x, table.size.x, col);
        let y = table::position(table.origin.y, table.size.y, row);
        let instances = vec![SpriteData {
            scale: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            translation: Vec3 { x, y, z: depth },
            size: table.size * 6.0, 
            ..Default::default()
        }];
        let sprite = Sprite::new(
            device, 
            tex_sampler, 
            texture_view, 
            sprite_brush, 
            instances
        );

        Self { 
            actor, 
            face_timer: 0.0, 
            face_state: PlayerFaceState::default(), 
            moving_timer: 0.0, 
            control_state: PlayerControlState::default(), 
            game_timer: 0.0, 
            game_state: PlayerGameState::default(), 
            depth, 
            curr: (row, col), 
            next: None, 
            path: VecDeque::with_capacity(64), 
            sprite 
        }
    }

    /// #### 한국어 </br>
    /// 플레이어의 충돌체를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the player's collider. </br>
    /// 
    #[inline]
    pub fn collider(&self) -> AABB {
        let instances = self.sprite.instances.lock().expect("Failed to access variable.");
        AABB {
            x: instances[0].translation.x, 
            y: instances[0].translation.y, 
            width: instances[0].size.x, 
            height: instances[0].size.y, 
        }
    }
}


const GAME_UPDATE_FN: [&'static dyn Fn(f64, &wgpu::Queue, &mut Player); 2] = [
    &update_player_empty_state, 
    &update_player_invincibility_state, 
];

/// #### 한국어 </br>
/// 플레이어의 게임 상태를 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the playaer's game state. </br>
/// 
#[inline]
pub fn update_player_game_state(elapsed_time: f64, queue: &wgpu::Queue, player: &mut Player) {
    GAME_UPDATE_FN[player.game_state as usize](elapsed_time, queue, player);
}

#[inline]
fn update_player_empty_state(_elapsed_time: f64, _queue: &wgpu::Queue, _player: &mut Player) {
    /* empty */
}

fn update_player_invincibility_state(elapsed_time: f64, queue: &wgpu::Queue, player: &mut Player) {
    const DURATION: f64 = 3.0;
    
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    player.game_timer += elapsed_time;

    // (한국어) 플레이어 스프라이트의 색상을 갱신합니다.
    // (English Translation) Updates the color of the player sprite. 
    let delta = {
        let t = (player.game_timer / DURATION).min(1.0) as f32;
        0.5 * (36.0 * PI * t).cos() + 0.5
    };
    let c = 0.75 + 0.25 * delta.round();
    player.sprite.update(queue, |instances| {
        instances[0].color = (c, c, c, instances[0].color.w).into();
    });
    
    // (한국어) 지속 시간보다 클 경우 `Empty` 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to `Empty` state. 
    if player.game_timer >= DURATION {
        player.game_timer = 0.0;
        player.game_state = PlayerGameState::Empty;
    }
}




const FACE_UPDATE_FN: [&'static dyn Fn(f64, &wgpu::Queue, &mut Player); 3] = [
    &update_player_idle_face, 
    &update_player_hit_face, 
    &update_player_smile_face, 
];

/// #### 한국어 </br>
/// 플레이어의 얼굴을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the player's face. </br>
/// 
#[inline]
pub fn update_player_face(elapsed_time: f64, queue: &wgpu::Queue, player: &mut Player) {
    FACE_UPDATE_FN[player.face_state as usize](elapsed_time, queue, player)
}


#[inline]
fn update_player_idle_face(_elapsed_time: f64, _queue: &wgpu::Queue, _player: &mut Player) {
    /* empty */
}

#[inline]
fn update_player_smile_face(elapsed_time: f64, queue: &wgpu::Queue, player: &mut Player) {
    const DURATION: f64 = 2.5;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    player.face_timer += elapsed_time;

    // (한국어) 지속 시간보다 클 경우 `Idle` 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to `Idle` state. 
    if player.face_timer >= DURATION {
        player.face_timer = 0.0;
        player.face_state = PlayerFaceState::Idle;
        player.sprite.update(queue, |instances| {
            instances[0].texture_index = PlayerFaceState::Idle as u32;
        });
    }
}

#[inline]
fn update_player_hit_face(elapsed_time: f64, queue: &wgpu::Queue, player: &mut Player) {
    const DURATION: f64 = 2.5;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    player.face_timer += elapsed_time;

    // (한국어) 지속 시간보다 클 경우 `Idle` 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to `Idle` state. 
    if player.face_timer >= DURATION {
        player.face_timer = 0.0;
        player.face_state = PlayerFaceState::Idle;
        player.sprite.update(queue, |instances| {
            instances[0].texture_index = PlayerFaceState::Idle as u32;
        });
    }
}



/// #### 한국어 </br>
/// 플레이어를 다음 위치로 이동시키는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This function moves the player to the next position. </br>
/// 
pub fn translation_player(
    elapsed_time: f64,
    table: &Table, 
    player: &mut Player, 
    queue: &wgpu::Queue
) {
    const DURATION: f64 = 0.05;

    // (한국어) 플레이어 타이머를 갱신합니다.
    // (English Translation) Updates the player timer.
    player.moving_timer += elapsed_time;

    if let Some(next) = player.next.take() {
        let delta = (player.moving_timer / DURATION).min(1.0) as f32;
        let beg_x = table::position(table.origin.x, table.size.x, player.curr.1);
        let beg_y = table::position(table.origin.y, table.size.y, player.curr.0);
        let end_x = table::position(table.origin.x, table.size.x, next.1);
        let end_y = table::position(table.origin.y, table.size.y, next.0);

        // (한국어) 현재 플레이어의 위치를 계산합니다.
        // (English Translation) Calculates the current player's position.
        let x = beg_x + (end_x - beg_x) * delta;
        let y = beg_y + (end_y - beg_y) * delta;

        player.sprite.update(queue, |instances| {
            instances[0].translation.x = x;
            instances[0].translation.y = y;
        });

        if player.moving_timer >= DURATION {
            player.moving_timer = 0.0;
            player.curr = next;
            player.next = None;
        } else {
            player.next = Some(next);
        }
    }
}


/// #### 한국어 </br>
/// 플레이어의 다음 위치를 지정하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This function specifies the player's next position. </br>
/// 
pub fn set_player_next_position(table: &Table, player: &mut Player) {
    if player.next.is_none() {
        player.next = match player.control_state {
            PlayerControlState::Idle => None,
            PlayerControlState::Left => (player.curr.1 > 0).then(|| {
                (player.curr.0, player.curr.1 - 1)
            }),
            PlayerControlState::Right => (player.curr.1 + 1 < table.num_cols).then(|| {
                (player.curr.0, player.curr.1 + 1)
            }),
            PlayerControlState::Down => (player.curr.0 > 0).then(|| {
                (player.curr.0 - 1, player.curr.1)
            }), 
            PlayerControlState::Up => (player.curr.0 + 1 < table.num_rows).then(|| {
                (player.curr.0 + 1, player.curr.1)
            })
        }
    }
} 


/// #### 한국어 </br>
/// 현재 플레이어의 위치가 경로에 포함되는지, 닫힌 공간이 만들어 졌는지 확인합니다. </br>
/// 플레이어가 영역을 획득했을 경우 `true`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Checks whether the current player's positiohn is included in the path </br>
/// or whether an enclosed space has been created. </br>
/// Returns `true` if the plyaer has acquired the tiles. </br>
/// 
pub fn check_current_pos(
    table: &mut Table, 
    player: &mut Player, 
    tile_brush: &TileBrush,
    queue: &wgpu::Queue
) -> Option<bool> {
    if player.next.is_none() {
        if !table.tiles[player.curr.0][player.curr.1].visited {
            // (한국어) 현재 타일에 방문하지 않았을 경우 경로에 추가한다.
            // (English Translation) If the tile is not currently visited, it is added to the path.
            table.tiles[player.curr.0][player.curr.1].visited = true;
            player.path.push_back(player.curr);
            tile_brush.update(queue, |instances| {
                instances[player.curr.0 * table.num_cols + player.curr.1].color = table.line_color;
            })
        } else if player.path.back().is_some_and(|&(r, c)| {
            r == player.curr.0 && c == player.curr.1
        }) {
            // (한국어) 플레이어가 타일 위에 멈춰 있는 경우 아무 처리도 하지 않는다.
            // (English Translation) If the player is stopped on a tile, no action is taken.
        } else if player.path.len() > 1 && player.path.iter().nth_back(1).is_some_and(|&(r, c)| {
            r == player.curr.0 && c == player.curr.1
        }) {
            // (한국어) 플레이어가 이전 타일에 있는 경우 경로에 마지막으로 추가된 타일을 제거한다.
            // (English Translation) If the player is at the previous tile, remove the last tile added to the path.
            let (row, col) = player.path.pop_back().unwrap();
            table.tiles[row][col].visited = false;
            tile_brush.update(queue, |instances| {
                instances[row * table.num_cols + col].color = table.tiles[row][col].color;
            });
        } else {
            let included = player.path.iter()
                .skip(1)
                .find(|(r, c)| *r == player.curr.0 && *c == player.curr.1)
                .is_some();
            if !included {
                if !player.path.is_empty() {
                    return Some(true);
                } 
            } else {
                return Some(false);
            }
        }
    }

    return None;
}

/// #### 한국어 </br>
/// 경로의 모든 타일을 원래 상태로 복구하고, 경로를 비웁니다. </br>
/// 플레이어를 처음 위치에 스폰합니다. </br>
/// 
/// #### English (Translation) </br>
/// Restores all tiles in the path to their original state and clears the path. </br>
/// Players spawn at their initial position. </br>
/// 
pub fn restore(
    queue: &wgpu::Queue, 
    table: &mut Table, 
    boss: &mut Boss, 
    player: &mut Player, 
    tile_brush: &TileBrush
) {
    tile_brush.update(queue, |instances| {
        for &(r, c) in player.path.iter() {
            instances[r * table.num_cols + c].color = table.tiles[r][c].color;
        }
    });
    while let Some((r, c)) = player.path.pop_front() {
        table.tiles[r][c].visited = false;
    }
    
    player.next = None;
    player.curr = table.player_spawn_pos;
    player.moving_timer = 0.0;
    player.control_state = PlayerControlState::Idle;
    player.face_timer = 0.0;
    player.face_state = PlayerFaceState::Hit;
    player.game_timer = 0.0;
    player.game_state = PlayerGameState::Invincibility;
    let x = table::position(table.origin.x, table.size.x, player.curr.1);
    let y = table::position(table.origin.y, table.size.y, player.curr.0);
    
    player.sprite.update(queue, |instances| {
        instances[0].translation.x = x;
        instances[0].translation.y = y;
        instances[0].texture_index = PlayerFaceState::Hit as u32;
    });

    boss.face_timer = 0.0;
    boss.face_state = BossFaceState::Smile;
    boss.sprite.update(queue, |instances| {
        instances[0].texture_index = BossFaceState::Smile as u32;
    });
}
