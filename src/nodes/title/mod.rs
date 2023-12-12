pub mod res;
mod state;
mod ty;

use std::sync::Arc;

use glam::Vec3;
use winit::event::Event;
use rodio::{Sink, Source, OutputStreamHandle};

use crate::{
    game_err,
    assets::bundle::AssetBundle,
    components::{
        sprite::brush::SpriteBrush,
        text::{
            font::FontSet,
            brush::TextBrush,
        },
        ui::brush::UiBrush,
        camera::GameCamera, 
        sound::SoundDecoder,
        script::{Script, ScriptTags},
        transform::{Projection, Orthographic},
        user::Settings, 
    },
    nodes::{path, consts},
    render::texture::ImageDecoder,
    scene::{node::SceneNode, state::SceneState},
    system::{
        error::{AppResult, GameError},
        event::AppEvent,
        shared::Shared,
    },
};

pub const DEF_PROJECTION: Orthographic = Orthographic {
    top: 4.5 * consts::PIXEL_PER_METER,
    left: -8.0 * consts::PIXEL_PER_METER,
    bottom: -4.5 * consts::PIXEL_PER_METER,
    right: 8.0 * consts::PIXEL_PER_METER,
    near: 0.0 * consts::PIXEL_PER_METER,
    far: 1000.0 * consts::PIXEL_PER_METER,
};

pub const FOCUSED_PROJECTIONS: [Orthographic; 4] = [
    FOCUSED_ON_YUZU,
    FOCUSED_ON_ARIS,
    FOCUSED_ON_MOMOI,
    FOCUSED_ON_MIDORI
];

const FOCUSED_ON_YUZU: Orthographic = Orthographic {
    top: 1.625 * consts::PIXEL_PER_METER,
    left: 0.2 * consts::PIXEL_PER_METER,
    bottom: -0.625 * consts::PIXEL_PER_METER,
    right: 4.2 * consts::PIXEL_PER_METER,
    near: 0.0 * consts::PIXEL_PER_METER,
    far: 1000.0 * consts::PIXEL_PER_METER,
};

const FOCUSED_ON_ARIS: Orthographic = Orthographic {
    top: 1.125 * consts::PIXEL_PER_METER,
    left: -3.0 * consts::PIXEL_PER_METER,
    bottom: -1.125 * consts::PIXEL_PER_METER,
    right: 1.0 * consts::PIXEL_PER_METER,
    near: 0.0 * consts::PIXEL_PER_METER,
    far: 1000.0 * consts::PIXEL_PER_METER,
};

const FOCUSED_ON_MOMOI: Orthographic = Orthographic {
    top: -0.575 * consts::PIXEL_PER_METER,
    left: -6.0 * consts::PIXEL_PER_METER,
    bottom: -2.825 * consts::PIXEL_PER_METER,
    right: -2.0 * consts::PIXEL_PER_METER,
    near: 0.0 * consts::PIXEL_PER_METER,
    far: 1000.0 * consts::PIXEL_PER_METER,
};

const FOCUSED_ON_MIDORI: Orthographic = Orthographic {
    top: -0.575 * consts::PIXEL_PER_METER,
    left: 0.0 * consts::PIXEL_PER_METER,
    bottom: -2.825 * consts::PIXEL_PER_METER,
    right: 4.0 * consts::PIXEL_PER_METER,
    near: 0.0 * consts::PIXEL_PER_METER,
    far: 1000.0 * consts::PIXEL_PER_METER,
};

pub const MENU_CAMERA_POS: Vec3 = Vec3::new(0.0 * consts::PIXEL_PER_METER, -4.5 * consts::PIXEL_PER_METER, 0.0);
pub const STAGE_CAMERA_POS: Vec3 = Vec3::new(0.0 * consts::PIXEL_PER_METER, 3.5 * consts::PIXEL_PER_METER, 0.0);


