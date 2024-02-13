use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use glam::{Vec3, Vec4, Vec4Swizzles};
use ab_glyph::FontArc;
use winit::{
    keyboard::{KeyCode, PhysicalKey},
    event::{Event, WindowEvent, MouseButton}, 
    dpi::PhysicalPosition, 
};

use crate::components::collider2d::Collider2d;
use crate::{
    game_err, 
    assets::bundle::AssetBundle, 
    components::{
        ui::{UiObject, UiBrush, UiObjectBuilder}, 
        text::{Text, TextBrush, TextBuilder}, 
        camera::GameCamera, 
        anchor::Anchor, 
        margin::Margin, 
        save::SaveData, 
        interpolation, 
        sound, 
    }, 
    nodes::{
        path, 
        in_game::NUM_TILES,
    }, 
    render::{
        depth::DepthBuffer, 
        texture::DdsTextureDecoder, 
    }, 
    scene::{node::SceneNode, state::SceneState}, 
    system::{
        error::{AppResult, GameError}, 
        event::AppEvent, 
        shared::Shared, 
    },
};



/// #### 한국어 </br>
/// 게임 크레딧 게임 장면을 로딩하는 게임 장면 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene loading the game credit game scene. </br>
/// 
#[derive(Debug)]
pub struct CreditLoadingScene {
    loading_text: Option<Text>, 
    loading: Option<JoinHandle<AppResult<CreditScene>>>, 
}

impl Default for CreditLoadingScene {
    #[inline]
    fn default() -> Self {
        Self { 
            loading_text: None, 
            loading: None 
        }
    }
}

