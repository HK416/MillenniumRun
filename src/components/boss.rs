use std::thread;
use std::sync::Arc;
use std::f32::consts::PI;

use rodio::OutputStreamHandle;
use rand::{Rng, seq::SliceRandom};
use glam::{Quat, Vec3, Vec3Swizzles, Vec2};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        collider2d::shape::AABB, 
        sprite::{Sprite, SpriteBrush, Instance as SpriteData}, 
        bullet::Instance as BulletData,  
        table::{self, Table},
        user::Settings, 
        sound, 
    }, 
    nodes::{
        path, 
        in_game::InGameScene, 
        consts::PIXEL_PER_METER, 
    }, 
    system::{
        error::AppResult, 
        shared::Shared, 
    }, 
};

const BULLET_LIFE_TIME: f64 = 5.0;
const BULLET_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 2.0 * PIXEL_PER_METER);
const COLLIDE_OFFSET: Vec2 = Vec2::new(0.0, 0.0);
const COLLIDE_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 2.0 * PIXEL_PER_METER);



/// #### 한국어 </br>
/// 보스 스프라이트 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// List of boss's sprite states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BossFaceState {
    #[default]
    Idle = 0, 
    Embarrass = 1, 
    Smile = 2,
}


/// #### 한국어 </br>
/// 보스의 행동 상태 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of the boss's action status. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BossBehaviorState {
    #[default]
    Idle = 0, 
    PrepareRush = 1, 
    Rush = 2, 
    FireBulletPattern0 = 3, 
    FireBulletPattern1 = 4, 
    FireBulletPattern2 = 5,
    WaitForFinish, 
    MoveForFinish, 
}


#[derive(Debug)]
pub struct Boss {
    pub direction: Vec2, 

    pub face_timer: f64, 
    pub face_state: BossFaceState, 

    behavior_count: u32,
    max_behavior_count: u32, 
    behavior_timer: f64,
    behavior_state: BossBehaviorState,
    previous_behavior: Option<BossBehaviorState>, 

    pub sprite: Sprite, 
}

impl Boss {
    pub fn new(
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
            size: table.size * 10.0, 
            ..Default::default()
        }];
        let sprite = Sprite::new(
            device, 
            tex_sampler, 
            texture_view, 
            sprite_brush, 
            instances
        );

        let mut rng = rand::thread_rng();
        let rotation = Quat::from_rotation_z(rng.gen_range(0.0..2.0 * PI));
        let direction = rotation.mul_vec3(Vec3::X).xy().normalize();

        Self { 
            direction, 
            face_timer: 0.0, 
            face_state: BossFaceState::default(), 
            behavior_count: 0, 
            max_behavior_count: 0, 
            behavior_timer: 0.0, 
            behavior_state: BossBehaviorState::default(), 
            previous_behavior: None, 
            sprite, 
        }
    }

    /// #### 한국어 </br>
    /// 보스의 충돌체를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the boss's collider. </br>
    /// 
    #[inline]
    pub fn collider(&self) -> AABB {
        let instances = self.sprite.instances.lock().expect("Failed to access variable.");
        AABB { 
            x: instances[0].translation.x, 
            y: instances[0].translation.y, 
            width: instances[0].size.x, 
            height: instances[0].size.y 
        }
    }
}



type UpdateFn = dyn for<'a> Fn(&'a mut InGameScene, &'a mut Shared, f64, f64) -> AppResult<()>;
const UPDATE_FUNC: [&'static UpdateFn; 8] = [
    &update_boss_idle_state, 
    &update_boss_prepare_rush_state,
    &update_boss_rush_state,
    &update_boss_fire_bullet_pattern0, 
    &update_boss_fire_bullet_pattern1, 
    &update_boss_fire_bullet_pattern2, 
    &update_boss_wait_for_finish, 
    &update_boss_move_for_finish, 
];

pub fn update_boss(this: &mut InGameScene, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
    UPDATE_FUNC[this.boss.behavior_state as usize](this, shared, total_time, elapsed_time)?;
    adjust_boss_position(&this.table, &mut this.boss);
    apply_boss_position(shared.get::<Arc<wgpu::Queue>>().unwrap(), &mut this.boss);
    Ok(())
}


