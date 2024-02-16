use std::thread;
use std::sync::Arc;
use std::f32::consts::PI;

use glam::{Quat, Vec3, Vec3Swizzles, Vec2};
use rand::{Rng, seq::SliceRandom};
use rodio::{OutputStream, OutputStreamHandle};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        collider2d::shape::AABB, 
        sprite::{Sprite, SpriteBrush, Instance as SpriteData}, 
        bullet::{Accelerator, Instance as BulletData},  
        table::{self, Table},
        user::Settings, 
        sound, 
    }, 
    nodes::{
        path, 
        in_game::{
            NUM_TILES, 
            InGameScene, 
        }, 
        consts::PIXEL_PER_METER, 
    }, 
    system::{
        error::AppResult, 
        shared::Shared, 
    }, 
};

const BULLET_LIFE_TIME: f64 = 5.0;
const BULLET_SIZE: Vec2 = Vec2::new(2.0 * PIXEL_PER_METER, 2.0 * PIXEL_PER_METER);
const COLLIDE_SIZE: Vec2 = Vec2::new(1.0 * PIXEL_PER_METER, 1.0 * PIXEL_PER_METER);



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
    Idle, 
    PrepareRush, 
    Rush, 
    PrepareShotRush,
    ShotRush, 
    FireBulletPattern0, 
    FireBulletPattern1, 
    FireBulletPattern2,
    FireBulletPattern3,
    FireBulletPattern4,
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
const UPDATE_FUNC: [&'static UpdateFn; 12] = [
    &update_boss_idle_state, 
    &update_boss_prepare_rush_state,
    &update_boss_rush_state,
    &update_boss_prepare_shot_rush_state,
    &update_boss_shot_rush_state,
    &update_boss_fire_bullet_pattern0, 
    &update_boss_fire_bullet_pattern1, 
    &update_boss_fire_bullet_pattern2, 
    &update_boss_fire_bullet_pattern3, 
    &update_boss_fire_bullet_pattern4, 
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
    const DURATION: f64 = 2.5;
    const SPEED: f32 = 7.0 * PIXEL_PER_METER; // meter per sec

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
        let mut next_state = if this.remaining_time <= 30.0 
        || this.num_owned_tiles >= NUM_TILES as u32 / 2 
        || this.player.path.len() >= 75 {
            vec![
                BossBehaviorState::FireBulletPattern2, 
                BossBehaviorState::FireBulletPattern3, 
                BossBehaviorState::FireBulletPattern4,
                BossBehaviorState::PrepareShotRush,
                BossBehaviorState::PrepareRush,
            ]
        } else {
            vec![
                BossBehaviorState::FireBulletPattern0, 
                BossBehaviorState::FireBulletPattern1, 
                BossBehaviorState::FireBulletPattern2, 
                BossBehaviorState::PrepareRush,
            ]
        };

        next_state.shuffle(&mut rand::thread_rng());


        match next_state.pop().unwrap() {
            BossBehaviorState::PrepareRush => {
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(path::YUUKA_ATTACK1_SOUND_PATH)?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::PrepareRush;
            }, 
            BossBehaviorState::PrepareShotRush => {
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(path::YUUKA_ATTACK1_SOUND_PATH)?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_timer = 0.0;
                this.boss.max_behavior_count = 3;
                this.boss.behavior_state = BossBehaviorState::PrepareShotRush;
            }
            BossBehaviorState::FireBulletPattern0 => {
                const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(PATHS[rand::thread_rng().gen_range(0..2)])?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 8;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern0;
            },
            BossBehaviorState::FireBulletPattern1 => {
                const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(PATHS[rand::thread_rng().gen_range(0..2)])?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 24;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern1;
            }, 
            BossBehaviorState::FireBulletPattern2 => {
                const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(PATHS[rand::thread_rng().gen_range(0..2)])?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 8;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern2;
            },
            BossBehaviorState::FireBulletPattern3 => {
                const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(PATHS[rand::thread_rng().gen_range(0..2)])?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 24;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern3;
            }, 
            BossBehaviorState::FireBulletPattern4 => {
                const PATHS: [&'static str; 2]  = [path::YUUKA_ATTACK2_SOUND_PATH, path::YUUKA_ATTACK3_SOUND_PATH];
                if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
                    if let Some(sink) = sound::try_new_sink(&stream_handle)? {
                        let settings = shared.get::<Settings>().unwrap();
                        let asset_bundle = shared.get::<AssetBundle>().unwrap();
                        let source = asset_bundle.get(PATHS[rand::thread_rng().gen_range(0..2)])?.read(&sound::SoundDecoder)?;
                        sink.set_volume(settings.voice_volume.norm());
                        sink.append(source);
                        thread::spawn(move || {
                            sink.sleep_until_end();
                        });
                        shared.push((stream, stream_handle));
                    }
                };

                this.boss.behavior_count = 0;
                this.boss.max_behavior_count = 24;
                this.boss.behavior_timer = 0.0;
                this.boss.behavior_state = BossBehaviorState::FireBulletPattern4;
            }
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

fn update_boss_prepare_shot_rush_state(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
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
        this.boss.behavior_state = BossBehaviorState::ShotRush;
    }

    Ok(())
}

fn update_boss_shot_rush_state(this: &mut InGameScene, _shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 1.5;
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

    // (한국어) 다음 상태로 변경합니다.
    // (English Translation) Changes to the next state. 
    if this.boss.behavior_timer >= DURATION {
        this.boss.behavior_count += 1;
        if this.boss.behavior_count >= this.boss.max_behavior_count {
            this.boss.behavior_count = 0;
            this.boss.max_behavior_count = 0;
            this.boss.behavior_timer = 0.25;
            this.boss.behavior_state = BossBehaviorState::WaitForFinish;
            this.boss.previous_behavior = None;
        } else {
            this.boss.behavior_timer = 0.0;
            this.boss.behavior_state = BossBehaviorState::PrepareShotRush;
            this.boss.previous_behavior = None;
        }
    }

    Ok(())
}

fn update_boss_fire_bullet_pattern0(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 30.0 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
            let settings = shared.get::<Settings>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(path::BULLET_FIRE_SOUND_PATH)?.read(&sound::SoundDecoder)?;
            sink.set_volume(settings.effect_volume.norm());
            sink.append(source);
            thread::spawn(move || {
                sink.sleep_until_end();
            });
            shared.push((stream, stream_handle));
        }
    }

    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 8;
    let mut degree: f32 = if this.boss.behavior_count % 2 == 0 { 0.0 } else { 22.5 };
    while count > 0 {
        let rotation = Quat::from_rotation_z(degree.to_radians());
        let direction = rotation.mul_vec3(Vec3::X).normalize_or_zero().xy();
        instances.push(BulletData {
            velocity: direction * BULLET_SPEED, 
            life_time: BULLET_LIFE_TIME, 
            translation, 
            size: BULLET_SIZE, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        degree += 45.0;
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
    const BULLET_SPEED: f32 = 40.0 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
            let settings = shared.get::<Settings>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(path::BULLET_FIRE_SOUND_PATH)?.read(&sound::SoundDecoder)?;
            sink.set_volume(settings.effect_volume.norm());
            sink.append(source);
            thread::spawn(move || {
                sink.sleep_until_end();
            });
            shared.push((stream, stream_handle));
        }
    }

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

    let direction = (dist - origin).normalize_or_zero().xy();
    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    instances.push(BulletData {
        velocity: direction * BULLET_SPEED,
        life_time: BULLET_LIFE_TIME, 
        translation: origin, 
        size: BULLET_SIZE, 
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
    const BULLET_SPEED: f32 = 30.0 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
            let settings = shared.get::<Settings>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(path::BULLET_FIRE_SOUND_PATH)?.read(&sound::SoundDecoder)?;
            sink.set_volume(settings.effect_volume.norm());
            sink.append(source);
            thread::spawn(move || {
                sink.sleep_until_end();
            });
            shared.push((stream, stream_handle));
        }
    }
    
    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 8;
    let mut degree: f32 = if this.boss.behavior_count % 2 == 0 { 0.0 } else { 22.5 };
    while count > 0 {
        let rotation = Quat::from_rotation_z(degree.to_radians());
        let direction = rotation.mul_vec3(Vec3::X).normalize_or_zero().xy();
        instances.push(BulletData {
            velocity: direction * BULLET_SPEED, 
            accelerator: None,
            life_time: BULLET_LIFE_TIME, 
            translation, 
            size: BULLET_SIZE, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        degree += 45.0;
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

#[derive(Debug)]
pub struct Curve;

impl Accelerator for Curve {
    fn value(&self, velocity: Vec2, _timer: f64) -> Vec2 {
        const VALUE: f32 = 500.0; // meter per sec^2
        let forward: Vec3 = (velocity, 0.0).into();
        let right = Vec3::Z.cross(forward.normalize()).xy();
        right * VALUE
    }
}

fn update_boss_fire_bullet_pattern3(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 30.0 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
            let settings = shared.get::<Settings>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(path::BULLET_FIRE_SOUND_PATH)?.read(&sound::SoundDecoder)?;
            sink.set_volume(settings.effect_volume.norm());
            sink.append(source);
            thread::spawn(move || {
                sink.sleep_until_end();
            });
            shared.push((stream, stream_handle));
        }
    }
    
    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 8;
    let mut degree: f32 = if this.boss.behavior_count % 2 == 0 { 0.0 } else { 22.5 };
    while count > 0 {
        let rotation = Quat::from_rotation_z(degree.to_radians());
        let direction = rotation.mul_vec3(Vec3::X).normalize().xy();
        instances.push(BulletData {
            velocity: direction * BULLET_SPEED, 
            accelerator: Some(Arc::new(Curve)), 
            life_time: BULLET_LIFE_TIME, 
            translation, 
            size: BULLET_SIZE, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        degree += 45.0;
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
        this.boss.behavior_timer = 0.05;
        this.boss.behavior_state = BossBehaviorState::MoveForFinish;
        this.boss.previous_behavior = Some(BossBehaviorState::FireBulletPattern3);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Transmission;

impl Accelerator for Transmission {
    fn value(&self, velocity: Vec2, timer: f64) -> Vec2 {
        const VALUE: f32 = 2400.0;
        if timer > 1.0 {
            velocity.normalize() * VALUE
        } else {
            Vec2::ZERO
        }
    }
}

fn update_boss_fire_bullet_pattern4(this: &mut InGameScene, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
    const BULLET_SPEED: f32 = 1.0 * PIXEL_PER_METER;

    // (한국어) 총알 발사 소리를 재생합니다.
    // (English Translation) Play the sound of a bullet being fired.
    if let Some((stream, stream_handle)) = shared.pop::<(OutputStream, OutputStreamHandle)>() {
        if let Some(sink) = sound::try_new_sink(&stream_handle)? {
            let settings = shared.get::<Settings>().unwrap();
            let asset_bundle = shared.get::<AssetBundle>().unwrap();
            let source = asset_bundle.get(path::BULLET_FIRE_SOUND_PATH)?.read(&sound::SoundDecoder)?;
            sink.set_volume(settings.effect_volume.norm());
            sink.append(source);
            thread::spawn(move || {
                sink.sleep_until_end();
            });
            shared.push((stream, stream_handle));
        }
    }

    // (한국어) 총알을 추가합니다.
    // (English Translation) Add bullets.
    let translation = {
        let instances = this.boss.sprite.instances.lock().expect("Failed to access variable.");
        instances[0].translation
    };

    let mut instances = this.enemy_bullet.instances.lock().expect("Failed to access variable.");
    let mut count = 4;
    let mut degree = Vec2::X.angle_between(this.boss.direction).to_degrees() + this.boss.behavior_count as f32 * 15.0;
    while count > 0 {
        let rotation = Quat::from_rotation_z(degree.to_radians());
        let direction = rotation.mul_vec3(Vec3::X).normalize_or_zero().xy();
        instances.push(BulletData {
            velocity: direction * BULLET_SPEED, 
            accelerator: Some(Arc::new(Transmission)), 
            life_time: BULLET_LIFE_TIME, 
            translation, 
            size: BULLET_SIZE, 
            box_size: COLLIDE_SIZE, 
            ..Default::default()
        });

        degree += 90.0;
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
        this.boss.behavior_timer = 0.05;
        this.boss.behavior_state = BossBehaviorState::WaitForFinish;
        this.boss.previous_behavior = Some(BossBehaviorState::FireBulletPattern4);
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


const FACE_UPDATE_FN: [&'static dyn Fn(f64, &wgpu::Queue, &mut Boss); 3] = [
    &update_boss_idle_face, 
    &update_boss_embarrass_face, 
    &update_boss_smile_face, 
];

/// #### 한국어 </br>
/// 보스의 얼굴을 갱신합니다. </br>
/// 
/// #### English (Translation) </br>
/// Updates the boss's face. </br>
/// 
#[inline]
pub fn update_boss_face(elapsed_time: f64, queue: &wgpu::Queue, boss: &mut Boss) {
    FACE_UPDATE_FN[boss.face_state as usize](elapsed_time, queue, boss)
}

#[inline]
fn update_boss_idle_face(_elapsed_time: f64, _queue: &wgpu::Queue, _boss: &mut Boss) {
    /* empty */
}

#[inline]
fn update_boss_embarrass_face(elapsed_time: f64, queue: &wgpu::Queue, boss: &mut Boss) {
    const DURATION: f64 = 2.5;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    boss.face_timer += elapsed_time;

    // (한국어) 지속 시간보다 클 경우 `Idle` 상태로 변경합니다.
    // (English Translation) If it is freater than the duration, it changes to `Idle` state.
    if boss.face_timer >= DURATION {
        boss.face_timer = 0.0;
        boss.face_state = BossFaceState::Idle;
        boss.sprite.update(queue, |instances| {
            instances[0].texture_index = BossFaceState::Idle as u32;
        });
    }
}

#[inline]
fn update_boss_smile_face(elapsed_time: f64, queue: &wgpu::Queue, boss: &mut Boss) {
    const DURATION: f64 = 2.5;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    boss.face_timer += elapsed_time;

    // (한국어) 지속 시간보다 클 경우 `Idle` 상태로 변경합니다.
    // (English Translation) If it is freater than the duration, it changes to `Idle` state.
    if boss.face_timer >= DURATION {
        boss.face_timer = 0.0;
        boss.face_state = BossFaceState::Idle;
        boss.sprite.update(queue, |instances| {
            instances[0].texture_index = BossFaceState::Idle as u32;
        });
    }
}