impl SceneNode for CreditLoadingScene {
    fn enter(&mut self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared objects to use. 
        let save = shared.get::<SaveData>().unwrap().clone();
        let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap().clone();
        let texture_map = shared.get::<Arc<HashMap<String, wgpu::Texture>>>().unwrap().clone();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
        let tex_sampler = shared.get::<Arc<wgpu::Sampler>>().unwrap().clone();
        let ui_brush = shared.get::<Arc<UiBrush>>().unwrap().clone();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap().clone();
        let asset_bundle = shared.get::<AssetBundle>().unwrap().clone();

        self.loading = Some(thread::spawn(move || {
            let nexon_lv2_gothic_bold = fonts.get(path::NEXON_LV2_GOTHIC_BOLD_PATH)
                .expect("A registered font could not be found.");
            let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
                .expect("A registered font could not be found.");

            // (한국어) 크레딧 텍스트들을 생성합니다. (HK416 <powerspirit127@gmail.com>이외의 사용자가 변경하는 것을 금지합니다.)
            // (English Translation) Creates credit texts. (Change by users other than HK416 <powerspirit127@gmail.com> is prohibited.)
            let mut credit_texts = Vec::new();
            credit_texts.push(
                TextBuilder::new(
                    Some("CreditText0"), 
                    nexon_lv2_gothic_medium, 
                    "This game is a fan-made secondary creation game from \"Blue Archive\", produced by Nexon Games.", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.98, 0.1, 0.94, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditText1"), 
                    nexon_lv2_gothic_medium, 
                    "Commercial use by any person or organization other than \nthe author of this game, \"Nexon Company\", \"Nexon Games\", or \"Yostar\" is prohibited.", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.94, 0.1, 0.88, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTitleCreator"), 
                    nexon_lv2_gothic_bold, 
                    "CREATOR", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.8, 0.1, 0.75, 0.2))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTextCreator0"), 
                    nexon_lv2_gothic_medium, 
                    "Planning, Interface and System design, Programming: HK416<powerspirit127@gmail.com>", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.74, 0.1, 0.7, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTitleLicense"), 
                    nexon_lv2_gothic_bold, 
                    "LICENSE", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.6, 0.1, 0.55, 0.2))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTextLicense0"), 
                    nexon_lv2_gothic_medium, 
                    "This game complies with the \"Blue Archive\" secondary creation guidelines.", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.54, 0.1, 0.5, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTextLicense1"), 
                    nexon_lv2_gothic_medium, 
                    "Anyone may reproduce and distribute this game with attribution to the author and source.", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.5, 0.1, 0.46, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );

            credit_texts.push(
                TextBuilder::new(
                    Some("CreditTextLicense2"), 
                    nexon_lv2_gothic_medium, 
                    "The source code for this game is licensed under the MIT license.", 
                    &text_brush
                )
                .with_anchor(Anchor::new(0.46, 0.1, 0.42, 0.9))
                .with_color((1.0, 1.0, 1.0, 1.0).into())
                .with_translation((0.0, 0.0, 0.5).into())
                .build(&device, &queue)
            );


            // (한국어) 텍스처를 불러옵니다.
            // (English Translation) Load the texture. 
            let texture = asset_bundle.get(path::BUTTON_RETURN_TEXTURE_PATH)?
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
                    device: &device,
                    queue: &queue, 
                })?;
            let texture_view = texture.create_view(
                &wgpu::TextureViewDescriptor {
                    ..Default::default()
                }
            );

            // (한국어) 사용을 완료한 에셋을 해제합니다.
            // (English Translation) Release assets that have been used.
            asset_bundle.release(path::BUTTON_RETURN_TEXTURE_PATH);

            // (한국어) 되돌아가기 버튼을 생성합니다.
            // (English Translation) Create a return button. 
            let return_button = UiObjectBuilder::new(
                Some("ReturnButton"), 
                &tex_sampler, 
                &texture_view, 
                &ui_brush
            )
            .with_anchor(Anchor::new(1.0, 0.0, 1.0, 0.0))
            .with_margin(Margin::new(-16, 16, -80, 80))
            .with_color((1.0, 1.0, 1.0, 1.0).into())
            .with_global_translation((0.0, 0.0, 0.75).into())
            .build(&device);

            // (한국어) 숨겨진 이미지를 불러옵니다. 
            // (English Translation) Load the hidden image. 
            let flag = save.stage_aris == NUM_TILES as u16
            && save.stage_momoi == NUM_TILES as u16 
            && save.stage_midori == NUM_TILES as u16 
            && save.stage_yuzu == NUM_TILES as u16;
            let rel_path = match flag {
                true => path::YUUKA_IMG_TEXTURE_PATH, 
                false => path::DEF_IMG_TEXTURE_PATH, 
            };
            let texture_view = texture_map.get(rel_path)
                .expect("Registered texture not found!")
                .create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

            // (한국어) 숨겨진 이미지 버튼을 생성합니다.
            // (English Translation) Creates a hidden image button. 
            let hidden_image_button = UiObjectBuilder::new(
                Some("HiddenImageButton"), 
                &tex_sampler, 
                &texture_view, 
                &ui_brush
            )
            .with_anchor(Anchor::new(1.0, 0.0, 1.0, 0.0))
            .with_margin(Margin::new(-96, 16, -160, 80))
            .with_color((1.0, 1.0, 1.0, 1.0).into())
            .with_global_translation((0.0, 0.0, 0.75).into())
            .build(&device);

            // (한국어) 숨겨진 이미지 뷰어를 생성합니다.
            // (English Translation) Creates a hidden image viewer. 
            let hidden_image_viewer = UiObjectBuilder::new(
                Some("HiddenImageViewer"), 
                &tex_sampler, 
                &texture_view, 
                &ui_brush
            )
            .with_anchor(Anchor::new(0.95, 0.2875, 0.05, 0.9625))
            .with_color((1.0, 1.0, 1.0, 0.0).into())
            .with_global_translation((0.0, 0.0, 0.1).into())
            .build(&device);


            Ok(CreditScene { 
                timer: 0.0, 
                state: CreditSceneState::default(), 
                credit_texts, 
                return_button, 
                hidden_image_button, 
                hidden_image_viewer, 
            })
        }));

        // (한국어) 로딩 텍스트를 생성합니다.
        // (English Translation) Create a loading text. 
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap();
        let fonts = shared.get::<Arc<HashMap<String, FontArc>>>().unwrap();

        let nexon_lv2_gothic_medium = fonts.get(path::NEXON_LV2_GOTHIC_MEDIUM_PATH)
            .expect("Registered font could not found!");
        let loading_text = TextBuilder::new(
            Some("LoadingText"), 
            nexon_lv2_gothic_medium, 
            "Loading", 
            text_brush
        )
        .with_anchor(Anchor::new(0.0, 1.0, 0.0, 1.0))
        .with_margin(Margin::new(128, -256, 0, 0))
        .with_color((1.0, 1.0, 1.0, 1.0).into())
        .build(device, queue);

        self.loading_text = Some(loading_text);

        Ok(())
    }

    fn update(&mut self, shared: &mut Shared, _total_time: f64, _elapsed_time: f64) -> AppResult<()> {
        if self.loading.as_ref().is_some_and(|it| it.is_finished()) {
            let next_scene = self.loading.take().unwrap().join().unwrap()?;
            *shared.get_mut::<SceneState>().unwrap() = SceneState::Change(Box::new(next_scene));
        }

        Ok(())
    }

    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        // (한국어) 사용할 공유 객체들을 가져옵니다.
        // (English Translation) Get shared objects to use.
        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
        let device = shared.get::<Arc<wgpu::Device>>().unwrap();
        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
        let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
        let camera = shared.get::<Arc<GameCamera>>().unwrap();
        let text_brush = shared.get::<Arc<TextBrush>>().unwrap();


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
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        // (한국어) 커맨드 버퍼를 생성합니다.
        // (English Translation) Creates a command buffer.
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(CreditLoadingScene)"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                                store: wgpu::StoreOp::Store,
                            }, 
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                        view: depth.view(), 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Discard
                        }), 
                        stencil_ops: None 
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );

            camera.bind(&mut rpass);
            text_brush.draw(&mut rpass, [
                self.loading_text.as_ref().unwrap()
            ].into_iter());
        }

        // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
        // (English Translation) Submit command buffers to the queue and output to the framebuffer.
        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Buttons {
    Return, 
    HiddenImage, 
}