/// #### 한국어 </br>
/// 보스의 행동 상태가 `Idle`일 때 호출되는 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function that is called when the boss's behavior state is `Idle`. </br>
/// 
fn update_boss_idle_state(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 3.0;
    const SPEED: f32 = 5.0 * PIXEL_PER_METER; // meter per sec

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.boss.behavior_timer += elapsed_time;

    // (한국어) 보스의 위치를 갱신합니다.
    // (English Translation) Update the boss's position. 
    let velocity = this.boss.direction * SPEED;
    let distance: Vec3 = (velocity * elapsed_time as f32, 0.0).into();
    let mut instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
    instances[0].translation += distance;

    // (한국어) 지속 시간보다 클 경우 임의의 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to a random state. 
    if this.boss.behavior_timer >= DURATION {
        let mut next_state = vec![
            BossBehaviorState::FireBulletPattern0, 
            BossBehaviorState::FireBulletPattern0, 
            BossBehaviorState::FireBulletPattern0, 
            BossBehaviorState::FireBulletPattern1, 
            BossBehaviorState::FireBulletPattern2, 
            BossBehaviorState::FireBulletPattern2, 
            BossBehaviorState::PrepareRush,
        ];
        next_state.shuffle(&mut rand::thread_rng());

        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared object to use. 
        let stream = shared.get::<OutputStreamHandle>().unwrap();
        let settings = shared.get::<Settings>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();

        match next_state.pop().unwrap() {
            BossBehaviorState::PrepareRush => {
                let source = asset_bundle.get(path::YUUKA_ATTACK1_SOUND_PATH)?
                    .read(&sound::SoundDecoder)?;
                let sink = sound::play_sound(settings.voice_volume, source, stream)?;
                thread::spawn(move || {
                    sink.sleep_until_end();
                    sink.detach();
                });

                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::PrepareRush;
            }, 
            BossBehaviorState::FireBulletPattern0 => {
                let mut rng = rand::thread_rng();
                if rng.gen_ratio(1, 4) {
                    const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                    let rel_path = PATHS[rng.gen_range(0..2)];
                    let source = asset_bundle.get(rel_path)?
                        .read(&sound::SoundDecoder)?;
                    let sink = sound::play_sound(settings.voice_volume, source, stream)?;
                    thread::spawn(move || {
                        sink.sleep_until_end();
                        sink.detach();
                    });
                }

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 8;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern0;
            },
            BossBehaviorState::FireBulletPattern1 => {
                let mut rng = rand::thread_rng();
                if rng.gen_ratio(1, 4) {
                    const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                    let rel_path = PATHS[rng.gen_range(0..2)];
                    let source = asset_bundle.get(rel_path)?
                        .read(&sound::SoundDecoder)?;
                    let sink = sound::play_sound(settings.voice_volume, source, stream)?;
                    thread::spawn(move || {
                        sink.sleep_until_end();
                        sink.detach();
                    });
                }

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 24;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern1;
            }, 
            BossBehaviorState::FireBulletPattern2 => {
                let mut rng = rand::thread_rng();
                if rng.gen_ratio(1, 4) {
                    const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                    let rel_path = PATHS[rng.gen_range(0..2)];
                    let source = asset_bundle.get(rel_path)?
                        .read(&sound::SoundDecoder)?;
                    let sink = sound::play_sound(settings.voice_volume, source, stream)?;
                    thread::spawn(move || {
                        sink.sleep_until_end();
                        sink.detach();
                    });
                }

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 8;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern2;
            },
            _ => { panic!("Invalid patterns!") }
        }
    }

    Ok(())
}

/// #### 한국어 </br>
/// 보스의 행동 상태가 `PrepareRush`일 때 호출되는 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function that is called when the boss's behavior state is `PrepareRush`. </br>
/// 
fn update_boss_prepare_rush_state(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 1.0;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    this.boss.behavior_timer += elapsed_time;

    // (한국어) 지속 시간보다 클 경우 `Rush` 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to `Rush` state. 
    if this.boss.behavior_timer >= DURATION {
        // (한국어) 보스의 현재 위치를 가져옵니다.
        // (English Translation) Get the current position of the boss. 
        let boss_position = {
            let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
            instances[0].translation.xy()
        };

        // (한국어) 플레이어의 현재 위치를 가져옵니다.
        // (English Translation) Get the current position of the player.
        let player_position = {
            let instances = this.player.sprite.instances.lock().expect("Failed to access variable.");
            instances[0].translation.xy()
        };

        // (한국어) 보스의 돌진 방향을 계산합니다.
        // (English Translation) Calculate the boss' charging direction. 
        let direction = (player_position - boss_position).normalize();

        this.boss.direction = direction;
        this.boss.behavior_timer = 0.0;
        this.boss.behavior_state = BossBehaviorState::Rush;
    }

    Ok(())
}