/// #### 한국어 </br>
/// 다른 게임 장면에서 `Title` 게임 장면으로 <b>진입</b>하는 게임 장면입니다. </br>
/// `Title` 게임 장면을 초기화 합니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene that <b>enters</b> the `Title` game scene from another game scene.
/// Initializes the `Title` game scene. </br>
/// 
#[derive(Debug)]
pub struct EnterTitleScene {
    exit_window: Option<ty::ExitWindow>,
    stage_window: Option<ty::StageWindow>,
    setting_window: Option<ty::SettingWindow>,
    background: Option<ty::Backgrounds>,
    system: Option<ty::SystemButtons>,
    sprite: Option<ty::SpriteButtons>,
    menu: Option<ty::MenuButtons>,
}

impl Default for EnterTitleScene {
    #[inline]
    fn default() -> Self {
        Self { 
            exit_window: None,
            stage_window: None, 
            setting_window: None,
            background: None, 
            system: None, 
            sprite: None, 
            menu: None 
        }
    }
}

impl EnterTitleScene {
    /// #### 한국어 </br>
    /// `EnterTitle` 게임 장면에서 사용되는 카메라를 재설정합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Resets the camera used in game scene. </br>
    /// 
    fn reset_camera(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let queue = shared.pop::<Arc<wgpu::Queue>>().unwrap();
        let mut camera = shared.pop::<GameCamera>().unwrap();

        // (한국어) 카메라를 초기화 합니다.
        // (English Translation) Initialize camera.
        camera.transform.set_position(MENU_CAMERA_POS);
        camera.projection = Projection::Orthographic(DEF_PROJECTION);
        camera.update_buffer(&queue);

        // (한국어) 사용을 완료한 에셋을 반환합니다.
        // (English Translation) Returns assets that have been used.
        shared.push(camera);
        shared.push(queue);

        Ok(())
    }

    /// #### 한국어 </br>
    /// `EnterTitle` 게임 장면을 설정합니다. </br>
    /// 
    /// #### English (Translaton) </br>
    /// set up the `EnterTitle` game scene. </br>
    /// 
    fn setup_scene(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
        let sprite_brush = shared.get::<Arc<SpriteBrush>>().unwrap();
        let asset_bundle = shared.get::<AssetBundle>().unwrap();
        let font_set = shared.get::<FontSet>().unwrap();
        let script = shared.get::<Script>().unwrap();

        // (한국어) `Title` 게임 장면에 사용되는 요소들을 생성합니다.
        // (English Translation) Creates elements used in `Title` game scene.
        let (
            exit_window, 
            stage_window,
            setting_window,
            background,
            system,
            sprite,
            menu,
        ) = create_title_scene_elements(
            device, 
            queue, 
            tex_sampler, 
            ui_brush, 
            text_brush, 
            sprite_brush, 
            asset_bundle, 
            font_set,
            script
        )?;

        self.exit_window = Some(exit_window);
        self.stage_window = Some(stage_window);
        self.setting_window = Some(setting_window);
        self.background = Some(background);
        self.system = Some(system);
        self.sprite = Some(sprite);
        self.menu = Some(menu);

        Ok(())
    }
}

impl SceneNode for EnterTitleScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        self.reset_camera(shared)?;
        self.setup_scene(shared)?;
        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // (한국어) 다음 게임 장면으로 변경합니다.
            // (English Translation) Changes to the next game scene.
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(
                TitleScene {
                    state: state::TitleState::Entry,
                    elapsed_time: 0.0,
                    exit_window: self.exit_window.take().unwrap(),
                    stage_window: self.stage_window.take().unwrap(),
                    setting_window: self.setting_window.take().unwrap(),
                    background: self.background.take().unwrap(),
                    system: self.system.take().unwrap(),
                    sprite: self.sprite.take().unwrap(),
                    menu: self.menu.take().unwrap(),
                }
            ));  
        });

        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체 가져오기.
        // (English Translation) Get shared object to use.
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let camera = shared.get::<GameCamera>().unwrap();


        // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
        // (English Translation) Wait until the previous operation is finished.
        device.poll(wgpu::Maintain::Wait);

        // (한국어) 다음 프레임을 가져옵니다.
        // (English Translation) Get the next frame.
        let frame = surface.get_current_texture()
            .map_err(|err| game_err!(
                "Failed to get next frame",
                "Failed to get next frame for the following reasons: {}",
                err.to_string()
            ))?;

        // (한국어) 프레임 버퍼의 텍스쳐 뷰를 생성합니다.
        // (English Translation) Creates a texture view of the framebuffer.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RenderPass(EntryTitleScene)"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            camera.bind(&mut rpass);
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}