/// #### 한국어 </br>
/// 게임 크레딧을 표시하는 게임 장면 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a game scene showing the game credits. </br>
/// 
#[derive(Debug)]
pub struct CreditScene {
    timer: f64, 
    state: CreditSceneState, 
    credit_texts: Vec<Text>, 
    return_button: UiObject, 
    hidden_image_button: UiObject, 
    hidden_image_viewer: UiObject, 
}

impl SceneNode for CreditScene {
    #[inline]
    fn handle_events(&mut self, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
        HANDLE_EVENTS_FN[self.state as usize](self, shared, event)
    }

    #[inline]
    fn update(&mut self, shared: &mut Shared, total_time: f64, elapsed_time: f64) -> AppResult<()> {
        UPDATE_FN[self.state as usize](self, shared, total_time, elapsed_time)
    }

    #[inline]
    fn draw(&self, shared: &mut Shared) -> AppResult<()> {
        DRAW_FN[self.state as usize](self, shared)
    }
}



/// #### 한국어 </br>
/// 게임 장면 상태 목록 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a list of game scene states. </br>
/// 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum CreditSceneState {
    #[default]
    Main, 
    EnterViewer, 
    ExitViewer, 
    Viewer, 
}

const HANDLE_EVENTS_FN: [&'static dyn Fn(&mut CreditScene, &mut Shared, Event<AppEvent>) -> AppResult<()>; 4] = [
    &main_state_handle_events, 
    &enter_viewer_state_handle_events, 
    &exit_viewer_state_handle_events, 
    &viewer_state_handle_events, 
];

const UPDATE_FN: [&'static dyn Fn(&mut CreditScene, &mut Shared, f64, f64) -> AppResult<()>; 4] = [
    &main_state_update, 
    &enter_viewer_state_update, 
    &exit_viewer_state_update, 
    &viewer_state_update, 
];

const DRAW_FN: [&'static dyn Fn(&CreditScene, &mut Shared) -> AppResult<()>; 4] = [
    &main_state_draw, 
    &enter_viewer_state_draw, 
    &exit_viewer_state_draw, 
    &viewer_state_draw, 
];


/// #### 한국어 </br>
/// 현재 눌려져있는 버튼의 데이터를 담고 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains data of the currently pressed button. </br>
/// 
static FOCUSED_UI: Mutex<Option<(Buttons, Vec3)>> = Mutex::new(None);

