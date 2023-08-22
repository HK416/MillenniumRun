mod app;
mod assets;
mod logic;
mod render;


use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender};
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};

use async_std::task;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{
    window::WindowBuilder,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::render::create_render_ctx;
use crate::{
    app::*,
    assets::*,
    logic::game_logic_loop,
    render::game_render_loop,
};



/// #### 한국어
/// 프로그램의 진입점 입니다.
/// `Windows`, `Linux`, `macOS` 이 세 가지 운영체제만 어플리케이션이 동작합니다.
/// 
/// #### English (Translation)
/// It is the entry point of the program.
/// The application works only on these three operating systems: `Windows`, `Linux`, and `macOS`.
/// 
fn main() {
    env_logger::init();
    log::info!("❖ Application Launching. ❖");

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))] {
        run();
    }

    #[allow(unreachable_code)] {
        panic!("❗️❗️❗️ This platform is not supported. ❗️❗️❗️")
    }
}


fn run() -> ! {
    log::debug!("Create a new Application");

    // (한국어) 사용자 설정을 가져옵니다. 윈도우를 생성할 때 사용됩니다.
    // (English Translation) Get user settings. Used when creating a window.
    let asset_bundle = AssetBundle::new()
    .unwrap_or_else(|e| panic_err!("Asset initialization failed", "{}", e.msg()).abort());
    let user_setting_handle = asset_bundle
        .load_asset(UserSetting::ASSETS_PATH)
        .unwrap_or_else(|e| panic_err!("Asset load failed", "{}", e.msg()).abort());
    let user_setting = user_setting_handle
        .get::<UserSetting>()
        .unwrap_or_else(|e| panic_err!("Asset load failed", "{}", e.msg()).abort());

    // (한국어) 사용자 설정을 참고하여 이벤트 루프와 윈도우를 생성합니다.
    // (English Translation) Create an event loop and window by referring to user settings.
    let running_flag = Arc::new(AtomicBool::new(true));
    let event_loop = EventLoop::new();
    let window = Arc::new(WindowBuilder::new()
        .with_active(true)
        .with_visible(true)
        .with_resizable(false)
        .with_window_icon(None)
        .with_title(get_wnd_title(&user_setting.locale))
        .with_inner_size(user_setting.resolution.as_logical_size())
        .build(&event_loop)
        .unwrap_or_else(|e| panic_err!("Window initialization failed", "{}", e.to_string()).abort()));
    _ = set_fullscreen(&window, &user_setting.screen_mode)
        .map_err(|e| e.abort());

    // (한국어) 게임 로직 스레드를 생성합니다.
    // (English Translation) Create a game logic thread.
    let window_cloned = window.clone();
    let (logic_event_sender, event_receiver) = mpsc::channel();
    let (message_sender, scene_message_receiver) = mpsc::channel();
    let asset_bundle_cloned = asset_bundle.clone();
    let running_flag_cloned = running_flag.clone();
    thread::spawn(|| game_logic_loop(
        window_cloned, 
        event_receiver, 
        message_sender, 
        asset_bundle_cloned, 
        running_flag_cloned
    ));

    let (instance, surface, adapter, device, queue) = task::block_on(
        create_render_ctx(&window)
    ).unwrap_or_else(|e| e.abort());

    let window_cloned = window.clone();
    let (render_event_sender, event_receiver) = mpsc::channel();
    let (message_sender, world_message_receiver) = mpsc::channel();
    let running_flag_cloned = running_flag.clone();
    thread::spawn(|| game_render_loop(
        window_cloned, 
        event_receiver,
        message_sender,
        running_flag_cloned,
        instance, 
        surface, 
        adapter, 
        device, 
        queue
    ));

    // (한국어) 윈도우 이벤트 루프를 실행합니다.
    // (English ) Executes the window event loop.
    log::debug!("Run event loop.");
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        while let Ok(msg) = scene_message_receiver.try_recv() {
            log::debug!("received scene message. (msg:{:?})", &msg);
            match msg {
                AppCmd::PanicError(err) => err.abort(),
                AppCmd::Terminate => control_flow.set_exit(),
            }
        }

        while let Ok(msg) = world_message_receiver.try_recv() {
            log::debug!("received world message. (msg:{:?}", &msg);
            match msg {
                AppCmd::PanicError(err) => err.abort(),
                AppCmd::Terminate => control_flow.set_exit(),
            }
        }

        match event {
            Event::NewEvents(_) => {
                send_event(
                    &running_flag, 
                    &logic_event_sender, 
                    GameLogicEvent::NextMainEvents
                ).unwrap_or_else(|e| e.abort());
            },
            Event::MainEventsCleared => {
                send_event(
                    &running_flag, 
                    &logic_event_sender,
                    GameLogicEvent::MainEventsCleared
                ).unwrap_or_else(|e| e.abort());
            },
            Event::LoopDestroyed => log::info!("❖ Application finish. ❖"),
            Event::WindowEvent { window_id, event } 
            if window.id() == window_id => match event{
                WindowEvent::Resized(_) => {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::WindowResized
                    ).unwrap_or_else(|e| e.abort());
                    send_event(
                        &running_flag, 
                        &render_event_sender, 
                        GameRenderEvent::WindowResized
                    ).unwrap_or_else(|e| e.abort());
                },
                WindowEvent::Moved(pos) => {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::WindowMoved { x: pos.x, y: pos.y }
                    ).unwrap_or_else(|e| e.abort());
                },
                WindowEvent::CloseRequested | WindowEvent::Destroyed  => {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::ApplicationTerminate
                    ).unwrap_or_else(|e| e.abort());
                    send_event(
                        &running_flag, 
                        &render_event_sender, 
                        GameRenderEvent::ApplicationTerminate
                    ).unwrap_or_else(|e| e.abort());
                    running_flag.store(false, MemOrdering::Release);
                    control_flow.set_exit();
                },
                WindowEvent::Focused(focuse) => match focuse {
                    true => send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::ApplicationResumed
                    ).unwrap_or_else(|e| e.abort()),
                    false => send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::ApplicationPaused
                    ).unwrap_or_else(|e| e.abort()),
                },
                WindowEvent::KeyboardInput { input, .. } =>
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => send_event(
                                &running_flag, 
                                &logic_event_sender, 
                                GameLogicEvent::KeyPressed(keycode)
                            ).unwrap_or_else(|e| e.abort())
                        ,
                        ElementState::Released => send_event(
                            &running_flag, 
                            &logic_event_sender, 
                            GameLogicEvent::KeyReleased(keycode)
                        ).unwrap_or_else(|e| e.abort())
                    }
                },
                WindowEvent::CursorMoved { position, .. } => {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::CursorMoved { x: position.x as f32, y: position.y as f32 }
                    ).unwrap_or_else(|e| e.abort())
                },
                WindowEvent::MouseWheel { delta, .. } =>
                if let MouseScrollDelta::LineDelta(horizontal, vertical) = delta {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::MouseWheel { horizontal, vertical }
                    ).unwrap_or_else(|e| e.abort())
                },
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::MousePressed(button)
                    ).unwrap_or_else(|e| e.abort()),
                    ElementState::Released => send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::MouseReleased(button)
                    ).unwrap_or_else(|e| e.abort())
                },
                WindowEvent::ScaleFactorChanged { .. } => {
                    send_event(
                        &running_flag, 
                        &logic_event_sender, 
                        GameLogicEvent::WindowResized
                    ).unwrap_or_else(|e| e.abort());
                    send_event(
                        &running_flag, 
                        &render_event_sender, 
                        GameRenderEvent::WindowResized
                    ).unwrap_or_else(|e| e.abort());
                },
                _ => { },
            },
            Event::Suspended => {
                send_event(
                    &running_flag, 
                    &logic_event_sender, 
                    GameLogicEvent::ApplicationPaused
                ).unwrap_or_else(|e| e.abort());
            },
            Event::Resumed => {
                send_event(
                    &running_flag, 
                    &logic_event_sender, 
                    GameLogicEvent::ApplicationResumed
                ).unwrap_or_else(|e| e.abort());
            },
            _ => { },
        };
        
    });
}



#[inline]
fn send_event<T>(
    running_flag: &Arc<AtomicBool>, 
    sender: &Sender<T>, 
    event: T,
) -> AppResult<()> {
    match running_flag.load(MemOrdering::Acquire) {
        true => sender.send(event).map_err(|e| 
            panic_err!("Failed to send event.", "{}", e.to_string())
        ),
        false => Ok(())
    }
}
