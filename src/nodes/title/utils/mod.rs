mod buttons;
mod sprite;
mod window;

pub use buttons::*;
pub use sprite::*;
pub use window::*;

use std::collections::HashMap;

use ab_glyph::FontArc;

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        ui::{UiBrush, UiObject, UiObjectBuilder},
        text::{Text, TextBrush, TextBuilder}, 
        script::Script,
        sprite::SpriteBrush, 
        anchor::Anchor, 
        player::Actor, 
        save::SaveData, 
        user::Settings, 
    },
    nodes::{
        path, 
        title::TitleScene,
        title::state::TitleState,  
        in_game::NUM_TILES, 
        consts::PIXEL_PER_METER, 
    },
    render::texture::DdsTextureDecoder,
    system::error::AppResult,
};



pub const MENU_TOP: f32 = (1.5 - 2.5) * PIXEL_PER_METER;
pub const MENU_LEFT: f32 = -2.0 * PIXEL_PER_METER;
pub const MENU_BOTTOM: f32 = (-1.5 - 2.5) * PIXEL_PER_METER;
pub const MENU_RIGHT: f32 = 2.0 * PIXEL_PER_METER;

pub const STAGE_TOP: f32 = (3.0 + 1.0) * PIXEL_PER_METER;
pub const STAGE_LEFT: f32 = -4.0 * PIXEL_PER_METER;
pub const STAGE_BOTTOM: f32 = (-3.0 + 1.0) * PIXEL_PER_METER;
pub const STAGE_RIGHT: f32 = 4.0 * PIXEL_PER_METER;