fn main_state_handle_events(this: &mut CreditScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    // (한국어) 소리를 재생합니다.
                    // (English Translation) Play the sounds. 
                    sound::play_cancel_sound(shared)?;

                    // (한국어) 눌려져있는 버튼을 원래 상태로 되돌립니다.
                    // (English Translation) Returns a pressed button to its original state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((button, ui_color)) = guard.take() {
                        match button {
                            Buttons::Return => this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::HiddenImage => this.hidden_image_button.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };
                    }

                    // (한국어) 다음 게임 장면으로 변경합니다.
                    // (English Translation) Change to the next game scene.
                    *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == button && state.is_pressed() {
                    // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the interface area.
                    let selected = [(Buttons::Return, &this.return_button), (Buttons::HiddenImage, &this.hidden_image_button)]
                        .into_iter()
                        .find_map(|(button, ui)| ui.test(&(cursor_pos, camera)).then_some(button));
                    if let Some(button) = selected {
                        // (한국어) `FOCUSED`에 인터페이스 데이터를 저장합니다.
                        // (English Translation) Store interface data in `FOCUSED`.
                        let ui_color = match button {
                            Buttons::Return => &this.return_button.data, 
                            Buttons::HiddenImage => &this.hidden_image_button.data,
                        }.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                        *guard = Some((button, ui_color));

                        // (한국어) 선택된 인터페이스의 색상을 변경합니다.
                        // (English Translation) Changes the color of the selected interface. 
                        match button {
                            Buttons::Return => &this.return_button, 
                            Buttons::HiddenImage => &this.hidden_image_button
                        }.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));

                        // (한국어) 소리를 재생합니다.
                        // (English Translation) Play the sounds. 
                        match button {
                            Buttons::Return => sound::play_cancel_sound(shared), 
                            Buttons::HiddenImage => sound::play_click_sound(shared)
                        }?;
                    }
                } else if MouseButton::Left == button && !state.is_pressed() {
                    // (한국어) 눌려져있는 버튼을 원래 상태로 되돌립니다.
                    // (English Translation) Returns a pressed button to its original state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((button, ui_color)) = guard.take() {
                        match button {
                            Buttons::Return => this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::HiddenImage => this.hidden_image_button.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };

                        // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the interface area.
                        let selected = [(Buttons::Return, &this.return_button), (Buttons::HiddenImage, &this.hidden_image_button)]
                            .into_iter()
                            .find_map(|(button, ui)| ui.test(&(cursor_pos, camera)).then_some(button));
                        if selected.is_some_and(|selected| selected == button) {
                            match button {
                                Buttons::Return => {
                                    // (한국어) 다음 게임 장면으로 변경합니다.
                                    // (English Translation) Change to the next game scene.
                                    *shared.get_mut::<SceneState>().unwrap() = SceneState::Pop;
                                },
                                Buttons::HiddenImage => {
                                    // (한국어) 다음 게임 상태로 변경합니다.
                                    // (English Translation) Change to the next game state.
                                    this.timer = 0.0;
                                    this.state = CreditSceneState::EnterViewer;
                                },
                            };
                        }
                    }
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }

    Ok(())
}

#[inline]
fn main_state_update(_: &mut CreditScene, _: &mut Shared, _: f64, _: f64) -> AppResult<()> {
    Ok(())
}

fn main_state_draw(this: &CreditScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();


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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(CreditScene)"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { 
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                            store: wgpu::StoreOp::Store,
                        }, 
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Discard
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );

        camera.bind(&mut rpass);
        text_brush.draw(&mut rpass, this.credit_texts.iter());
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.hidden_image_button
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}



