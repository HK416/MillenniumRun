use std::collections::VecDeque;

use glam::{Mat4, Vec4, Vec2};

use crate::{
    assets::bundle::AssetBundle,
    components::{
        map::{self, Table, TileBrush, Tile}, 
        sprite::{InstanceData, Sprite, SpriteBrush}, 
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
pub enum FaceState {
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
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}



#[derive(Debug)]
pub struct Player {
    pub face_timer: f64, 
    pub face_state: FaceState, 
    pub moving_timer: f64, 
    pub control_state: ControlState,

    pub keyboard_pressed: bool, 
    pub life_count: u32, 

    pub spawn: (usize, usize), 
    pub curr: (usize, usize),
    pub next: Option<(usize, usize)>,
    pub path: VecDeque<(usize, usize)>,

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
        tile_set: &Table,
        row: usize,
        col: usize,
        depth: f32,
        size: Vec2
    ) -> AppResult<Self> {
        use crate::nodes::path;

        assert!(row <= tile_set.num_rows && col <= tile_set.num_cols, "The given row and column are out of range!");
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
        let x = map::position(tile_set.origin.x, tile_set.size.x, col);
        let y = map::position(tile_set.origin.y, tile_set.size.y, row);
        let z = depth;
        let instance = vec![
            InstanceData {
                transform: Mat4::from_translation((x, y, z).into()).into(),
                size,
                ..Default::default()
            },
        ];
        let sprite = Sprite::new(
            device, 
            tex_sampler, 
            &texture_view, 
            sprite_brush, 
            instance
        );

        Ok(Self {
            face_timer: 0.0, 
            face_state: FaceState::Idle, 
            moving_timer: 0.0, 
            control_state: ControlState::Idle, 
            keyboard_pressed: false, 
            life_count: 3, 
            spawn: (row, col), 
            curr: (row, col), 
            next: None, 
            path: VecDeque::with_capacity(64), 
            sprite, 
        })
    }
}


/// #### 한국어 </br>
/// 플레이어의 얼굴 상태를 변경합니다. </br>
/// 
/// #### English (Translation) </br>
/// Change the state of the player's face. </br>
/// 
fn change_player_face_state(
    new_state: FaceState, 
    queue: &wgpu::Queue, 
    player: &mut Player
) {
    player.face_timer = 0.0;
    player.face_state = new_state;
    player.sprite.update(queue, |instances| {
        instances[0].texture_index = new_state as u32;
    });
}


const UPDATE_FN: [&'static dyn Fn(f64, &wgpu::Queue, &mut Player); 3] = [
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
    UPDATE_FN[player.face_state as usize](elapsed_time, queue, player)
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
        change_player_face_state(FaceState::Idle, queue, player);
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
        change_player_face_state(FaceState::Idle, queue, player);
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
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    player: &mut Player, 
    queue: &wgpu::Queue
) {
    const DURATION: f64 = 0.05;

    debug_assert!(width > 0.0 && height > 0.0, "The given size must be greater than zero!");

    // (한국어) 플레이어 타이머를 갱신합니다.
    // (English Translation) Updates the player timer.
    player.moving_timer += elapsed_time;

    if let Some(next) = player.next.take() {
        let delta = (player.moving_timer / DURATION).min(1.0) as f32;
        let beg_x = map::position(x, width, player.curr.1);
        let beg_y = map::position(y, height, player.curr.0);
        let end_x = map::position(x, width, next.1);
        let end_y = map::position(y, height, next.0);

        // (한국어) 현재 플레이어의 위치를 계산합니다.
        // (English Translation) Calculates the current player's position.
        let x = beg_x + (end_x - beg_x) * delta;
        let y = beg_y + (end_y - beg_y) * delta;

        player.sprite.update(queue, |instances| {
            let z = instances[0].transform.get_position().z;
            instances[0].transform.set_position((x, y, z).into());
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
pub fn set_player_next_position(max_rows: usize, max_cols: usize, player: &mut Player) {
    if player.next.is_none() {
        player.next = match player.control_state {
            ControlState::Idle => None,
            ControlState::Left => (player.curr.1 > 0).then(|| {
                (player.curr.0, player.curr.1 - 1)
            }),
            ControlState::Right => (player.curr.1 + 1 < max_cols).then(|| {
                (player.curr.0, player.curr.1 + 1)
            }),
            ControlState::Down => (player.curr.0 > 0).then(|| {
                (player.curr.0 - 1, player.curr.1)
            }), 
            ControlState::Up => (player.curr.0 + 1 < max_rows).then(|| {
                (player.curr.0 + 1, player.curr.1)
            })
        }
    }
} 


/// #### 한국어 </br>
/// 현재 플레이어의 위치가 경로에 포함되는지, 닫힌 공간이 만들어 졌는지 확인합니다. </br>
/// 
/// #### English (Translation) </br>
/// Checks whether the current player's positiohn is included in the path </br>
/// or whether an enclosed space has been created. </br>
/// 
pub fn check_current_pos(
    x: f32,
    y: f32,
    width: f32,
    height: f32, 
    max_rows: usize, 
    max_cols: usize,
    line_color: Vec4, 
    edge_color: Vec4, 
    fill_color: Vec4, 
    tile_brush: &TileBrush, 
    tiles: &mut Vec<Vec<Tile>>, 
    player: &mut Player, 
    num_owned_tiles: &mut u32, 
    owned_tiles: &mut VecDeque<(f64, Vec<(usize, usize)>)>, 
    queue: &wgpu::Queue
) {
    if player.next.is_none() {
        if !tiles[player.curr.0][player.curr.1].visited {
            // (한국어) 
            // 현재 타일에 방문하지 않았을 경우 경로에 추가한다.
            // 
            // (English Translation) 
            // If the tile is not currently visited, it is added to the path.
            // 
            tiles[player.curr.0][player.curr.1].visited = true;
            player.path.push_back(player.curr);
            tile_brush.update(queue, |instances| {
                instances[player.curr.0 * max_cols + player.curr.1].color = line_color;
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
                        max_rows, 
                        max_cols, 
                        tiles, 
                        &player.path, 
                    );
                    
                    // (한국어) 선분 영역의 타일들을 구한다.
                    // (English Translation) Finds the tiles in edge area.
                    let mut edge_tiles = search_edge_tiles(
                        max_rows, 
                        max_cols, 
                        tiles,
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
                            instances[r * max_cols + c].color = edge_color;
                        }
                        
                        // (한국어) 안쪽 영역의 타일을 갱신합니다.
                        // (English Translation) Updates the tiles in the inner area.
                        for &(r, c) in inside_tiles.iter() {
                            instances[r * max_cols + c].color = fill_color;
                        }
                    });
                    
                    // (한국어) 타일을 재설정 합니다.
                    // (English Translation) Reset the tile.
                    while let Some((r, c)) = edge_tiles.pop() {
                        tiles[r][c].color = edge_color;
                    }
                    for &(r, c) in inside_tiles.iter() {
                        tiles[r][c].color = fill_color;
                        tiles[r][c].visited = true;
                    }
                    
                    // (한국어) 소유 타일 목록에 추가합니다.
                    // (English Translation) Add it to the list of owned tiles. 
                    *num_owned_tiles += inside_tiles.len() as u32;
                    owned_tiles.push_back((0.0, inside_tiles));

                    if !player.keyboard_pressed {
                        player.control_state = ControlState::Idle;
                    }

                    change_player_face_state(FaceState::Smile, queue, player);
                } 
            } else {
                // (한국어) 
                // 현재 타일에 방문한 적이 있고, 경로에 포함되는 경우:
                // - 경로의 모든 타일을 원래 상태로 복구하고, 경로를 비웁니다.
                // - 플레이어의 라이프 카운트를 감소시킵니다.
                // - 플레이어는 처음 위치에서 스폰됩니다.
                // 
                // (English Translation)
                // If the current tile has been visited and is included in the path:
                // - Restores all tiles in the path to their original state and clears the path.
                // - Decreases the player's life count. 
                // - Players spawn at their initial location. 
                // 
                tile_brush.update(queue, |instances| {
                    for &(r, c) in player.path.iter() {
                        instances[r * max_cols + c].color = tiles[r][c].color;
                    }
                });
                while let Some((r, c)) = player.path.pop_front() {
                    tiles[r][c].visited = false;
                }
                
                player.life_count -= 1;
                if player.life_count == 0 {
                    todo!("Game Over!");
                }
                
                player.curr = player.spawn;
                player.moving_timer = 0.0;
                player.control_state = ControlState::Idle;

                let x = map::position(x, width, player.curr.1);
                let y = map::position(y, height, player.curr.0);
                player.sprite.update(queue, |instance| {
                    let z = instance[0].transform.get_position().z;
                    instance[0].transform.set_position((x, y, z).into());
                });

                change_player_face_state(FaceState::Hit, queue, player);
            }
        }
    }
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
