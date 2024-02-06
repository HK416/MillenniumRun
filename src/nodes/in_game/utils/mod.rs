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
        player::{self, Actor, Player, PlayerFaceState}, 
        boss::{Boss, BossFaceState}, 
        table::{Table, TileBrush}, 
        anchor::Anchor, margin::Margin, 
        script::{Script, ScriptTags}, 
        user::{Language, Resolution, Settings}, 
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
    GiveUp = 3, 
}

/// #### 한국어 </br>
/// 종료 창의 버튼 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the list of buttons in the exit window.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExitWndButton {
    Yes = 0,
    No = 1, 
}

/// #### 한국어 </br>
/// 사용자가 설정 할 수 있는 음향 옵션 목록입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of sound options that the user can set. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeOptions {
    Background, 
    Effect, 
    Voice, 
}

pub const SETTING_VOLUME_RANGE_MAX: i32 = 272;
pub const SETTING_VOLUME_RANGE_MIN: i32 = -240;
pub const VOLUME_BAR_WIDTH: i32 = 8;


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
    settings: &Settings,
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

    let setting_volume_background = create_setting_volume_background(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    )?;
    let setting_volume_bar = create_setting_volume_bar(
        settings, 
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
    let default_texture = texture_map.get(path::DEF_IMG_TEXTURE_PATH)
        .expect("Registered image not found!");
    let stage_texture = texture_map.get(match actor {
        Actor::Aris => path::ARIS_IMG_TEXTURE_PATH, 
        Actor::Momoi => path::MOMOI_IMG_TEXTURE_PATH, 
        Actor::Midori => path::MIDORI_IMG_TEXTURE_PATH, 
        Actor::Yuzu => path::YUZU_IMG_TEXTURE_PATH, 
    }).expect("Registered image not found!");
    let stage_images = create_stage_image(
        device, 
        tex_sampler, 
        default_texture, 
        stage_texture,
        ui_brush
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
    let texture = asset_bundle.get(path::YUUKA_BULLET_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Bullet(Enemy)"), 
            size: wgpu::Extent3d { width: 128, height: 128, depth_or_array_layers: 1 }, 
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
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::YUUKA_BULLET_TEXTURE_PATH);

    // (한국어) 총알 스프라이트들을 생성합니다.
    // (English Translation) Create bullet sprites.
    let enemy_bullet = Bullet::with_capacity(
        device, 
        tex_sampler, 
        &texture_view, 
        bullet_brush, 
        128
    );



    // (한국어) 이미지 파일을 불러오고, 텍스처를 생성합니다.
    // (English Translation) Load an image file and create a texture. 
    let texture = asset_bundle.get(path::YUUKA_ENEMY_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Yuuka"), 
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
            queue,
        })?;
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::YUUKA_ENEMY_TEXTURE_PATH);

    let boss = Boss::new(
        table.boss_spawn_pos.0, 
        table.boss_spawn_pos.1, 
        -0.5 * PIXEL_PER_METER, 
        &table, 
        device, 
        tex_sampler, 
        &texture_view, 
        sprite_brush
    );

    let boss_faces = create_boss_face(
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
    let window_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::WINDOW_RATIO_4_3_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 하위 윈도우 배경 텍스처를 생성합니다.
    // (English Translation) Create a sub window background texture from a `dds`image file. 
    let texture = asset_bundle.get(path::WINDOW_RATIO_8_1_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("ExitMessageBoxBackground"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 128,
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
    let sub_window_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::WINDOW_RATIO_8_1_TEXTURE_PATH);


    let (remaining_timer_bg, remaining_timer_text) = create_remaining_timer(
        nexon_lv2_gothic_bold, 
        device, 
        queue, 
        tex_sampler, 
        &window_texture_view, 
        ui_brush, 
        text_brush
    );

    let pause_exit_window = create_exit_window(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &window_texture_view, 
        ui_brush, 
        text_brush
    )?;

    let setting_windows = create_setting_windows(
        device, 
        tex_sampler, 
        &window_texture_view, 
        &sub_window_texture_view, 
        ui_brush
    );
    let setting_titles = create_setting_window_titles(
        nexon_lv2_gothic_medium, 
        nexon_lv2_gothic_bold, 
        script, 
        device, 
        queue, 
        text_brush
    )?;


    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::BUTTON_MEDIUM_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("MediumButton"),
            size: wgpu::Extent3d {
                width: 768,
                height: 256,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            mip_level_count: 10,
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
    asset_bundle.release(path::BUTTON_MEDIUM_TEXTURE_PATH);

    let result_window_btn = create_result_window_btn(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    )?;

    let result_condition_texts = create_result_condition_texts(
        nexon_lv2_gothic_bold, 
        script, 
        device, 
        queue, 
        text_brush
    )?;

    let pause_exit_buttons = create_exit_buttons(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    )?;

    let setting_languages = create_setting_languages(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    );
    let setting_resolutions = create_setting_resolutions(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    );
    let setting_return_button = create_setting_return_button(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &texture_view, 
        ui_brush, 
        text_brush
    )?;


    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::FINISH_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Finish"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 512,
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
    asset_bundle.release(path::FINISH_TEXTURE_PATH);

    let result_title = create_result_title(
        device, 
        tex_sampler, 
        &texture_view, 
        ui_brush
    );


    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::STAR_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Star"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 512,
                depth_or_array_layers: 5,
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
    let result_stars = create_result_stars(
        device, 
        tex_sampler, 
        &texture, 
        ui_brush
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

    let lost_hearts = VecDeque::with_capacity(player::MAX_PLAYER_HEARTS);
    let owned_hearts = create_player_hearts(
        player::MAX_PLAYER_HEARTS as u32, 
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
            path::YUUKA_ATTACK0_SOUND_PATH, 
            path::ARIS_DAMAGE_0_SOUND_PATH, 
            path::ARIS_DAMAGE_1_SOUND_PATH, 
            path::ARIS_DAMAGE_2_SOUND_PATH
        ], 
        Actor::Momoi => vec![
            path::YUUKA_ATTACK0_SOUND_PATH, 
            path::MOMOI_DAMAGE_0_SOUND_PATH, 
            path::MOMOI_DAMAGE_1_SOUND_PATH, 
            path::MOMOI_DAMAGE_2_SOUND_PATH, 
        ], 
        Actor::Midori => vec![
            path::YUUKA_ATTACK0_SOUND_PATH, 
            path::MIDORI_DAMAGE_0_SOUND_PATH, 
            path::MIDORI_DAMAGE_1_SOUND_PATH, 
            path::MIDORI_DAMAGE_2_SOUND_PATH, 
        ], 
        Actor::Yuzu => vec![
            path::YUUKA_ATTACK0_SOUND_PATH, 
            path::YUZU_DAMAGE_0_SOUND_PATH, 
            path::YUZU_DAMAGE_1_SOUND_PATH, 
            path::YUZU_DAMAGE_2_SOUND_PATH, 
        ],
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
    asset_bundle.get(bgm_sound)?;



    Ok(InGameScene {
        timer: 0.0, 
        remaining_time: in_game::GAME_DURATION_SEC, 
        state: InGameState::default(), 
        pause_text, 
        pause_buttons, 
        pause_exit_window, 
        pause_exit_buttons, 
        percent, 
        percent_timer: in_game::PERCENT_DURATION, 
        num_total_tiles: in_game::NUM_TILES as u32, 
        num_owned_tiles: 0, 
        owned_tiles: VecDeque::new(), 
        owned_hearts, 
        lost_hearts, 
        foreground, 
        background, 
        stage_images, 
        menu_button, 
        remaining_timer_bg, 
        remaining_timer_text, 
        result_window_btn, 
        result_title, 
        result_stars, 
        result_star_index: 0, 
        result_challenge_texts: result_condition_texts, 
        table, 
        player, 
        player_faces, 
        boss, 
        boss_faces, 
        enemy_bullet, 
        player_startup_sound, 
        player_smile_sounds, 
        player_damage_sounds, 
        bgm_sound, 
        setting_windows, 
        setting_titles, 
        setting_languages, 
        setting_resolutions, 
        setting_return_button, 
        setting_volume_background, 
        setting_volume_bar, 
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
    default_texture: &wgpu::Texture, 
    stage_texture: &wgpu::Texture, 
    ui_brush: &UiBrush
) -> Vec<UiObject> {
    let global_translation = Vec3::new(0.0, 0.0, 0.75);
    let anchor = Anchor::new(0.9166666667, 0.0625, 0.0833333333, 0.6875);
    let mut stage_images = Vec::with_capacity(5);

    let texture_view = default_texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );
    stage_images.push(
        UiObjectBuilder::new(
            Some("StageImage0"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_global_translation(global_translation)
        .build(device)
    );

    let texture_view = stage_texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 0, 
            array_layer_count: Some(1), 
            ..Default::default()
        }
    );
    stage_images.push(
        UiObjectBuilder::new(
            Some("StageImage1"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_global_translation(global_translation)
        .build(device)
    );

    let texture_view = stage_texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 1, 
            array_layer_count: Some(1), 
            ..Default::default()
        }
    );
    stage_images.push(
        UiObjectBuilder::new(
            Some("StageImage2"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_global_translation(global_translation)
        .build(device)
    );

    let texture_view = stage_texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 2, 
            array_layer_count: Some(1), 
            ..Default::default()
        }
    );
    stage_images.push(
        UiObjectBuilder::new(
            Some("StageImage3"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_global_translation(global_translation)
        .build(device)
    );

    return stage_images;
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

    let min = (in_game::GAME_DURATION_SEC / 60.0) as u32;
    let sec = (in_game::GAME_DURATION_SEC % 60.0) as u32;
    let text = TextBuilder::new(
        Some("RemainingTimer"), 
        font,         
        &format!("{}:{:0>2}", min, sec), 
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
) -> HashMap<PlayerFaceState, UiObject> {
    let scale = (0.0, 0.0, 0.0).into();
    let anchor = Anchor::new(0.55 + 0.1333333333, 0.88, 0.55, 0.98);

    // (한국어) `Idle` 상태의 플레이어 얼굴 인터페이스를 생성합니다. 
    // (English Translation) Creates a player face interface in the `Idle` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: PlayerFaceState::Idle as u32, 
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
            base_array_layer: PlayerFaceState::Hit as u32,
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
            base_array_layer: PlayerFaceState::Smile as u32,
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
        (PlayerFaceState::Idle, idle), 
        (PlayerFaceState::Hit, hit), 
        (PlayerFaceState::Smile, smile), 
    ]);
}

/// #### 한국어 </br>
/// 보스의 상태를 보여주는 사용자 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a user interface that shows the boss's status. </br>
/// 
fn create_boss_face(
    device: &wgpu::Device, 
    texture: &wgpu::Texture, 
    tex_sampler: &wgpu::Sampler, 
    ui_brush: &UiBrush
) -> HashMap<BossFaceState, UiObject> {
    let scale = Vec3::new(0.0, 0.0, 0.0);
    let anchor = Anchor::new(0.55 + 0.1333333333, 0.72, 0.55, 0.82);

    // (한국어) `Idle` 상태의 보스 얼굴 인터페이스를 생성합니다.
    // (English Transalation) Creates a boss face interface in the `Idle` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: BossFaceState::Idle as u32, 
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2), 
            ..Default::default()
        }
    );
    let idle = UiObjectBuilder::new(
        Some("BossIdleFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);
    
    // (한국어) `Embarrass` 상태의 보스 얼굴 인터페이스를 생성합니다.
    // (English Translation) Creates a boss face interface in the `Embarrass` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: BossFaceState::Embarrass as u32, 
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2), 
            ..Default::default()
        }
    );
    let embarrass = UiObjectBuilder::new(
        Some("BossEmbarrassFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);

    // (한국어) `Smile` 상태의 보스 얼굴 인터페이스를 생성합니다.
    // (English Translation) Creates a boss face interface in the `Smile` state. 
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            base_array_layer: BossFaceState::Smile as u32, 
            array_layer_count: Some(1), 
            dimension: Some(wgpu::TextureViewDimension::D2), 
            ..Default::default()
        }
    );
    let smile = UiObjectBuilder::new(
        Some("BossSmileFace"), 
        tex_sampler, 
        &texture_view, 
        ui_brush
    )
    .with_global_translation((0.0, 0.0, 0.5).into())
    .with_local_scale(scale)
    .with_anchor(anchor)
    .build(device);

    return HashMap::from_iter([
        (BossFaceState::Idle, idle), 
        (BossFaceState::Embarrass, embarrass), 
        (BossFaceState::Smile, smile), 
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
    let height = 0.78;
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
    .with_anchor(Anchor::new(0.15 + 0.3, 0.72, 0.15, 0.98))
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
    let text = script.get(ScriptTags::InGamePauseTitle)?;
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
    let text = script.get(ScriptTags::InGameResumeButton)?;
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
    let text = script.get(ScriptTags::InGameSettingButton)?;
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
    let text = script.get(ScriptTags::InGameGiveUpButton)?;
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
        (PauseButton::GiveUp, (exit_button, exit_text)), 
    ]));
}

