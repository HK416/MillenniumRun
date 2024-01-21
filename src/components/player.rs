use std::f32::consts::PI;
use std::collections::{VecDeque, HashMap};

use glam::{Vec2, Vec3};
use lazy_static::lazy_static;

use crate::{
    components::{
        collider2d::shape::AABB, 
        sprite::{Sprite, SpriteBrush, Instance as SpriteData}, 
        bullet::{self, Bullet, Instance as BulletData}, 
        table::{self, Table, Tile, TileBrush},
    }, 
    nodes::consts::PIXEL_PER_METER, 
};

pub const MAX_PLAYER_HEARTS: usize = 3;
pub const BULLET_LIFE_TIME: f64 = 5.0;

lazy_static! {
    /// #### 한국어 </br>
    /// 발사할 때 총알의 최대 개수입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Maximum number of bullets when firing. </br>
    /// 
    static ref MAX_SHOT_NUM: HashMap<Actor, u32> = HashMap::from_iter([
        (Actor::Aris, 1),
        (Actor::Momoi, 3), 
        (Actor::Midori, 1), 
        (Actor::Yuzu, 1), 
    ]);

    /// #### 한국어 </br>
    /// 다음 총알이 발포될 때까지의 시간입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is the time until the next bullet is fired. </br>
    /// 
    static ref SHOT_TIME: HashMap<Actor, f64> = HashMap::from_iter([
        (Actor::Aris, 0.0), 
        (Actor::Momoi, 0.1), 
        (Actor::Midori, 0.0), 
        (Actor::Yuzu, 0.0), 
    ]);

    /// #### 한국어 </br>
    /// 다음 마우스 입력을 받을 때까지의 시간입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is the time until the next mouse input is received. </br>
    /// 
    static ref COOL_TIME: HashMap<Actor, f64> = HashMap::from_iter([
        (Actor::Aris, 2.25), 
        (Actor::Momoi, 1.0), 
        (Actor::Midori, 0.7), 
        (Actor::Yuzu, 1.75), 
    ]);

    /// #### 한국어 </br>
    /// 발사되는 총알의 속도 입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is the speed of the fired bullet. </br>
    /// 
    static ref BULLET_SPEED: HashMap<Actor, f32> = HashMap::from_iter([
        (Actor::Aris, 0.5 * PIXEL_PER_METER), 
        (Actor::Momoi, 0.7 * PIXEL_PER_METER), 
        (Actor::Midori, 0.7 * PIXEL_PER_METER), 
        (Actor::Yuzu, 0.6 * PIXEL_PER_METER), 
    ]);

    /// #### 한국어 </br>
    /// 발사되는 총알의 크기입니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// This is the size of the fired bullet. </br>
    /// 
    static ref BULLET_SIZE: HashMap<Actor, Vec2> = HashMap::from_iter([
        (Actor::Aris, Vec2::new(8.0, 4.0) * PIXEL_PER_METER), 
        (Actor::Momoi, Vec2::new(1.5, 1.5) * PIXEL_PER_METER), 
        (Actor::Midori, Vec2::new(1.5, 1.5) * PIXEL_PER_METER),
        (Actor::Yuzu, Vec2::new(2.0, 2.0) * PIXEL_PER_METER),  
    ]);

    static ref COLLIDE_OFFSET: HashMap<Actor, Vec2> = HashMap::from_iter([
        (Actor::Aris, Vec2::new(0.0, 0.0)), 
        (Actor::Momoi, Vec2::new(0.0, 0.0)), 
        (Actor::Midori, Vec2::new(0.0, 0.0)), 
        (Actor::Yuzu, Vec2::new(0.0, 0.0)), 
    ]);

    static ref COLLIDE_SIZE: HashMap<Actor, Vec2> = HashMap::from_iter([
        (Actor::Aris, Vec2::new(1.0, 1.0)), 
        (Actor::Momoi, Vec2::new(1.0, 1.0)), 
        (Actor::Midori, Vec2::new(1.0, 1.0)), 
        (Actor::Yuzu, Vec2::new(1.0, 1.0)), 
    ]);
}