/// #### 한국어 </br>
/// 보스의 행동 상태가 `Rush`일 때 호출되는 업데이트 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an update function that is called when the boss's behavior state is `Rush`. </br>
/// 
fn update_boss_rush_state(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 3.0;
    const SPEED: f32 = 70.0 * PIXEL_PER_METER; // meter per sec;
    
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer. 
    this.boss.behavior_timer += elapsed_time;

    // (한국어) 보스의 위치를 갱신합니다.
    // (English Translation) Updates the boss's position. 
    let delta = rush_speed_interpolation(this.boss.behavior_timer, DURATION) as f32;
    let velocity = this.boss.direction * SPEED * delta;
    let distance: Vec3 = (velocity * elapsed_time as f32, 0.0).into();
    let mut instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
    instances[0].translation += distance;

    // (한국어) 지속 시간보다 클 경우 `Idle` 상태로 변경합니다.
    // (English Translation) If it is greater than the duration, it changes to `Idle` state. 
    if this.boss.behavior_timer >= DURATION {
        this.boss.behavior_timer = 0.0;
        this.boss.behavior_state = BossBehaviorState::Idle;
    }

    Ok(())
}

fn update_boss_fire_bullet_pattern0(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 0.5 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();    
    let source = asset_bundle.get(path::YUUKA_FIRE_SOUND_PATH)?
        .read(&sound::SoundDecoder)?;
    let sink = sound::play_sound(settings.effect_volume, source, stream)?;
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });

    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 8;
    let mut angle = if this.boss.behavior_count % 2 == 0 { 0.0 * PI } else { 0.1666666667 * PI };
    while count > 0 {
        let rotation = Quat::from_rotation_z(angle);
        let direction = rotation.mul_vec3(Vec3::X);
        instances.push(BulletData {
            speed: BULLET_SPEED, 
            life_time: BULLET_LIFE_TIME, 
            direction, 
            translation, 
            size: BULLET_SIZE, 
            box_offset: COLLIDE_OFFSET, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        angle += 0.25 * PI;
        count -= 1;
    }

    // (한국어) 행동 카운트를 증가시킵니다.
    // (English Translation) Increases behavior count. 
    this.boss.behavior_count += 1;

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state. 
    if this.boss.behavior_count >= this.boss.max_behavior_count {
        this.boss.behavior_count = 0;
        this.boss.max_behavior_count = 0;
        this.boss.behavior_timer = 0.25;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = None;
    } else {
        this.boss.behavior_timer = 0.25;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = Some(BossBehaviorState::FireBulletPattern0);
    }

    Ok(())
}

fn update_boss_fire_bullet_pattern1(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 0.75 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();    
    let source = asset_bundle.get(path::YUUKA_FIRE_SOUND_PATH)?
        .read(&sound::SoundDecoder)?;
    let sink = sound::play_sound(settings.effect_volume, source, stream)?;
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });

    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let dist = {
        let instances = this.player.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };
    let origin = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    instances.push(BulletData {
        speed: BULLET_SPEED, 
        life_time: BULLET_LIFE_TIME, 
        direction: (dist - origin).normalize(), 
        translation: origin, 
        size: BULLET_SIZE, 
        box_offset: COLLIDE_OFFSET, 
        box_size: COLLIDE_SIZE,
        ..Default::default() 
    });

    // (한국어) 행동 카운트를 증가시킵니다.
    // (English Translation) Increases behavior count. 
    this.boss.behavior_count += 1;

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state. 
    if this.boss.behavior_count >= this.boss.max_behavior_count {
        this.boss.behavior_count = 0;
        this.boss.max_behavior_count = 0;
        this.boss.behavior_timer = 0.5;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = None;
    } else {
        this.boss.behavior_timer = 0.1;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = Some(BossBehaviorState::FireBulletPattern1);
    }

    Ok(())
}