/// #### 한국어 </br>
/// 결과 화면의 `나가기` 버튼을 생성합니다. </br>
///  
/// #### English (Translation) </br>
/// Creates an `Exit` button in the results screen. </br>
/// 
fn create_result_window_btn(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<(UiObject, Text)> {
    let anchor = Anchor::new(0.075 + 0.1155555556, 0.72, 0.075, 0.98);
    Ok((
        UiObjectBuilder::new(
            Some("ExitButton"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_color((255.0 / 255.0, 103.0 / 255.0, 105.0 / 255.0, 0.0).into())
        .build(device), 
        TextBuilder::new(
            Some("ExitButton"), 
            font, 
            script.get(ScriptTags::InGameExitButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_color((0.0, 0.0, 0.0, 0.0).into())
        .build(device, queue)
    ))
}

/// #### 한국어 </br>
/// 결과 화면의 타이틀을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a title for the results screen. </br>
/// 
fn create_result_title(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> UiObject {
    UiObjectBuilder::new(
        Some("Finish"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.8 + 0.1733333333, 0.72, 0.8, 0.98))
    .with_local_scale((0.0, 0.0, 0.0).into())
    .build(device)
}

/// #### 한국어 </br>
/// 결과 화면의 점수를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates scores for the results screen. </br>
/// 
fn create_result_stars(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture: &wgpu::Texture, 
    ui_brush: &UiBrush
) -> Vec<UiObject> {
    let scale = Vec3::new(0.0, 0.0, 0.0);
    let anchor = Anchor::new(0.7 + 0.1333333333, 0.75, 0.7, 0.95);
    let mut stars = Vec::with_capacity(5);
    
    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 0, 
            array_layer_count: Some(1), 
            ..Default::default()
        }
    );
    stars.push(
        UiObjectBuilder::new(
            Some("EmptyStar"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_local_scale(scale)
        .build(device)
    );

    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 1, 
            array_layer_count: Some(1),
            ..Default::default()
        }
    );
    stars.push(
        UiObjectBuilder::new(
            Some("OneStar"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_local_scale(scale)
        .build(device)
    );

    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 2, 
            array_layer_count: Some(1),
            ..Default::default()
        }
    );
    stars.push(
        UiObjectBuilder::new(
            Some("TwoStar"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_local_scale(scale)
        .build(device)
    );

    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 3, 
            array_layer_count: Some(1),
            ..Default::default()
        }
    );
    stars.push(
        UiObjectBuilder::new(
            Some("ThreeStar"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_local_scale(scale)
        .build(device)
    );

    let texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2), 
            base_array_layer: 4, 
            array_layer_count: Some(1),
            ..Default::default()
        }
    );
    stars.push(
        UiObjectBuilder::new(
            Some("PerfectStar"), 
            tex_sampler, 
            &texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_local_scale(scale)
        .build(device)
    );

    return stars;
}

fn create_result_condition_texts(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> AppResult<Vec<Text>> {
    let mut texts = Vec::with_capacity(3);
    texts.push(
        TextBuilder::new(
            Some("Condition0"), 
            font, 
            script.get(ScriptTags::InGameChallenge0)?,
            text_brush
        )
        .with_anchor(Anchor::new(0.625, 0.72, 0.55, 0.98))
        .with_color((162.0 / 255.0, 162.0 / 255.0, 160.0 / 255.0, 0.0).into())
        .build(device, queue)
    );

    texts.push(
        TextBuilder::new(
            Some("Condition1"), 
            font, 
            script.get(ScriptTags::InGameChallenge1)?, 
            text_brush
        )
        .with_anchor(Anchor::new(0.55, 0.72, 0.475, 0.98))
        .with_color((162.0 / 255.0, 162.0 / 255.0, 160.0 / 255.0, 0.0).into())
        .build(device, queue)
    );

    texts.push(
        TextBuilder::new(
            Some("Condition2"), 
            font, 
            script.get(ScriptTags::InGameChallenge2)?, 
            text_brush
        )
        .with_anchor(Anchor::new(0.475, 0.72, 0.4, 0.98))
        .with_color((162.0 / 255.0, 162.0 / 255.0, 160.0 / 255.0, 0.0).into())
        .build(device, queue)
    );

    return Ok(texts);
}

/// #### 한국어 </br>
/// 종료 창을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a exit window. </br>
/// 
fn create_exit_window(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<(UiObject, Text)> {
    const ANCHOR_TOP: f32 = 0.5;
    const ANCHOR_LEFT: f32 = 0.5;
    const ANCHOR_BOTTOM: f32 = 0.5;
    const ANCHOR_RIGHT: f32 = 0.5;

    const WND_WIDTH: i32 = 400;
    const WND_HEIGHT: i32 = WND_WIDTH / 4 * 3;
    
    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let wnd_margin = Margin::new(WND_HEIGHT / 2, -WND_WIDTH / 2, -WND_HEIGHT / 2, WND_WIDTH / 2);
    let text_margin = Margin::new(WND_HEIGHT / 5, -WND_WIDTH / 2, 0, WND_WIDTH / 2);
    let ui = UiObjectBuilder::new(
        Some("ExitWindow"), 
        tex_sampler, 
        texture_view, 
        ui_brush
    )
    .with_anchor(anchor)
    .with_margin(wnd_margin)
    .with_color((1.0, 1.0, 1.0, 1.0).into())
    .with_global_scale((0.0, 0.0, 0.0).into())
    .with_global_translation((0.0, 0.0, 0.75).into())
    .build(device);

    let text = TextBuilder::new(
        Some("ExitWindowText"), 
        font, 
        script.get(ScriptTags::InGameGiveUpReconfirmMessage)?, 
        text_brush
    )
    .with_anchor(anchor)
    .with_margin(text_margin)
    .with_scale((0.0, 0.0, 0.0).into())
    .with_color((0.0, 0.0, 0.0, 1.0).into())
    .with_translation((0.0, 0.0, 0.5).into())
    .build(device, queue);

    return Ok((ui, text));
}

/// #### 한국어 </br>
/// 종료 창의 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create buttons for the exit window. </br>
/// 
fn create_exit_buttons(
    font: &FontArc, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<HashMap<ExitWndButton, (UiObject, Text)>> {
    const ANCHOR_TOP: f32 = 0.5;
    const ANCHOR_LEFT: f32 = 0.5;
    const ANCHOR_BOTTOM: f32 = 0.5;
    const ANCHOR_RIGHT: f32 = 0.5;

    const WND_WIDTH: i32 = 400;
    const WND_HEIGHT: i32 = WND_WIDTH / 4 * 3;

    const BTN_WIDTH: i32 = 150;
    const BTN_HEIGHT: i32 = BTN_WIDTH / 3;
    const BTN_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.5);

    const YES_BTN_COLOR: Vec4 = Vec4::new(255.0 / 255.0, 103.0 / 255.0, 105.0 / 255.0, 1.0);
    const NO_BTN_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

    const TEXT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.25);
    const TEXT_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);

    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(
        BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        -BTN_WIDTH / 2 - WND_WIDTH / 5,
        -BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        BTN_WIDTH / 2 - WND_WIDTH / 5
    );
    let yes_btn = (
        UiObjectBuilder::new(
            Some("YesButton"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(YES_BTN_COLOR)
        .with_global_translation(BTN_TRANSLATION)
        .build(device),
        TextBuilder::new(
            Some("YesButtonText"), 
            font, 
            script.get(ScriptTags::InGameGiveUpOkayButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue)
    );

    let anchor = Anchor::new(ANCHOR_TOP, ANCHOR_LEFT, ANCHOR_BOTTOM, ANCHOR_RIGHT);
    let margin = Margin::new(
        BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        -BTN_WIDTH / 2 + WND_WIDTH / 5,
        -BTN_HEIGHT / 2 - WND_HEIGHT * 3 / 10,
        BTN_WIDTH / 2 + WND_WIDTH / 5
    );
    let no_btn = (
        UiObjectBuilder::new(
            Some("NoButton"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(NO_BTN_COLOR)
        .with_global_translation(BTN_TRANSLATION)
        .build(device), 
        TextBuilder::new(
            Some("NoButtonText"), 
            font, 
            script.get(ScriptTags::InGameGiveUpCancelButton)?, 
            text_brush
        )
        .with_anchor(anchor)
        .with_margin(margin)
        .with_color(TEXT_COLOR)
        .with_translation(TEXT_TRANSLATION)
        .build(device, queue)
    );

    return Ok([
            (ExitWndButton::Yes, yes_btn), 
            (ExitWndButton::No, no_btn),
        ]
        .into_iter()
        .collect()
    )
}

/// #### 한국어 </br>
/// 설정 창의 배경 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a background interface for the settings window. </br>
/// 
fn create_setting_windows(
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    window_texture_view: &wgpu::TextureView, 
    sub_window_texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> Vec<UiObject> {
    let background = UiObjectBuilder::new(
        Some("SettingBackground"), 
        tex_sampler, 
        window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(300, -400, -300, 400))
    .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.9))
    .build(device);

    let item0 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(204, -368, 108, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    let item1 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(76, -368, -20, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    let item2 = UiObjectBuilder::new(
        Some("SettingSubBackground"), 
        tex_sampler, 
        sub_window_texture_view, 
        ui_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-52, -368, -204, 368))
    .with_color(Vec4::new(222.0 / 255.0, 226.0 / 255.0, 230.0 / 255.0, 1.0))
    .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_global_translation(Vec3::new(0.0, 0.0, 0.8))
    .build(device);

    return vec![
        background, 
        item0, 
        item1, 
        item2
    ];
}

/// #### 한국어 </br>
/// 설정 창의 타이틀 텍스트들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates title texts for the settings window. </br>
/// 
fn create_setting_window_titles(
    nexon_lv2_gothic_medium: &FontArc, 
    nexon_lv2_gothic_bold: &FontArc, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    text_brush: &TextBrush
) -> AppResult<Vec<Text>> {
    let main_title = TextBuilder::new(
        Some("SettingTitle"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(292, -368, 244, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item0_title = TextBuilder::new(
        Some("SettingItem0Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingLanguageOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(236, -368, 204, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item0_sub_title = TextBuilder::new(
        Some("SettingItem0SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingLanguageOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(204, -368, 172, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item1_title = TextBuilder::new(
        Some("SettingItem1Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingResolutionOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(108, -368, 76, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item1_sub_title = TextBuilder::new(
        Some("SettingItem1SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingResolutionOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(76, -368, 44, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item2_title = TextBuilder::new(
        Some("SettingItem2Title"), 
        nexon_lv2_gothic_bold, 
        script.get(ScriptTags::SettingVolumeOptionTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-20, -368, -52, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    let item2_sub_title = TextBuilder::new(
        Some("SettingItem2SubTitle"), 
        nexon_lv2_gothic_medium, 
        script.get(ScriptTags::SettingVolumeOptionSubTitle)?, 
        text_brush
    )
    .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
    .with_margin(Margin::new(-52, -368, -84, 368))
    .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
    .with_scale(Vec3::new(0.0, 0.0, 0.0))
    .with_translation(Vec3::new(0.0, 0.0, 0.75))
    .build(device, queue);

    return Ok(vec![
        main_title, 
        item0_title, 
        item0_sub_title, 
        item1_title, 
        item1_sub_title, 
        item2_title, 
        item2_sub_title, 
    ]);
}

/// #### 한국어 </br>
/// 설정 창의 언어 선택 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create language selection buttons in the setting window. </br>
/// 
pub(super) fn create_setting_languages(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> HashMap<Language, (UiObject, Text)> {
    const TOP: i32 = 164;
    const LEFT: i32 = -344;
    const HEIGHT: i32 = 36;
    const WIDTH: i32 = HEIGHT * 3;
    const GAP: i32 = 8;

    let mut left = LEFT;
    let mut languages = HashMap::new();
    const LANGUAGES: [(Language, &'static str); 1] = [
        (Language::Korean, "한국어"), 
    ];

    for (language, text) in LANGUAGES {
        languages.insert(
            language, 
            (
                UiObjectBuilder::new(
                    Some(&format!("{}_Button", text)), 
                    tex_sampler, 
                    texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDTH))
                .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
                .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
                .build(device), 
                TextBuilder::new(
                    Some(&format!("{}_ButtonText", text)), 
                    font, 
                    text, 
                    text_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDTH))
                .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.4))
                .build(device, queue)
            )
        );

        left += GAP + WIDTH;
    }

    return languages;
}

/// #### 한국어 </br>
/// 설정 창의 해상도 선택 버튼들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create resolution selection buttons in the setting window. </br>
/// 
pub(super) fn create_setting_resolutions(
    font: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> HashMap<Resolution, (UiObject, Text)> {
    const TOP: i32 = 36;
    const LEFT: i32 = -344;
    const HEIGHT: i32 = 36;
    const WIDHT: i32 = HEIGHT * 3; 
    const GAP: i32 = 8;

    let mut left = LEFT;
    let mut resolutions = HashMap::new();
    const RESOLUTIONS: [(Resolution, &'static str); 6] = [
        (Resolution::W800H600, "800x600"),
        (Resolution::W1024H768, "1024x768"), 
        (Resolution::W1152H864, "1152x864"), 
        (Resolution::W1280H960, "1280x960"), 
        (Resolution::W1400H1050, "1400x1050"), 
        (Resolution::W1600H1200, "1600x1200"), 
    ];

    for (resolution, text) in RESOLUTIONS {
        resolutions.insert(
            resolution, 
            (
                UiObjectBuilder::new(
                    Some(&format!("{}_Button", text)), 
                    tex_sampler, 
                    texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDHT))
                .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
                .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
                .build(device),
                TextBuilder::new(
                    Some(&format!("{}_ButtonText", text)), 
                    font, 
                    text, 
                    text_brush
                )
                .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
                .with_margin(Margin::new(TOP, left, TOP - HEIGHT, left + WIDHT))
                .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.4))
                .build(device, queue)
            )
        );

        left += GAP + WIDHT;
    }

    return resolutions;
}

/// #### 한국어 </br>
/// 돌아가기 버튼을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a return button. </br>
/// 
#[inline]
pub(super) fn create_setting_return_button(
    font: &FontArc, 
    script: &Script,
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<(UiObject, Text)> {
    return Ok((
        UiObjectBuilder::new(
            Some("SettingReturnButton"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-220, 224, -268, 368))
        .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
        .build(device), 
        TextBuilder::new(
            Some("SettingReturnButtonText"), 
            font, 
            script.get(ScriptTags::SettingReturnButton)?, 
            text_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-220, 224, -268, 368))
        .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
        .with_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_translation(Vec3::new(0.0, 0.0, 0.4))
        .build(device, queue)
    ))
}

/// #### 한국어 </br>
/// 설정 창 볼륨 조절 인터페이스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a settings window volume control interface. </br>
/// 
pub(super) fn create_setting_volume_background(
    font: &FontArc, 
    script: &Script, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> AppResult<HashMap<VolumeOptions, (UiObject, Text)>> {
    let mut backgrounds = HashMap::new();
    backgrounds.insert(
        VolumeOptions::Background, 
        (
            UiObjectBuilder::new(
                Some("BackgroundVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-96, SETTING_VOLUME_RANGE_MIN, -104, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("BackgroundVolumeText"), 
                font, 
                script.get(ScriptTags::BackgroundVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-84, -368, -116, -240))
            .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
            .with_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.4))
            .build(device, queue)
        )
    );

    backgrounds.insert(
        VolumeOptions::Effect, 
        (
            UiObjectBuilder::new(
                Some("EffectVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-128, SETTING_VOLUME_RANGE_MIN, -136, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("EffectVolumeText"), 
                font, 
                script.get(ScriptTags::EffectVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-116, -368, -148, -240))
            .with_color(Vec4::new(0.0, 0.0, 0.0, 1.0))
            .with_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.4))
            .build(device, queue)
        )
    );

    backgrounds.insert(
        VolumeOptions::Voice, 
        (
            UiObjectBuilder::new(
                Some("VoiceVolume"), 
                tex_sampler, 
                texture_view, 
                ui_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-160, SETTING_VOLUME_RANGE_MIN, -168, SETTING_VOLUME_RANGE_MAX))
            .with_color(Vec4::new(187.0 / 255.0, 239.0 / 255.0, 249.0 / 255.0, 1.0))
            .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
            .with_global_translation(Vec3::new(0.0, 0.0, 0.5))
            .build(device), 
            TextBuilder::new(
                Some("VoiceVolumeText"), 
                font, 
                script.get(ScriptTags::VoiceVolume)?, 
                text_brush
            )
            .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
            .with_margin(Margin::new(-148, -368, -180, -240))
            .build(device, queue)
        )
    );

    return Ok(backgrounds);
}

/// #### 한국어 </br>
/// 설정 창의 볼륨 조절 막대기를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a volume control bar in the setting window. </br>
/// 
pub(super) fn create_setting_volume_bar(
    settings: &Settings, 
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_view: &wgpu::TextureView, 
    ui_brush: &UiBrush
) -> HashMap<VolumeOptions, UiObject> {
    const RANGE: i32 = SETTING_VOLUME_RANGE_MAX - SETTING_VOLUME_RANGE_MIN;
    let mut bar = HashMap::new();

    let delta = RANGE as f32 * settings.background_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Background, 
        UiObjectBuilder::new(
            Some("BackgroundVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-90, pos - VOLUME_BAR_WIDTH / 2, -110, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    let delta = RANGE as f32 * settings.effect_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Effect, 
        UiObjectBuilder::new(
            Some("EffectVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-122, pos - VOLUME_BAR_WIDTH / 2, -142, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    let delta = RANGE as f32 * settings.voice_volume.norm().min(1.0);
    let pos = SETTING_VOLUME_RANGE_MIN + delta as i32;
    bar.insert(
        VolumeOptions::Voice, 
        UiObjectBuilder::new(
            Some("VoiceVolumeBar"), 
            tex_sampler, 
            texture_view, 
            ui_brush
        )
        .with_anchor(Anchor::new(0.5, 0.5, 0.5, 0.5))
        .with_margin(Margin::new(-154, pos - VOLUME_BAR_WIDTH / 2, -174, pos + VOLUME_BAR_WIDTH / 2))
        .with_color(Vec4::new(234.0 / 255.0, 250.0 / 255.0, 253.0 / 255.0, 1.0))
        .with_global_scale(Vec3::new(0.0, 0.0, 0.0))
        .with_global_translation(Vec3::new(0.0, 0.0, 0.3))
        .build(device)
    );

    return bar;
}
