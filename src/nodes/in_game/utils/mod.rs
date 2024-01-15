use std::sync::Arc;
use std::collections::{VecDeque, HashMap};

use rand::prelude::*;
use ab_glyph::FontArc;
use glam::{Vec4, Vec3, Vec2};
use rodio::{Sink, OutputStreamHandle};

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        bullet::{Bullet, BulletBrush}, 
        sprite::SpriteBrush, 
        text::{TextBrush, Text, TextBuilder},
        ui::{UiBrush, UiObject, UiObjectBuilder}, 
        player::{Actor, Player, FaceState}, 
        table::{Table, TileBrush}, 
        anchor::Anchor, margin::Margin, 
        script::{Script, ScriptTags}, 
        user::Settings, 
    }, 
    nodes::{
        path, 
        consts::PIXEL_PER_METER, 
        in_game::{
            self, 
            InGameScene, 
            state::InGameState, 
        }
    }, 
    render::texture::DdsTextureDecoder, 
    system::error::AppResult, 
};



/// #### 한국어 </br>
/// 일시정지 버튼 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of pause buttons. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PauseButton {
    Resume = 0, 
    Setting = 1, 
    Exit = 3, 
}



/// #### 한국어 </br>
/// `InGame` 게임 장면에서 사용되는 [`rodio::Sink`]의 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A setof [`rodio::Sink`] used in `InGame` game scene. </br>
/// 
pub struct InGameAudio {
    pub background: Sink, 
    pub voice: Sink, 
}

impl InGameAudio {
    pub fn new(settings: &Settings, stream: &OutputStreamHandle) -> AppResult<Arc<Self>> {
        use crate::components::sound;

        let background = sound::create_sink(stream)?;
        background.set_volume(settings.background_volume.norm());

        let voice = sound::create_sink(stream)?;
        voice.set_volume(settings.voice_volume.norm());

        Ok(Self {
            background, 
            voice, 
        }.into())
    }
}