#[inline]
fn enter_viewer_state_handle_events(_: &mut CreditScene, _: &mut Shared, _: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

fn enter_viewer_state_update(this: &mut CreditScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 0.2;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.timer += elapsed_time;

    // (한국어) 투명도를 갱신합니다.
    // (English Translation) Updates the transparency.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let delta = interpolation::f64::smooth_step(this.timer, DURATION) as f32;
    let transparency = 1.0 * delta; 
    this.hidden_image_viewer.update(queue, |data| data.color.w = transparency);

    let transparency = 1.0 - 1.0 * delta;
    this.hidden_image_button.update(queue, |data| data.color.w = transparency);
    for text in this.credit_texts.iter() {
        text.update(queue, |data| data.color.w = transparency);
    }

    // (한국어) 지속 시간을 초과할 경우 다음 게임 장면 상태로 변경합니다.
    // (English Translation) If the duration is exceeded, it changes to the next game scene state.
    if this.timer >= DURATION {
        this.timer = 0.0;
        this.state = CreditSceneState::Viewer;
    }

    Ok(())
}

fn enter_viewer_state_draw(this: &CreditScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();


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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(CreditScene)"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { 
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                            store: wgpu::StoreOp::Store,
                        }, 
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Discard
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );

        camera.bind(&mut rpass);
        text_brush.draw(&mut rpass, this.credit_texts.iter());
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.hidden_image_button, 
            &this.hidden_image_viewer
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}



#[inline]
fn exit_viewer_state_handle_events(_: &mut CreditScene, _: &mut Shared, _: Event<AppEvent>) -> AppResult<()> {
    Ok(())
}

fn exit_viewer_state_update(this: &mut CreditScene, shared: &mut Shared, _total_time: f64, elapsed_time: f64) -> AppResult<()> {
    const DURATION: f64 = 0.2;

    // (한국어) 타이머를 갱신합니다.
    // (English Translation) Updates the timer.
    this.timer += elapsed_time;

    // (한국어) 투명도를 갱신합니다.
    // (English Translation) Updates the transparency.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let delta = interpolation::f64::smooth_step(this.timer, DURATION) as f32;
    let transparency = 1.0 - 1.0 * delta; 
    this.hidden_image_viewer.update(queue, |data| data.color.w = transparency);

    let transparency = 1.0 * delta;
    this.hidden_image_button.update(queue, |data| data.color.w = transparency);
    for text in this.credit_texts.iter() {
        text.update(queue, |data| data.color.w = transparency);
    }

    // (한국어) 지속 시간을 초과할 경우 다음 게임 장면 상태로 변경합니다.
    // (English Translation) If the duration is exceeded, it changes to the next game scene state.
    if this.timer >= DURATION {
        this.timer = 0.0;
        this.state = CreditSceneState::Main;
    }

    Ok(())
}

fn exit_viewer_state_draw(this: &CreditScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();
    let text_brush = shared.get::<Arc<TextBrush>>().unwrap();


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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(CreditScene)"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { 
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                            store: wgpu::StoreOp::Store,
                        }, 
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Discard
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );

        camera.bind(&mut rpass);
        text_brush.draw(&mut rpass, this.credit_texts.iter());
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.hidden_image_button, 
            &this.hidden_image_viewer
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}