pub fn create_title_scene(
    save: &SaveData, 
    settings: &Settings, 
    nexon_lv2_gothic_medium: &FontArc, 
    nexon_lv2_gothic_bold: &FontArc, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    script: &Script, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush, 
    sprite_brush: &SpriteBrush, 
    texture_map: &HashMap<String, wgpu::Texture>, 
    asset_bundle: &AssetBundle
) -> AppResult<TitleScene> {
    // (한국어) `dds`이미지 파일로부터 배경 텍스처를 생성합니다.
    // (English Translation) Create a background texture from a `dds`image file. 
    let texture = asset_bundle.get(path::TITLE_BACKGROUND_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Background"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bc7RgbaUnorm,
        mip_level_count: 11,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
        device: &device,
        queue: &queue,
    })?;
    let background_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::TITLE_BACKGROUND_TEXTURE_PATH);

    // (한국어) 배경 스프라이트들을 생성합니다.
    // (English Translation) Create a background sprites. 
    let background = create_background_sprite(
        device,
        tex_sampler,
        &background_texture_view, 
        sprite_brush
    )?;



    // (한국어) `dds`이미지 파일로부터 `Aris` 텍스처를 생성합니다.
    // (English Translation) Create a `Momoi` texture from a `dds`image file. 
    let texture = asset_bundle.get(path::ARIS_STANDING_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Aris"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1412,
            depth_or_array_layers: 2,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8Unorm,
        mip_level_count: 11,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
        device: &device,
        queue: &queue,
    })?;
    let aris_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::ARIS_STANDING_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 `Momoi` 텍스처를 생성합니다.
    // (English Translation) Create a `Momoi` texture from a `dds`image file. 
    let texture = asset_bundle.get(path::MOMOI_STANDING_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Momoi"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1184,
            depth_or_array_layers: 2,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8Unorm,
        mip_level_count: 11,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
        device: &device,
        queue: &queue,
    })?;
    let momoi_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::MOMOI_STANDING_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 `Midori` 텍스처를 생성합니다.
    // (English Translation) Create a `Midori` texture from a `dds`image file. 
    let texture = asset_bundle.get(path::MIDORI_STANDING_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Midori"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1356,
            depth_or_array_layers: 2,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8Unorm,
        mip_level_count: 11,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
        device: &device,
        queue: &queue,
    })?;
    let midori_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::MIDORI_STANDING_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 `Yuzu` 텍스처를 생성합니다.
    // (English Translation) Create a `Yuzu` texture from a `dds`image file. 
    let texture = asset_bundle.get(path::YUZU_STANDING_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Yuzu"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1861,
            depth_or_array_layers: 2,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8Unorm,
        mip_level_count: 11,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
        device: &device,
        queue: &queue,
    })?;
    let yuzu_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::YUZU_STANDING_TEXTURE_PATH);

    // (한국어) 캐릭터 스프라이트를 생성합니다.
    // (English Translation) Create a character sprite. 
    let texture_views = CharactorSpriteTextureViews {
        aris_texture_view: &aris_texture_view,
        momoi_texture_view: &momoi_texture_view,
        midori_texture_view: &midori_texture_view,
        yuzu_texture_view: &yuzu_texture_view, 
    };
    let sprites = create_character_sprites(
        device, 
        tex_sampler, 
        texture_views, 
        sprite_brush
    )?;


    // (한국어) `dds`이미지 파일로부터 시작 버튼 텍스처를 생성합니다.
    // (English Translation) Create a start button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::TITLE_BUTTON_START_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("StartButton"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 160,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bc7RgbaUnorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let start_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::TITLE_BUTTON_START_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 설정 메뉴 텍스처를 생성합니다.
    // (English Translation) Create a setting menu texture from a `dds`image file. 
    let texture = asset_bundle.get(path::TITLE_BUTTON_SETTING_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("SettingMenu"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 160,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bc7RgbaUnorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let setting_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::TITLE_BUTTON_SETTING_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 종료 메뉴 텍스처를 생성합니다.
    // (English Translation) Create a setting exit texture from a `dds`image file. 
    let texture = asset_bundle.get(path::TITLE_BUTTON_EXIT_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("ExitMenu"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 160,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bc7RgbaUnorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let exit_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::TITLE_BUTTON_EXIT_TEXTURE_PATH);


    // (한국어) 메뉴 버튼들을 생성합니다.
    // (English Translation) Create a menu buttons.
    let texture_views = MenuButtonTextureViews {
        start_btn_texture_view: &start_btn_texture_view,
        setting_btn_texture_view: &setting_btn_texture_view,
        exit_btn_texture_view: &exit_btn_texture_view, 
    };
    let menu_buttons = create_menu_buttons(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush, 
    )?;


    // (한국어) `dds` 이미지 파일로부터 되돌아가기 버튼 텍스처를 생성합니다.
    // (English Translation) Create a return button texture from the `dds` image file. 
    let texture = asset_bundle.get(path::BUTTON_RETURN_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("ReturnButton"),
            size: wgpu::Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 8,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue, 
        })?;
    let return_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용을 완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::BUTTON_RETURN_TEXTURE_PATH);


    // (한국어) 시스템 버튼들을 생성합니다.
    // (English Translation) Create a system buttons.
    let texture_views = SystemButtonTextureViews {
        return_btn_texture_view: &return_btn_texture_view,  
    };
    let return_button = create_system_buttons(
        device, 
        tex_sampler, 
        texture_views, 
        ui_brush, 
    );


    // (한국어) `dds` 이미지 파일로부터 되돌아가기 버튼 텍스처를 생성합니다.
    // (English Translation) Create a return button texture from the `dds` image file. 
    let texture = asset_bundle.get(path::BUTTON_INFO_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("InfoButton"),
            size: wgpu::Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 8,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue, 
        })?;
    let info_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );

    // (한국어) 사용을 완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::BUTTON_INFO_TEXTURE_PATH);

    let info_button = create_information_button(
        device, 
        tex_sampler, 
        &info_btn_texture_view, 
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
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
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
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
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
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 10,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });
    
    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::BUTTON_MEDIUM_TEXTURE_PATH);

    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::BUTTON_WIDE_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("WideButton"),
            size: wgpu::Extent3d {
                width:1024,
                height:192,
                depth_or_array_layers:1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 11,
            sample_count:1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device: &device,
            queue: &queue,
        })?;
    let wide_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        });
    
    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::BUTTON_WIDE_TEXTURE_PATH);


    // (한국어) 종료 메시지 박스를 생성합니다. 
    // (English Translation) Create a exit message box. 
    let texture_views = ExitMsgBoxTextureViews {
        window_texture_view: &window_texture_view,
        yes_btn_texture_view: &btn_texture_view, 
        no_btn_texture_view: &btn_texture_view, 
    };
    let exit_msg_box = create_exit_message_box(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush
    )?;


    // (한국어) 설정 윈도우를 생성합니다.
    // (English Translation) Create a setting window.
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
    let setting_languages = create_setting_languages(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        &btn_texture_view, 
        ui_brush, 
        text_brush
    );
    let setting_resolutions = create_setting_resolutions(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        &btn_texture_view, 
        ui_brush, 
        text_brush
    );
    let setting_return_button = create_setting_return_button(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &btn_texture_view, 
        ui_brush, 
        text_brush
    )?;

    let texture = texture_map.get(path::DUMMY_TEXTURE_PATH)
        .expect("Registered texture not found!");
    let dummy_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            ..Default::default()
        }
    );
    let setting_volume_background = create_setting_volume_background(
        nexon_lv2_gothic_medium, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &dummy_texture_view, 
        ui_brush, 
        text_brush
    )?;
    let setting_volume_bar = create_setting_volume_bar(
        settings, 
        device, 
        tex_sampler, 
        &dummy_texture_view, 
        ui_brush
    );


    let texture_views = StageWindowTextureView {
        window_texture_view: &window_texture_view,
        enter_btn_texture_view: &wide_btn_texture_view, 
    };
    let (stage_window, stage_enter_button) = create_stage_window(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush
    )?;


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


    let star_texture = asset_bundle.get(path::STAR_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Star"),
            size: wgpu::Extent3d {
                width: 512,
                height: 256,
                depth_or_array_layers: 5,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 10,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let stage_images = create_stage_image(
        nexon_lv2_gothic_medium, 
        &save, 
        device, 
        queue, 
        tex_sampler, 
        &star_texture, 
        texture_map, 
        ui_brush, 
        text_brush
    );
    let stage_viewer_images = create_stage_viewer_images(
        &save, 
        device, 
        tex_sampler, 
        texture_map, 
        ui_brush
    );
    asset_bundle.release(path::STAR_TEXTURE_PATH);


    let tutorial_textture = asset_bundle.get(path::TUTORIAL_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Tutorial"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 512,
                depth_or_array_layers: 4,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bc7RgbaUnorm,
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            device,
            queue,
        })?;
    let tutorials = create_tutorial_windows(
        nexon_lv2_gothic_bold, 
        script, 
        device, 
        queue, 
        tex_sampler, 
        &tutorial_textture, 
        ui_brush, 
        text_brush
    )?;
    let (tutorial_prev_btn, tutorial_next_btn) = create_tutorial_buttons(
        device, 
        tex_sampler, 
        &return_btn_texture_view, 
        ui_brush
    );
    asset_bundle.release(path::TUTORIAL_TEXTURE_PATH);


    // (한국어) 로고 이미지 텍스처를 생성합니다.
    // (English Translation) Create a logo image texture. 
    let logo_texture = asset_bundle.get(path::LOGO_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Logo"), 
            size: wgpu::Extent3d {
                width: 512, 
                height: 512, 
                depth_or_array_layers: 1, 
            }, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Bc7RgbaUnorm, 
            mip_level_count: 10, 
            sample_count: 1, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device: &device, 
            queue: &queue
        })?;

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::LOGO_TEXTURE_PATH);

    let logo_texture_view = logo_texture.create_view(
        &wgpu::TextureViewDescriptor { 
            ..Default::default()
        }
    );
    let credit_button = create_credit_button(
        device, 
        tex_sampler, 
        &logo_texture_view, 
        ui_brush
    );




    return Ok(TitleScene {
        timer: 0.0, 
        duration: 0.0, 
        state: TitleState::Enter,
        foreground, 
        background, 
        sprites,
        menu_buttons, 
        credit_button, 
        return_button, 
        info_button, 
        exit_msg_box, 
        stage_window, 
        stage_enter_button, 
        stage_images, 
        stage_viewer_images, 
        setting_titles, 
        setting_windows, 
        setting_languages, 
        setting_resolutions, 
        setting_return_button, 
        setting_volume_background, 
        setting_volume_bar, 
        tutorial_prev_btn, 
        tutorial_next_btn, 
        tutorials, 
    })
}