/// #### 한국어 </br>
/// 게임을 설정하거나, 시작하는 게임 장면입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene that sets up or starts the game. </br>
/// 
#[derive(Debug)]
pub struct TitleScene {
    state: state::TitleState,
    elapsed_time: f64,
    exit_window: ty::ExitWindow,
    stage_window: ty::StageWindow,
    setting_window: ty::SettingWindow,
    background: ty::Backgrounds,
    system: ty::SystemButtons,
    sprite: ty::SpriteButtons,
    menu: ty::MenuButtons,
}

impl SceneNode for TitleScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        play_background_sound(shared)?;
        Ok(())
    }

    fn exit(&mut self, shared: &mut Shared) -> AppResult<()> {
        clear_background_sound(shared)?;
        Ok(())
    }

    #[inline]
    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        state::HANDLE_EVENTS[self.state as usize](self, shared, event)
    }

    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        state::UPDATES[self.state as usize](self, shared, total_time, elapsed_time)
    }

    #[inline]
    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        state::DRAWS[self.state as usize](self, shared)
    }
}



/// #### 한국어 </br>
/// 배경 음악을 재생합니다. </br>
/// 
/// #### English (Translation) </br>
/// Play background music. </br>
/// 
fn play_background_sound(shared: &mut Shared) -> AppResult<()> {
    use crate::components::sound::create_sink;

    // (한국어) 사용할 공유 객체 가져오기.
    // (English Translation) Get shared object to use.
    let stream_handle = shared.get::<OutputStreamHandle>().unwrap();
    let asset_bundle = shared.get::<AssetBundle>().unwrap();
    let settings = shared.get::<Settings>().unwrap();

    // (한국어) BGM을 재생합니다.
    // (English Translation) Play BGM.
    let source = asset_bundle.get(path::title::BGM_SOUND_PATH)?
        .read(&SoundDecoder)?
        .repeat_infinite();
    let sink = create_sink(stream_handle)?;
    sink.set_volume(settings.background_volume.get_norm());
    sink.append(source);
    
    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::title::BGM_SOUND_PATH);
    
    // (한국어) 배경 음악을 공유 객체에 등록합니다.
    // (English Translation) Register background music to a shared object.
    shared.push(sink);

    Ok(())
}


/// #### 한국어 </br>
/// 재생중인 음악을 지웁니다. </br>
/// 
/// #### English (Translation) </br>
/// Clears the music being played. </br>
/// 
#[inline]
fn clear_background_sound(shared: &mut Shared) -> AppResult<()> {
    Ok(shared.pop::<Sink>().unwrap().detach())
}