fn viewer_state_handle_events(this: &mut CreditScene, shared: &mut Shared, event: Event<AppEvent>) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let cursor_pos = shared.get::<PhysicalPosition<f64>>().unwrap();

    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => 
            if let PhysicalKey::Code(code) = event.physical_key {
                if KeyCode::Escape == code && !event.repeat && event.state.is_pressed() {
                    // (한국어) 소리를 재생합니다.
                    // (English Translation) Play the sounds. 
                    sound::play_cancel_sound(shared)?;

                    // (한국어) 눌려져있는 버튼을 원래 상태로 되돌립니다.
                    // (English Translation) Returns a pressed button to its original state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((button, ui_color)) = guard.take() {
                        match button {
                            Buttons::Return => this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::HiddenImage => this.hidden_image_button.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };
                    }

                    // (한국어) 다음 게임 상태로 변경합니다.
                    // (English Translation) Change to the next game state.
                    this.timer = 0.0;
                    this.state = CreditSceneState::ExitViewer;
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                if MouseButton::Left == button && state.is_pressed() {
                    // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                    // (English Translation) Make sure the mouse cursor is inside the interface area.
                    let selected = [(Buttons::Return, &this.return_button)]
                        .into_iter()
                        .find_map(|(button, ui)| ui.test(&(cursor_pos, camera)).then_some(button));
                    if let Some(button) = selected {
                        // (한국어) `FOCUSED`에 인터페이스 데이터를 저장합니다.
                        // (English Translation) Store interface data in `FOCUSED`.
                        let ui_color = match button {
                            Buttons::Return => &this.return_button.data, 
                            Buttons::HiddenImage => &this.hidden_image_button.data,
                        }.lock().expect("Failed to access variable.").color.xyz();
                        let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                        *guard = Some((button, ui_color));

                        // (한국어) 선택된 인터페이스의 색상을 변경합니다.
                        // (English Translation) Changes the color of the selected interface. 
                        match button {
                            Buttons::Return => &this.return_button, 
                            Buttons::HiddenImage => &this.hidden_image_button
                        }.update(queue, |data| data.color *= Vec4::new(0.5, 0.5, 0.5, 1.0));

                        // (한국어) 소리를 재생합니다.
                        // (English Translation) Play the sounds. 
                        match button {
                            Buttons::Return => sound::play_cancel_sound(shared), 
                            Buttons::HiddenImage => sound::play_click_sound(shared)
                        }?;
                    }
                } else if MouseButton::Left == button && !state.is_pressed() {
                    // (한국어) 눌려져있는 버튼을 원래 상태로 되돌립니다.
                    // (English Translation) Returns a pressed button to its original state. 
                    let mut guard = FOCUSED_UI.lock().expect("Failed to access variable.");
                    if let Some((button, ui_color)) = guard.take() {
                        match button {
                            Buttons::Return => this.return_button.update(queue, |data| data.color = (ui_color, data.color.w).into()), 
                            Buttons::HiddenImage => this.hidden_image_button.update(queue, |data| data.color = (ui_color, data.color.w).into()),
                        };

                        // (한국어) 마우스 커서가 인터페이스 영역 안에 있는지 확인합니다.
                        // (English Translation) Make sure the mouse cursor is inside the interface area.
                        let selected = [(Buttons::Return, &this.return_button), (Buttons::HiddenImage, &this.hidden_image_button)]
                            .into_iter()
                            .find_map(|(button, ui)| ui.test(&(cursor_pos, camera)).then_some(button));
                        if selected.is_some_and(|selected| selected == button) {
                            match button {
                                Buttons::Return => {
                                    // (한국어) 다음 게임 상태로 변경합니다.
                                    // (English Translation) Change to the next game state.
                                    this.timer = 0.0;
                                    this.state = CreditSceneState::ExitViewer;
                                },
                                Buttons::HiddenImage => {
                                    /* empty */
                                },
                            };
                        }
                    }
                }
            },
            _ => { /* empty */ }
        },
        _ => { /* empty */ }
    }
    
    Ok(())
}

#[inline]
fn viewer_state_update(_: &mut CreditScene, _: &mut Shared, _: f64, _: f64) -> AppResult<()> {
    Ok(())
}

fn viewer_state_draw(this: &CreditScene, shared: &mut Shared) -> AppResult<()> {
    // (한국어) 사용할 공유 객체들을 가져옵니다.
    // (English Translation) Get shared objects to use.
    let surface = shared.get::<Arc<wgpu::Surface>>().unwrap();
    let device = shared.get::<Arc<wgpu::Device>>().unwrap();
    let queue = shared.get::<Arc<wgpu::Queue>>().unwrap();
    let depth = shared.get::<Arc<DepthBuffer>>().unwrap();
    let camera = shared.get::<Arc<GameCamera>>().unwrap();
    let ui_brush = shared.get::<Arc<UiBrush>>().unwrap();


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
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

    // (한국어) 커맨드 버퍼를 생성합니다.
    // (English Translation) Creates a command buffer.
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("RenderPass(CreditScene)"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { 
                        view: &view, 
                        resolve_target: None, 
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                            store: wgpu::StoreOp::Store,
                        }, 
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: depth.view(), 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Discard
                    }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );

        camera.bind(&mut rpass);
        ui_brush.draw(&mut rpass, [
            &this.return_button, 
            &this.hidden_image_viewer
        ].into_iter());
    }

    // (한국어) 명령어 대기열에 커맨드 버퍼를 제출하고, 프레임 버퍼를 출력합니다.
    // (English Translation) Submit command buffers to the queue and output to the framebuffer.
    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