/// #### 한국어 </br>
/// 게임 장면 전환에 사용되는 전경 사용자 인터페이스를 생성합니다.
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
    .with_color((1.0, 1.0, 1.0, 1.0).into())
    .build(device)
}

/// #### 한국어 </br>
/// 스테이지 이미지들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a stage images. </br>
/// 
fn create_stage_image(
    font: &FontArc, 
    save: &SaveData, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue,
    tex_sampler: &wgpu::Sampler, 
    star_texture: &wgpu::Texture, 
    texture_map: &HashMap<String, wgpu::Texture>, 
    ui_brush: &UiBrush, 
    text_brush: &TextBrush
) -> HashMap<Actor, (UiObject, UiObject, Text)> {
    const MAP: [(Actor, &'static str, &'static str); 4] = [
        (Actor::Aris, "Aris", path::ARIS_IMG_TEXTURE_PATH), 
        (Actor::Momoi, "Momoi", path::MOMOI_IMG_TEXTURE_PATH), 
        (Actor::Midori, "Midori", path::MIDORI_IMG_TEXTURE_PATH), 
        (Actor::Yuzu, "Yuzu", path::YUZU_IMG_TEXTURE_PATH)
    ];

    let mut stage_image = HashMap::new();
    for (actor, label, rel_path) in MAP {
        let percent = match actor {
            Actor::Aris => save.stage_aris,
            Actor::Momoi => save.stage_momoi, 
            Actor::Midori => save.stage_midori, 
            Actor::Yuzu => save.stage_yuzu
        } as f32 / NUM_TILES as f32 * 100.0;

        let stage_img_texture_view = if percent < 20.0 {
            let texture = texture_map.get(path::DEF_IMG_TEXTURE_PATH)
                .expect("Registered texture not found!");
            texture.create_view(
                &wgpu::TextureViewDescriptor {
                    ..Default::default()
                }
            )
        } else {
            let texture = texture_map.get(rel_path)
                .expect("Registered texture not found!");
            texture.create_view(
                &wgpu::TextureViewDescriptor {
                    base_array_layer: 
                        if 20.0 <= percent && percent < 50.0 {
                            0
                        } else if 50.0 <= percent && percent < 80.0 {
                            1
                        } else {
                            2
                        },
                    array_layer_count: Some(1),
                    dimension: Some(wgpu::TextureViewDimension::D2), 
                    ..Default::default()
                }
            )
        };

        let star_img_texture_view = star_texture.create_view(
            &wgpu::TextureViewDescriptor {
                base_array_layer: if percent < 20.0 {
                    0
                } else if 20.0 <= percent && percent < 50.0 {
                    1
                } else if 50.0 <= percent && percent < 80.0 {
                    2
                } else if 80.0 <= percent && percent < 100.0 {
                    3
                } else {
                    4
                },
                array_layer_count: Some(1), 
                dimension: Some(wgpu::TextureViewDimension::D2), 
                ..Default::default()
            }
        );

        stage_image.insert(
            actor, 
            (
                UiObjectBuilder::new(
                    Some(&format!("{}StageImage", label)), 
                    tex_sampler, 
                    &stage_img_texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(
                    1.0 - 0.05, 
                    0.5 - 0.225, 
                    1.0 - 0.35, 
                    0.5 - 0.225 + 0.225
                ))
                .with_color((1.0, 1.0, 1.0, 0.0).into())
                .with_global_translation((0.0, 0.0, 0.3).into())
                .build(device), 
                UiObjectBuilder::new(
                    Some(&format!("{}StageResult", label)), 
                    tex_sampler, 
                    &star_img_texture_view, 
                    ui_brush
                )
                .with_anchor(Anchor::new(
                    1.0 - 0.05, 
                    0.55, 
                    1.0 - 0.15, 
                    0.55 + 0.15))
                .with_color((1.0, 1.0, 1.0, 0.0).into())
                .with_global_translation((0.0, 0.0, 0.3).into())
                .build(device), 
                TextBuilder::new(
                    Some(&format!("{}StagePercentText", label)), 
                    font, 
                    &format!("{}%", percent.floor()), 
                    text_brush
                )
                .with_anchor(Anchor::new(
                    1.0 - 0.2, 
                    0.55, 
                    1.0 - 0.3, 
                    0.55 + 0.15
                ))
                .with_color((0.0, 0.0, 0.0, 0.0).into())
                .with_translation((0.0, 0.0, 0.3).into())
                .build(device, queue)
            )
        );
    }

    return stage_image;
}