/// #### 한국어 </br>
/// `Title` 게임 장면에 사용되는 요소들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create elements used in `Title` game scene. </br>
/// 
fn create_title_scene_elements(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex_sampler: &wgpu::Sampler,
    ui_brush: &UiBrush,
    text_brush: &TextBrush,
    sprite_brush: &SpriteBrush,
    asset_bundle: &AssetBundle,
    font_set: &FontSet,
    script: &Script,
) -> AppResult<(
    ty::ExitWindow,
    ty::StageWindow, 
    ty::SettingWindow,
    ty::Backgrounds, 
    ty::SystemButtons, 
    ty::SpriteButtons, 
    ty::MenuButtons
)> {
    // (한국어) 이미지를 로드하고, 텍스처를 생성합니다.
    // (English Translation) Load the image and create the texture.
    let texture = asset_bundle.get(path::title::BACKGROUND_PATH)?
        .read(&ImageDecoder::new(Some("Background"), device, queue))?;
    let background_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::title::CABINET_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Cabinet"), device, queue))?;
    let cabinet_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::title::SOFA_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Sofa"), device, queue))?;
    let sofa_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let background_descs = [
        (ty::BackgroundTags::Background, ty::BackgroundDesc { texture_view: &background_texture_view }),
        (ty::BackgroundTags::Cabinet, ty::BackgroundDesc { texture_view: &cabinet_texture_view }),
        (ty::BackgroundTags::Sofa, ty::BackgroundDesc { texture_view: &sofa_texture_view }),
    ].into_iter().collect();

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::title::BACKGROUND_PATH);
    asset_bundle.release(path::title::CABINET_TEXTURE_PATH);
    asset_bundle.release(path::title::SOFA_TEXTURE_PATH);


    let texture = asset_bundle.get(path::title::YUZU_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Yuzu"), device, queue))?;
    let yuzu_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::title::ARIS_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Aris"), device, queue))?;
    let aris_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::title::MOMOI_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Momoi"), device, queue))?;
    let momoi_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::title::MIDORI_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Midori"), device, queue))?;
    let midori_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sprite_descs = [
        (ty::SpriteButtonTags::Yuzu, ty::SpriteButtonDesc { texture_view: &yuzu_texture_view }),
        (ty::SpriteButtonTags::Aris, ty::SpriteButtonDesc { texture_view: &aris_texture_view }),
        (ty::SpriteButtonTags::Momoi, ty::SpriteButtonDesc { texture_view: &momoi_texture_view }),
        (ty::SpriteButtonTags::Midori, ty::SpriteButtonDesc { texture_view: &midori_texture_view })
    ].into_iter().collect();

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::title::YUZU_TEXTURE_PATH);
    asset_bundle.release(path::title::ARIS_TEXTURE_PATH);
    asset_bundle.release(path::title::MOMOI_TEXTURE_PATH);
    asset_bundle.release(path::title::MIDORI_TEXTURE_PATH);



    let texture = asset_bundle.get(path::sys::WINDOW_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Window(Big)"), device, queue))?;
    let window_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_SMALL_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Small)"), device, queue))?;
    let btn_small_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_SMALL_EX_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(SmallEx)"), device, queue))?;
    let btn_small_ex_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_START_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Start)"), device, queue))?;
    let btn_start_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_SETTING_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Setting)"), device, queue))?;
    let btn_setting_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_EXIT_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Exit)"), device, queue))?;
    let btn_exit_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_RETURN_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Return)"), device, queue))?;
    let btn_return_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let texture = asset_bundle.get(path::sys::BUTTON_ENTER_TEXTURE_PATH)?
        .read(&ImageDecoder::new(Some("Button(Enter)"), device, queue))?;
    let btn_enter_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let system_descs = [
        (ty::SystemButtonTags::ReturnButton, ty::SystemButtonDesc {
            text: vec![],
            texture_view: &btn_return_texture_view,
        }),
    ].into_iter().collect();
    let menu_descs = [
        (ty::MenuButtonTags::StartButton, ty::MenuButtonDesc {
            text: vec![script.get(ScriptTags::StartMenu)?],
            texture_view: &btn_start_texture_view,
        }),
        (ty::MenuButtonTags::SettingButton, ty::MenuButtonDesc {
            text: vec![script.get(ScriptTags::SettingMenu)?],
            texture_view: &btn_setting_texture_view,
        }),
        (ty::MenuButtonTags::ExitButton, ty::MenuButtonDesc {
            text: vec![script.get(ScriptTags::ExitMenu)?],
            texture_view: &btn_exit_texture_view,
        }),
    ].into_iter().collect();
    let exit_window_descs_bg = [
        (ty::ExitWindowTags::Window, ty::ExitWindowDesc {
            text: vec![script.get(ScriptTags::ExitMessage)?],
            texture_view:&window_texture_view,
        }),
        (ty::ExitWindowTags::Okay, ty::ExitWindowDesc {
            text: vec![script.get(ScriptTags::Exit)?],
            texture_view: &btn_small_ex_texture_view,
        }),
        (ty::ExitWindowTags::Cancel, ty::ExitWindowDesc {
            text: vec![script.get(ScriptTags::NoExit)?],
            texture_view: &btn_small_texture_view,
        }),
    ].into_iter().collect();
    let stage_window_descs = [
        (ty::StageWindowTags::Window, ty::StageWindowDesc {
            text: vec![],
            texture_view: &window_texture_view,
        }),
        (ty::StageWindowTags::Enter, ty::StageWindowDesc {
            text: vec![script.get(ScriptTags::EnterStage)?],
            texture_view: &btn_enter_texture_view
        }),
    ].into_iter().collect();
    let setting_window_descs = [
        (ty::SettingWindowTags::Window, ty::SettingWindowDesc {
            texts: vec![],
            texture_view: &window_texture_view,
        }),
        (ty::SettingWindowTags::SaveButton, ty::SettingWindowDesc {
            texts: vec![script.get(ScriptTags::Store)?],
            texture_view: &btn_small_ex_texture_view,
        }),
        (ty::SettingWindowTags::ExitButton, ty::SettingWindowDesc {
            texts: vec![script.get(ScriptTags::NoStore)?],
            texture_view: &btn_small_texture_view,
        })
    ].into_iter().collect();

    // (한국어) 사용을 완료한 에셋을 정리합니다.
    // (English Translation) Release assets that have been used.
    asset_bundle.release(path::sys::WINDOW_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_SMALL_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_SMALL_EX_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_START_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_SETTING_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_EXIT_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_RETURN_TEXTURE_PATH);
    asset_bundle.release(path::sys::BUTTON_ENTER_TEXTURE_PATH);

    // (한국어) 종료 윈도우를 생성합니다.
    // (English Translation) Create a exit window.
    let exit_window = ty::ExitWindow::new(
        font_set.get(path::FONT_MEDIUM_PATH).unwrap(),
        device,
        queue,
        tex_sampler,
        ui_brush,
        text_brush,
        exit_window_descs_bg
    );

    // (한국어) 스테이지 윈도우를 생성합니다.
    // (English Translation) Create stage window.
    let stage_window = ty::StageWindow::new(
        font_set.get(path::FONT_MEDIUM_PATH).unwrap(),
        device,
        queue,
        tex_sampler,
        ui_brush,
        text_brush,
        stage_window_descs
    );

    // (한국어) 설정 윈도우 배경을 생성합니다.
    // (English Translation) Create setting window background.
    let setting_window = ty::SettingWindow::new(
        font_set.get(path::FONT_MEDIUM_PATH).unwrap(), 
        device, 
        queue, 
        tex_sampler, 
        ui_brush, 
        text_brush, 
        setting_window_descs
    );

    // (한국어) 배경 스프라이트들을 생성합니다.
    // (English Translation) Create background sprites.
    let background = ty::Backgrounds::new(
        device, 
        tex_sampler, 
        sprite_brush, 
        background_descs
    );

    // (한국어) 시스템 버튼들을 생성합니다. 
    // (English Translation) Create system buttons.
    let system = ty::SystemButtons::new(
        font_set.get(path::FONT_MEDIUM_PATH).unwrap(),
        device,
        queue,
        tex_sampler,
        ui_brush,
        text_brush,
        system_descs
    );

    // (한국어) 스프라이트 버튼들을 생성합니다.
    // (English Translation) Create sprite buttons. 
    let sprite = ty::SpriteButtons::new(
        device,
        tex_sampler,
        sprite_brush,
        sprite_descs
    );

    // (한국어) 메뉴 버튼들을 생성합니다.
    // (English Translation) Create menu buttons.
    let menu = ty::MenuButtons::new(
        font_set.get(path::FONT_BLOD_PATH).unwrap(),
        device,
        queue,
        tex_sampler,
        ui_brush,
        text_brush,
        menu_descs
    );


    Ok((
        exit_window,
        stage_window,
        setting_window,
        background,
        system,
        sprite,
        menu,
    ))
}