pub fn create_game_scene(
    actor: Actor, 
    fonts: &HashMap<String, FontArc>, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    text_brush: &TextBrush, 
    ui_brush: &UiBrush, 
    sprite_brush: &SpriteBrush, 
    tile_brush: &TileBrush, 
    bullet_brush: &BulletBrush, 
    texture_map: &HashMap<String, wgpu::Texture>, 
    asset_bundle: &AssetBundle
) -> AppResult<InGameScene> {
    let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
        .expect("Registered font not found!");

    let nexon_lv2_gothic_bold = fonts.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)
        .expect("Registered font not found!");

    // (한국어) 텍스처 맵에서 더미 텍스처를 가져와 전경을 생성합니다.
    // (English Translation) Creates the foreground by taking a dummy texture from the texture map. 
    let texture = texture_map.get(path::DUMMY_TEXTURE_PATH)
        .expect("A registered texture could not be found.");
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );
    let foreground = create_foreground(
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );


    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let texture = asset_bundle.get(path::INGAME_BACKGROUND_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Background"), 
            size: wgpu::Extent3d {
                width: 1024,
                height: 768, 
                depth_or_array_layers: 1, 
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 11, 
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue, 
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::INGAME_BACKGROUND_TEXTURE_PATH);

    let background = create_background(
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );



    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let image_rel_path = match actor {
        _ => path::TEMP_STAGE_TEXTURE_PATH, 
    };
    let texture = asset_bundle.get(image_rel_path)?
        .read(&DdsTextureDecoder {
            name: Some("Image"), 
            size: wgpu::Extent3d {
                width: 2048, 
                height: 2048, 
                depth_or_array_layers: 1,
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 12, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(image_rel_path);

    let stage_image = create_stage_image(
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );



    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let rel_path = match actor {
        Actor::Aris => path::ARIS_BULLET_TEXTURE_PATH, 
        Actor::Yuzu => path::YUZU_BULLET_TEXTURE_PATH, 
        Actor::Momoi => path::MOMOI_BULLET_TEXTURE_PATH, 
        Actor::Midori => path::MIDORI_BULLET_TEXTURE_PATH, 
    };

    let texture = asset_bundle.get(rel_path)?
        .read(&DdsTextureDecoder {
            name: Some("Bullet"), 
            size: match actor {
                Actor::Aris => wgpu::Extent3d { width: 256, height: 128, depth_or_array_layers: 1 }, 
                _ => wgpu::Extent3d { width: 128, height: 128, depth_or_array_layers: 1 },
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: match actor {
                Actor::Aris => 9,
                _ => 8,
            }, 
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue, 
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(rel_path);

    // (한국어) 총알 스프라이트들을 생성합니다.
    // (English Translation) Create bullet sprites.
    let player_bullet = Bullet::with_capacity(
        device, 
        tex_sampler, 
        &texture_view, 
        bullet_brush, 
        64
    );



    // (한국어) 게임 장면의 타일들을 생성합니다.
    // (English Translation) Create tiles for the `InGame` game scene.
    let table = Table::new(
        100, 
        100, 
        6, 
        Vec4::new(137.0 / 255.0, 207.0 / 255.0, 243.0 / 255.0, 1.0), 
        Vec4::new(160.0 / 255.0, 233.0 / 255.0, 255.0 / 255.0, 1.0), 
        Vec4::new(1.0, 0.0, 0.0, 1.0), 
        Vec3::new(
            -35.0 * PIXEL_PER_METER, 
            -25.0 * PIXEL_PER_METER, 
            -1.0 * PIXEL_PER_METER
        ),
        Vec2::new(
            0.5 * PIXEL_PER_METER, 
            0.5 * PIXEL_PER_METER
        ),
        queue, 
        tile_brush
    );



    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let image_rel_path = match actor {
        Actor::Aris => path::ARIS_PLAYER_TEXTURE_PATH, 
        Actor::Momoi => path::MOMOI_PLAYER_TEXTURE_PATH, 
        Actor::Midori => path::MIDORI_PLAYER_TEXTURE_PATH, 
        Actor::Yuzu => path::YUZU_PLAYER_TEXTURE_PATH, 
    };
    let texture = asset_bundle.get(image_rel_path)?
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
            dimension: Some(wgpu::TextureViewDimension::D2Array), 
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(image_rel_path);
    
    let player = Player::new(
        actor, 
        table.player_spawn_pos.0, 
        table.player_spawn_pos.1, 
        -0.5 * PIXEL_PER_METER, 
        &table, 
        device, 
        tex_sampler, 
        &texture_view, 
        sprite_brush
    );

    let player_faces = create_player_face(
        device, 
        &texture, 
        tex_sampler, 
        ui_brush
    );


    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let texture = asset_bundle.get(path::BUTTON_ETC_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("EtcButton"), 
            size: wgpu::Extent3d {
                width: 256, 
                height: 256, 
                depth_or_array_layers: 1, 
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 9, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue, 
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::BUTTON_ETC_TEXTURE_PATH);

    let menu_button = create_menu_button(
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );



    // (한국어) `dds`이미지 파일로부터 윈도우 배경 텍스처를 생성합니다.
    // (English Translation) Create a window background texture from a `dds`image file. 
    let texture = asset_bundle.get(path::WINDOW_RATIO_4_3_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("ExitMessageBoxBackground"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 768,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::WINDOW_RATIO_4_3_TEXTURE_PATH);

    let (remaining_timer_bg, remaining_timer_text) = create_remaining_timer(
        nexon_lv2_gothic_bold, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    );



    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다. 
    // (English Translation) Load an image file and create a texture. 
    let texture = asset_bundle.get(path::HEART_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Heart"), 
            size: wgpu::Extent3d {
                width: 256, 
                height: 256, 
                depth_or_array_layers: 1, 
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 9, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device, 
            queue, 
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::HEART_TEXTURE_PATH);

    let lost_hearts = VecDeque::with_capacity(3);
    let owned_hearts = create_player_hearts(
        3, 
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );



    // (한국어) 현재 플레이어가 차지한 영역의 비율을 보여주는 텍스트를 생성합니다.
    // (English Translation) Creates text showing the percentage of area currently occupied by the player. 
    let percent = create_percent_text(
        nexon_lv2_gothic_bold, 
        device, 
        queue, 
        text_brush
    );


    
    // (한국어) 일시정지 버튼 텍스처를 생성합니다.
    // (English Translation) Creates a pause window. 
    let texture = asset_bundle.get(path::BUTTON_WIDE_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("WideButton"),
            size: wgpu::Extent3d {
                width:1024,
                height:192,
                depth_or_array_layers:1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 11,
            sample_count:1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device: &device,
            queue: &queue,
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::BUTTON_WIDE_TEXTURE_PATH);

    // (한국어) 일시정지 타이틀 텍스트와 일시정지 버튼들을 생성합니다.
    // (English Translation) Create pause title text and pause buttons. 
    let pause_text = create_pause_text(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        text_brush, 
        script
    )?;
    let pause_buttons = create_pause_buttons(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    )?;


    // (한국어) `InGame` 게임 장면에서 사용되는 음향 에셋들을 로드합니다.
    // (English Translation) Load sound assets used in `InGame` game scene. 
    let player_startup_sound = match actor {
        Actor::Aris => path::ARIS_STAGE_START_SOUND_PATH,
        Actor::Momoi => path::MOMOI_STAGE_START_SOUND_PATH, 
        Actor::Midori => path::MIDORI_STAGE_START_SOUND_PATH, 
        Actor::Yuzu => path::YUZU_STAGE_START_SOUND_PATH, 
    };

    let player_smile_sounds = match actor {
        Actor::Aris => vec![
            path::ARIS_SMILE_0_SOUND_PATH, 
            path::ARIS_SMILE_1_SOUND_PATH
        ],
        Actor::Momoi => vec![
            path::MOMOI_SMILE_0_SOUND_PATH, 
            path::MOMOI_SMILE_1_SOUND_PATH, 
        ], 
        Actor::Midori => vec![
            path::MIDORI_SMILE_0_SOUND_PATH, 
            path::MIDORI_SMILE_1_SOUND_PATH, 
        ], 
        Actor::Yuzu => vec![
            path::YUZU_SMILE_0_SOUND_PATH, 
            path::YUZU_SMILE_1_SOUND_PATH, 
        ], 
    };

    let player_damage_sounds = match actor {
        Actor::Aris => vec![
            path::ARIS_DAMAGE_0_SOUND_PATH, 
            path::ARIS_DAMAGE_1_SOUND_PATH, 
            path::ARIS_DAMAGE_2_SOUND_PATH
        ], 
        Actor::Momoi => vec![
            path::MOMOI_DAMAGE_0_SOUND_PATH, 
            path::MOMOI_DAMAGE_1_SOUND_PATH, 
            path::MOMOI_DAMAGE_2_SOUND_PATH, 
        ], 
        Actor::Midori => vec![
            path::MIDORI_DAMAGE_0_SOUND_PATH, 
            path::MIDORI_DAMAGE_1_SOUND_PATH, 
            path::MIDORI_DAMAGE_2_SOUND_PATH, 
        ], 
        Actor::Yuzu => vec![
            path::YUZU_DAMAGE_0_SOUND_PATH, 
            path::YUZU_DAMAGE_1_SOUND_PATH, 
            path::YUZU_DAMAGE_2_SOUND_PATH, 
        ],
    };

    let player_fire_sound = match actor {
        Actor::Aris => path::ARIS_FIRE_SOUND_PATH, 
        Actor::Momoi => path::MOMOI_FIRE_SOUND_PATH, 
        Actor::Midori => path::MIDORI_FIRE_SOUND_PATH, 
        Actor::Yuzu => path::YUZU_FIRE_SOUND_PATH, 
    };

    let mut candidates = [path::THEME18_SOUND_PATH, path::THEME19_SOUND_PATH, path::THEME30_SOUND_PATH];
    candidates.shuffle(&mut rand::thread_rng());
    let bgm_sound = candidates[0];


    // (한국어) 현재 게임 장면에서 사용되는 에셋들을 로드합니다.
    // (English Translation) Loads assets used in the current game scene. 
    asset_bundle.get(player_startup_sound)?;
    for rel_path in player_smile_sounds.iter() {
        asset_bundle.get(rel_path)?;
    }
    for rel_path in player_damage_sounds.iter() {
        asset_bundle.get(rel_path)?;
    }
    asset_bundle.get(player_fire_sound)?;
    asset_bundle.get(bgm_sound)?;



    Ok(InGameScene {
        mouse_pressed: false, 
        keyboard_pressed: false, 
        timer: 0.0, 
        remaining_time: in_game::GAME_DURATION_SEC, 
        state: InGameState::default(), 
        pause_text, 
        pause_buttons, 
        percent, 
        percent_timer: in_game::PERCENT_DURATION, 
        num_total_tiles: in_game::NUM_TILES as u32, 
        num_owned_tiles: 0, 
        owned_tiles: VecDeque::new(), 
        owned_hearts, 
        lost_hearts, 
        foreground, 
        background, 
        stage_image, 
        menu_button, 
        remaining_timer_bg, 
        remaining_timer_text, 
        table, 
        player, 
        player_faces, 
        player_bullet, 
        player_startup_sound, 
        player_smile_sounds, 
        player_damage_sounds, 
        player_fire_sound, 
        bgm_sound, 
    })
}

/// #### 한국어 </br>
/// 게임 장면 전환에 사용되는 전경 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a foreground user interface used for game transitions. </br>
/// 
#[inline]
fn create_foreground(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> UiObject {
    UiObjectBuilder::new(
        Some("Foreground"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(1.0, 0.0, 0.0, 1.0))
    .with_color((0.0, 0.0, 0.0, 1.0).into())
    .build(device)
}

/// #### 한국어 </br>
/// `InGame` 게임 장면의 배경을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a background for a `InGame` game scene. </br>
/// 
#[inline]
fn create_background(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> UiObject {
    UiObjectBuilder::new(
        Some("Background"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(1.0, 0.0, 0.0, 1.0))
    .with_global_translation((0.0, 0.0, 1.0).into())
    .build(device)
}

/// #### 한국어 </br>
/// `InGame` 게임 장면의 스테이지 이미지를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a stage image for a `InGame` game scene. </br>
/// 
#[inline]
fn create_stage_image(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> UiObject {
    UiObjectBuilder::new(
        Some("StageImage"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(
        0.9166666667, 
        0.0625, 
        0.0833333333, 
        0.6875
    ))
    .with_global_translation((0.0, 0.0, 0.75).into())
    .build(device)
}

/// #### 한국어 </br>
/// `InGame` 게임 장면의 메뉴 버튼을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a menu button for a `InGame` game scene. </br>
/// 
#[inline]
fn create_menu_button(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> UiObject {
    UiObjectBuilder::new(
        Some("MenuButton"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(
        1.0 - 0.02666666667, 
        0.9, 
        1.0 - 0.1333333333, 
        0.98
    ))
    .with_global_translation((0.0, 0.0, 0.5).into())
    .build(device)
}

/// #### 한국어 </br>
/// 남은 시간을 표시하는 타이머를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a timer that displays the remaining time. </br>
/// 
fn create_remaining_timer(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> (UiObject, Text) {
    let bg = UiObjectBuilder::new(
        Some("RemainingTimer"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.75).into())
    .with_anchor(Anchor::new(1.0 - 0.03666666667, 0.73, 1.0 - 0.1233333333, 0.88))
    .build(device);

    let text = TextBuilder::new(
        Some("RemainingTimer"), 
        font, 
        "2:00", 
        text_brush
    )
    .with_translation((0.0, 0.0, 0.5).into())
    .with_anchor(Anchor::new(1.0 - 0.01666666667, 0.73, 1.0 - 0.1433333333, 0.88))
    .build(device, queue);

    return (bg, text);
}

/// #### 한국어 </br>
/// 플레이어의 상태를 보여주는 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface that shows the player's status. </br>
/// 
fn create_player_face(
    device: &wgpu::Device, 
    texture: &wgpu::Texture, 
    tex_sampler: &wgpu::Sampler, 
    ui_brush: &UiBrush
) -> HashMap<FaceState, UiObject> {
    let scale = (0.0, 0.0, 0.0).into();
    let anchor = Anchor::new(0.5 + 0.1333333333, 0.88, 0.5, 0.98);

    // (한국어) `Idle` 상태의 플레이어 얼굴 인터페이스를 생성합니다. 
    // (English Translation) Creates a player face interface in the `Idle` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: FaceState::Idle as u32, 
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2), 
            ..Default::default()
        }
    );
    let idle = UiObjectBuilder::new(
        Some("PlayerIdleFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);

    // (한국어) `Hit` 상태의 플레이어 얼굴 인터페이스를 생성합니다. 
    // (English Translation) Creates a player face interface in the `Hit` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: FaceState::Hit as u32,
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        }
    );
    let hit = UiObjectBuilder::new(
        Some("PlayerHitFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);

    // (한국어) `Smile` 상태의 플레이어 얼굴 인터페이스를 생성합니다. 
    // (English Translation) Creates a player face interface in the `Smile` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: FaceState::Smile as u32,
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        }
    );
    let smile = UiObjectBuilder::new(
        Some("PlayerSmileFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);

    return HashMap::from_iter([
        (FaceState::Idle, idle), 
        (FaceState::Hit, hit), 
        (FaceState::Smile, smile), 
    ]);
}

/// #### 한국어 </br>
/// 플레이어 체력을 보여주는 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface that shows player health. </br>
/// 
fn create_player_hearts(
    life_count: u32, 
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> VecDeque<UiObject> {
    let left = 0.72;
    let right = 0.98;
    let gap = 0.02;
    let height = 0.75;
    let width = ((right - left) - gap * (life_count - 1) as f32) / life_count as f32;

    let mut hearts = VecDeque::with_capacity(life_count as usize);
    for num in 0..life_count {
        hearts.push_back(UiObjectBuilder::new(
            Some(&format!("PlayerHeart({})", num)), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_global_translation((0.0, 0.0, 0.5).into())
        .with_local_scale((0.0, 0.0, 0.0).into())
        .with_anchor(Anchor::new(
            height + 0.5 * (width * 4.0 / 3.0),
            left + (width * num as f32) + (gap * num as f32), 
            height - 0.5 * (width * 4.0 / 3.0),
            left + (width * (num + 1) as f32) + (gap * num as f32)
        ))
        .build(device))
    }

    return hearts;
}

/// #### 한국어 </br>
/// 플레이어가 차지한 영역의 퍼센트를 보여주는 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface that shows the percentage of owned area by the player. </br>
#[inline]
fn create_percent_text(
    font: &FontArc,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> Text {
    TextBuilder::new(
        Some("Percent"), 
        font, 
        "0%", 
        text_brush
    )
    .with_anchor(Anchor::new(0.1 + 0.3, 0.72, 0.1, 0.98))
    .with_translation((0.0, 0.0, 0.25).into())
    .build(device, queue)
}

/// #### 한국어 </br>
/// 일시정지 사용자 인터페이스 윈도우를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a puse user interface window. </br>
/// 
fn create_pause_text(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush, 
    script: &Script
) -> AppResult<Text> {
    let text = script.get(ScriptTags::PauseTitle)?;
    return Ok(TextBuilder::new(
        Some("PuaseWindow"), 
        font, 
        text, 
        text_brush
    ).with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(240, -320, 112, 320))
    .with_color((1.0, 1.0, 1.0, 0.0).into())
    .with_translation((0.0, 0.0, 0.5).into())
    .build(device, queue));
}

/// #### 한국어 </br>
/// 일시정지 화면에서 사용되는 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create buttons used on the pause screen. </br>
/// 
fn create_pause_buttons(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush, 
) -> AppResult<HashMap<PauseButton, (UiObject, Text)>> {
    let resume_btn = UiObjectBuilder::new(
        Some("ResumeButton"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(100, -160, 40, 160))
    .with_color((1.0, 1.0, 1.0, 0.0).into())
    .build(device);
    let text = script.get(ScriptTags::PauseResumeButton)?;
    let resume_text = TextBuilder::new(
        Some("ResumeText"), 
        font, 
        text, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(100, -160, 40, 160))
    .with_color((0.0, 0.0, 0.0, 0.0).into())
    .build(device, queue);

    let setting_btn = UiObjectBuilder::new(
        Some("SettingButton"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(10, -160, -50, 160))
    .with_color((1.0, 1.0, 1.0, 0.0).into())
    .build(device);
    let text = script.get(ScriptTags::PuaseSettingButton)?;
    let setting_text = TextBuilder::new(
        Some("SettingText"), 
        font, 
        text, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(10, -160, -50, 160))
    .with_color((0.0, 0.0, 0.0, 0.0).into())
    .build(device, queue);

    let exit_button = UiObjectBuilder::new(
        Some("ExitButton"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-80, -160, -140, 160))
    .with_color((255.0 / 255.0, 103.0 / 255.0, 105.0 / 255.0, 0.0).into())
    .build(device);
    let text = script.get(ScriptTags::PauseExitButton)?;
    let exit_text =  TextBuilder::new(
        Some("ExitText"), 
        font, 
        text, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-80, -160, -140, 160))
    .with_color((0.0, 0.0, 0.0, 0.0).into())
    .build(device, queue);

    return Ok(HashMap::from_iter([
        (PauseButton::Resume, (resume_btn, resume_text)), 
        (PauseButton::Setting, (setting_btn, setting_text)), 
        (PauseButton::Exit, (exit_button, exit_text)), 
    ]));
}