/// #### 한국어 </br>
/// 사용자가 선택 가능한 캐릭터의 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of characters that the user can select. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Actor {
    Aris = 0,
    #[default]
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
    actor: Actor, 

    pub shot_count: u32, 
    pub shot_time: f64,
    pub shot_cool_time: f64, 

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
            shot_count: MAX_SHOT_NUM[&actor], 
            shot_time: SHOT_TIME[&actor], 
            shot_cool_time: COOL_TIME[&actor], 
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
    /// 총알을 발사합니다. 발사에 성공할 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Fires a bullet. Returns `true` if the fire is successful. </br>
    /// 
    pub fn try_fire(&mut self) -> bool {
        if self.shot_cool_time >= COOL_TIME[&self.actor] {
            self.shot_count = 0;
            self.shot_time = 0.0;
            self.shot_cool_time = 0.0;
            return true;
        }
        return false;
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
    keyboard_pressed: bool, 
    num_owned_tiles: &mut u32, 
    owned_tiles: &mut VecDeque<(f64, Vec<(usize, usize)>)>, 
    tile_brush: &TileBrush,
    queue: &wgpu::Queue
) -> Option<bool> {
    if player.next.is_none() {
        if !table.tiles[player.curr.0][player.curr.1].visited {
            // (한국어) 
            // 현재 타일에 방문하지 않았을 경우 경로에 추가한다.
            // 
            // (English Translation) 
            // If the tile is not currently visited, it is added to the path.
            // 
            table.tiles[player.curr.0][player.curr.1].visited = true;
            player.path.push_back(player.curr);
            tile_brush.update(queue, |instances| {
                instances[player.curr.0 * table.num_cols + player.curr.1].color = table.line_color;
            })
        } else if player.path.back().is_some_and(|&(r, c)| {
            r == player.curr.0 && c == player.curr.1
        }) {
            // (한국어) 
            // 플레이어가 이전 타일 위에 있는 경우 아무 처리도 하지 않는다.
            // 
            // (English Translation) 
            // If the player is on the previous tile, no action is taken.
            // 
        } else {
            // (한국어) 
            // 현재 타일에 방문한 적이 있고, 경로에 포함되지 않은 경우:
            // - 경로가 비어있지 않으면 타일 탐색 시작.
            // - 경로에 포함된 경우, 플레이어의 라이프 카운트를 감소시키고, 원래대로 되돌리기.
            // 
            // (English Translation) 
            // If the current tile has been visited and is not included in the path:
            // - If the path is not empty, start searching for tiles.
            // - If included in the path, decreases the player's life count 
            //   and returns it to its original state. 
            // 
            let included = player.path.iter()
                .skip(1)
                .find(|(r, c)| *r == player.curr.0 && *c == player.curr.1)
                .is_some();
            if !included {
                if !player.path.is_empty() {
                    // (한국어) 안쪽 영역의 타일들을 구한다.
                    // (English Translation) Finds the tiles in the inner area. 
                    let mut inside_tiles = search_inside_tiles(
                        table.num_rows, 
                        table.num_cols, 
                        &table.tiles, 
                        &player.path, 
                    );
                    
                    // (한국어) 선분 영역의 타일들을 구한다.
                    // (English Translation) Finds the tiles in edge area.
                    let mut edge_tiles = search_edge_tiles(
                        table.num_rows, 
                        table.num_cols, 
                        &table.tiles,
                        &player.path, 
                        &inside_tiles
                    );
                    
                    // (한국어) 안쪽 영역 타일에 경로를 포함시킵니다.
                    // (English Translation) Include the path in the inner area tile.
                    while let Some(path) = player.path.pop_front() {
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

                    // (한국어) 키보드 입력이 없는 경우 플레이어 조작 상태를 `Idle`로 변경합니다.
                    // (English Translation) If there is no keyboard input, change the player control state to `Idle`.
                    if !keyboard_pressed {
                        player.control_state = PlayerControlState::Idle;
                    }

                    // (한국어) 플레이어의 표정을 웃는 표정으로 변경합니다.
                    // (English Translation) Changes the player's face to a smiley face. 
                    player.face_timer = 0.0;
                    player.face_state = PlayerFaceState::Smile;
                    player.sprite.update(queue, |instances| {
                        instances[0].texture_index = PlayerFaceState::Smile as u32;
                    });

                    return Some(true);
                } 
            } else {
                // (한국어) 
                // 현재 타일에 방문한 적이 있고, 경로에 포함되는 경우:
                // - 경로의 모든 타일을 원래 상태로 복구하고, 경로를 비웁니다.
                // - 플레이어는 처음 위치에서 스폰됩니다.
                // 
                // (English Translation)
                // If the current tile has been visited and is included in the path:
                // - Restores all tiles in the path to their original state and clears the path.
                // - Players spawn at their initial location. 
                // 
                restore(queue, table, player, tile_brush);
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

    let x = table::position(table.origin.x, table.size.x, player.curr.1);
    let y = table::position(table.origin.y, table.size.y, player.curr.0);
    player.sprite.update(queue, |instances| {
        instances[0].translation.x = x;
        instances[0].translation.y = y;
    });

    // (한국어) 플레이어의 얼굴을 `Hit` 얼굴로 변경합니다.
    // (English Translation) Changes the player's face to `Hit` face. 
    player.face_timer = 0.0;
    player.face_state = PlayerFaceState::Hit;
    player.sprite.update(queue, |instances| {
        instances[0].texture_index = PlayerFaceState::Hit as u32;
    });

    // (한국어) 플레이어의 게임 상태를 `Invincibility`로 변경합니다.
    // (English Translation) Changes the player's game state to `Invincibility` state. 
    player.game_timer = 0.0;
    player.game_state = PlayerGameState::Invincibility;
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

pub fn update_player_bullet(
    queue: &wgpu::Queue, 
    table: &Table,
    bullet: &Bullet, 
    player: &mut Player, 
    elapsed_time: f64, 
    cursor_pos_world: Vec2
) {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    player.shot_time += elapsed_time;
    player.shot_cool_time += elapsed_time;

    // (한국어) 플레이어의 총알을 추가해야 하는지 확인합니다.
    // (English Translation) Check if the player's bullets need to be added. 
    if player.shot_count < MAX_SHOT_NUM[&player.actor] 
    && player.shot_time >= SHOT_TIME[&player.actor] {
        let mut instances = bullet.instances.lock().expect("Failed to access variable.");
        instances.push(create_bullet(table, player, cursor_pos_world));

        player.shot_count += 1;
        player.shot_time -= SHOT_TIME[&player.actor];
    } 

    // (한국어) 총알을 갱신합니다.
    // (English Translation) Updates the bullets.
    bullet::update_bullets(queue, table, bullet, elapsed_time);
}

fn create_bullet(
    table: &Table,
    player: &Player, 
    cursor_pos_world: Vec2
) -> BulletData {
    let x = table::position(table.origin.x, table.size.x, player.curr.1);
    let y = table::position(table.origin.y, table.size.y, player.curr.0);
    let dir = (cursor_pos_world - Vec2::new(x, y)).normalize();
    let direction = Vec3::new(dir.x, dir.y, 0.0);
    let translation = Vec3::new(x, y, player.depth);
    BulletData {
        speed: BULLET_SPEED[&player.actor], 
        life_time: BULLET_LIFE_TIME, 
        size: BULLET_SIZE[&player.actor], 
        direction, 
        translation, 
        box_offset: COLLIDE_OFFSET[&player.actor], 
        box_size: COLLIDE_SIZE[&player.actor], 
        ..Default::default()
    }
}
