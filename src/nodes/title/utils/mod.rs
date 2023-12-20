mod buttons;
mod sprite;
mod window;

pub use buttons::*;
pub use sprite::*;
pub use window::*;

use ab_glyph::Font;

use crate::{
    assets::bundle::AssetBundle, 
    components::{
        text2d::brush::Text2dBrush,
        ui::brush::UiBrush,
        camera::GameCamera,
        transform::{Transform, Projection},
        script::Script,
        sprite::SpriteBrush, 
    },
    nodes::{
        path, 
        title::TitleScene,
        title::state::TitleState,  
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



/// #### 한국어 </br>
/// 카메라를 초기 상태로 재설정 합니다. </br>
/// 
/// #### English (Translation) </br>
/// Reset the camera to its initial state. </br>
/// 
pub fn reset_camera(camera: &GameCamera, queue: &wgpu::Queue) {
    camera.update(queue, |data| {
        data.transform = Transform::new();
        data.projection = Projection::new_ortho(
            MENU_TOP, 
            MENU_LEFT, 
            MENU_BOTTOM, 
            MENU_RIGHT, 
            0.0, 
            1000.0
        );
    });
}



pub fn create_title_scene<F: Font>(
    nexon_lv2_gothic_medium: &F, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    tex_sampler: &wgpu::Sampler, 
    script: &Script, 
    ui_brush: &UiBrush, 
    text_brush: &Text2dBrush, 
    sprite_brush: &SpriteBrush, 
    asset_bundle: &AssetBundle
) -> AppResult<TitleScene> {
    // (한국어) `dds`이미지 파일로부터 배경 텍스처를 생성합니다.
    // (English Translation) Create a background texture from a `dds`image file. 
    let texture = asset_bundle.get(path::BACKGROUND_TEXTURE_PATH)?  
    .read(&DdsTextureDecoder {
        name: Some("Background"),
        size: wgpu::Extent3d {
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
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
    let background_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        },
    );

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::BACKGROUND_TEXTURE_PATH);

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
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let start_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor::default()
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
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let setting_btn_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

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
            format: wgpu::TextureFormat::Bgra8Unorm, 
            mip_level_count: 11,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[], 
            device,
            queue, 
        })?;
    let exit_btn_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

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
    let texture = asset_bundle.get(path::TITLE_BUTTON_RETURN_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("ReturnButton"),
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
    let return_btn_texture_view = texture.create_view(
        &wgpu::TextureViewDescriptor::default()
    );

    // (한국어) 사용을 완료한 에셋을 해제합니다.
    // (English Translation) Release assets that have been used. 
    asset_bundle.release(path::TITLE_BUTTON_RETURN_TEXTURE_PATH);


    // (한국어) 시스템 버튼들을 생성합니다.
    // (English Translation) Create a system buttons.
    let texture_views = SystemButtonTextureViews {
        return_btn_texture_view: &return_btn_texture_view,  
    };
    let system_buttons = create_system_buttons(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush, 
    )?;


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
    let window_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::WINDOW_RATIO_4_3_TEXTURE_PATH);


    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::BUTTON_MEDIUM_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("Button"),
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
    let btn_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::BUTTON_MEDIUM_TEXTURE_PATH);

    // (한국어) `dds`이미지 파일로부터 버튼 텍스처를 생성합니다.
    // (English Translation) Create a button texture from a `dds`image file. 
    let texture = asset_bundle.get(path::BUTTON_WIDE_TEXTURE_PATH)?
        .read(&DdsTextureDecoder {
            name: Some("General"),
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
    let wide_btn_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
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
    let texture_views = SettingWindowTextureView {
        window_texture_view: &window_texture_view,
        store_btn_texture_view: &btn_texture_view,
        exit_btn_texture_view: &btn_texture_view, 
    };
    let setting_window = create_setting_window(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush
    )?;


    let texture_views = StageWindowTextureView {
        window_texture_view: &window_texture_view,
        enter_btn_texture_view: &wide_btn_texture_view, 
    };
    let stage_window = create_stage_window(
        nexon_lv2_gothic_medium, 
        device, 
        queue, 
        tex_sampler, 
        texture_views, 
        script, 
        ui_brush, 
        text_brush
    )?;

    return Ok(TitleScene {
        light_timer: 0.0, 
        elapsed_time: 0.0, 
        state: TitleState::Enter,
        background, 
        sprites,
        menu_buttons, 
        system_buttons, 
        exit_msg_box, 
        setting_window, 
        stage_window, 
    })
}