/// #### 한국어 </br>
/// 스테이지 보기 이미지를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a stage view image. </br>
/// 
fn create_stage_viewer_images(
    save: &SaveData, 
    device: &wgpu::Device, 
    tex_sampler: &wgpu::Sampler, 
    texture_map: &HashMap<String, wgpu::Texture>, 
    ui_brush: &UiBrush
) -> HashMap<Actor, UiObject> {
    const MAP: [(Actor, &'static str, &'static str); 4] = [
        (Actor::Aris, "Aris", path::ARIS_IMG_TEXTURE_PATH), 
        (Actor::Momoi, "Momoi", path::MOMOI_IMG_TEXTURE_PATH), 
        (Actor::Midori, "Midori", path::MIDORI_IMG_TEXTURE_PATH), 
        (Actor::Yuzu, "Yuzu", path::YUZU_IMG_TEXTURE_PATH)
    ];

    let mut stage_viewer_images = HashMap::new();
    for (actor, label, rel_path) in MAP {
        let percent = match actor {
            Actor::Aris => save.stage_aris, 
            Actor::Momoi => save.stage_momoi, 
            Actor::Midori => save.stage_midori, 
            Actor::Yuzu => save.stage_yuzu
        } as f32 / NUM_TILES as f32 * 100.0;

        let stage_image_texture_view = match percent < 20.0 {
            true => {
                let texture = texture_map.get(path::DEF_IMG_TEXTURE_PATH)
                    .expect("Registered texture not found!");
                texture.create_view(
                    &wgpu::TextureViewDescriptor {
                        ..Default::default()
                    }
                )
            },
            false => {
                let texture = texture_map.get(rel_path)
                    .expect("Registered texture not found!");
                texture.create_view(
                    &wgpu::TextureViewDescriptor {
                        base_array_layer: if 20.0 <= percent && percent < 50.0 {
                            0
                        } else if 50.0 <= percent && percent < 80.0 {
                            1
                        } else {
                            2
                        },
                        array_layer_count: Some(1), 
                        dimension: Some(wgpu::TextureViewDimension::D2), 
                        ..Default::default()
                    }
                )
            }
        };

        stage_viewer_images.insert(
            actor, 
            UiObjectBuilder::new(
                Some(&format!("{}StageViewerImage", label)), 
                tex_sampler, 
                &stage_image_texture_view, 
                ui_brush
            ) 
            .with_anchor(Anchor::new(0.95, 0.2875, 0.05, 0.9625))
            .with_color((1.0, 1.0, 1.0, 0.0).into())
            .with_global_translation((0.0, 0.0, 0.1).into())
            .build(device)
        );
    }

    return stage_viewer_images;
}