fn update_boss_fire_bullet_pattern2(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 0.5 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    let stream = shared.get::<OutputStreamHandle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();    
    let source = asset_bundle.get(path::YUUKA_FIRE_SOUND_PATH)?
        .read(&sound::SoundDecoder)?;
    let sink = sound::play_sound(settings.effect_volume, source, stream)?;
    thread::spawn(move || {
        sink.sleep_until_end();
        sink.detach();
    });
    
    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 8;
    let mut angle = if this.boss.behavior_count % 2 == 0 { 0.0 * PI } else { 0.1666666667 * PI };
    while count > 0 {
        let rotation = Quat::from_rotation_z(angle);
        let direction = rotation.mul_vec3(Vec3::X);
        instances.push(BulletData {
            speed: BULLET_SPEED, 
            life_time: BULLET_LIFE_TIME, 
            direction, 
            translation, 
            size: BULLET_SIZE, 
            box_offset: COLLIDE_OFFSET, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        angle += 0.25 * PI;
        count -= 1;
    }

    // (한국어) 행동 카운트를 증가시킵니다.
    // (English Translation) Increases behavior count. 
    this.boss.behavior_count += 1;

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state. 
    if this.boss.behavior_count >= this.boss.max_behavior_count {
        this.boss.behavior_count = 0;
        this.boss.max_behavior_count = 0;
        this.boss.behavior_timer = 0.25;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = None;
    } else {
        this.boss.behavior_timer = 0.25;
        this.boss.behavior_state = BossBehaviorState::MoveForFinish;
        this.boss.previous_behavior = Some(BossBehaviorState::FireBulletPattern2);
    }

    Ok(())
}

fn update_boss_wait_for_finish(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.boss.behavior_timer -= elapsed_time;

    // (한국어) 타이머가 0보다 작을 경우 다음 상태로 변경합니다.
    // (English Translation) If the timer is less than 0, it changes to the next state. 
    if this.boss.behavior_timer <= 0.0 {
        if let Some(behavior_state) = this.boss.previous_behavior.take() {
            this.boss.behavior_timer = 0.0;
            this.boss.behavior_state = behavior_state;
        } else {
            this.boss.behavior_timer = 0.0;
            this.boss.behavior_state = BossBehaviorState::Idle;
        }
    }

    Ok(())
}

fn update_boss_move_for_finish(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const SPEED: f32 = 5.0 * PIXEL_PER_METER; // meter per sec

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.boss.behavior_timer -= elapsed_time;

    // (한국어) 보스의 위치를 갱신합니다.
    // (English Translation) Update the boss's position. 
    let velocity = this.boss.direction * SPEED;
    let distance: Vec3 = (velocity * elapsed_time as f32, 0.0).into();
    let mut instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
    instances[0].translation += distance;

    // (한국어) 타이머가 0보다 작을 경우 다음 상태로 변경합니다.
    // (English Translation) If the timer is less than 0, it changes to the next state. 
    if this.boss.behavior_timer <= 0.0 {
        if let Some(behavior_state) = this.boss.previous_behavior.take() {
            this.boss.behavior_timer = 0.0;
            this.boss.behavior_state = behavior_state;
        } else {
            this.boss.behavior_timer = 0.0;
            this.boss.behavior_state = BossBehaviorState::Idle;
        }
    }

    Ok(())
}

/// #### 한국어 </br>
/// 보스의 위치가 이동 가능한 영역을 벗어나는지 확인합니다. </br> 
/// 보스의 위치가 이동 가능한 영역을 벗어날 경우 위치와 방향을 조정합니다. </br>
/// 
/// English (Translation) </br>
/// Check whether the boss's position is outside the moveable area. </br>
/// If the boss's position is outside the moveable area, 
/// adjustits position and direction. </br>
///
fn adjust_boss_position(table: &Table, boss: &mut Boss) {
    let mut instances = boss.sprite.instances.lock().expect("Failed to access variable.");
    let top = table.aabb.y + 0.5 * table.aabb.height;
    let left = table.aabb.x - 0.5 * table.aabb.width;
    let bottom = table.aabb.y - 0.5 * table.aabb.height;
    let right = table.aabb.x + 0.5 * table.aabb.width;
        
    if top < instances[0].translation.y {
        // case 1: bounding box top < y position
        boss.direction.y = -boss.direction.y;
        
        let delta = instances[0].translation.y - top;
        instances[0].translation.y = top - delta;
    } 
        
    if instances[0].translation.y < bottom {
        // case 2: bounding box bottom > y position 
        boss.direction.y = -boss.direction.y;
        
        let delta = bottom - instances[0].translation.y;
        instances[0].translation.y = bottom + delta;
    } 
        
    if right < instances[0].translation.x {
        // case 3: bounding box right < x position
        boss.direction.x = -boss.direction.x;
        
        let delta = instances[0].translation.x - right;
        instances[0].translation.x = right - delta;
    } 
        
    if instances[0].translation.x < left {
        // case 4: bounding bos left > x position
        boss.direction.x = -boss.direction.x;
        
        let delta = left - instances[0].translation.x;
        instances[0].translation.x = left + delta;
    }
}

/// (한국어) 보스의 변경된 위치를 반영합니다.
/// (English Translation) Applies the changed position of the boss.
#[inline]
fn apply_boss_position(queue: &wgpu::Queue, boss: &mut Boss) {
    boss.sprite.update(queue, |_| { /* empty */ });
}

#[inline]
fn rush_speed_interpolation(val: f64, max: f64) -> f64 {
    debug_assert!(val >= 0.0 && max >= 0.0, "The given values must be greater than or equal to 0!");
    let t = (val / max).clamp(0.0, 1.0);
    return -0.01 * t * t * t + t.powf(-2.0 * t) - 1.0;
}
