use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

use async_std::task;
use winit::{
    event::{
        Event, 
        WindowEvent, 
        ElementState, 
        MouseScrollDelta
    },
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    panic_msg,
    app::{
        abort::PanicMsg,
        device::{KeyboardState, MouseButtonState},
        locale::get_wnd_title,
        message::{
            send,
            AppCommand,
            AppCommandChannel,
            GameLogicEvent,
            GameRenderEvent,
        },
        resolution::set_screen_mode,
        running_flag::RunningFlag,
        user_setting::{
            UserSetting,
            Decoder as UserSettingDecoder,
            Encoder as UserSettingEncoder,
        },
        timer::GameTimer,
    },
    assets::bundle::AssetBundle,
    game::init::scene::InitScene,
    logic::main_loop::game_logic_loop,
    render::{
        initialize::create_render_ctx,
        main_loop::game_render_loop,
    },
};



pub fn run() -> ! {
    log::info!("❖ Application Launching. ❖");
    
    let asset_bundle = AssetBundle::new()
        .unwrap_or_else(|msg| msg.abort());
    let handle =  task::block_on(asset_bundle.get("user.setting"))
        .unwrap_or_else(|msg| msg.abort());
    let user_setting = handle.read_or_default::<UserSetting, UserSettingDecoder, UserSettingEncoder>()
        .unwrap_or_else(|msg| msg.abort());

    let event_loop = EventLoop::new();
    let window = Arc::new(
        WindowBuilder::new()
            .with_active(true)
            .with_visible(true)
            .with_resizable(false)
            .with_window_icon(None)
            .with_title(get_wnd_title(&user_setting.locale))
            .with_inner_size(user_setting.resolution.as_ref().clone())
            .build(&event_loop)
            .unwrap_or_else(|e| 
                panic_msg!(
                    "Window system initialization faild",
                    "Window creation failed for the following reasons:{}",
                    e.to_string()
                ).abort()
            )
    );
    set_screen_mode(&window, &user_setting.screen_mode);


    // (한국어) `wgpu` 컨텍스트들을 생성합니다.
    // (English Translation) Create `wgpu` contexts.
    let (instance, surface, adapter, device, queue) = create_render_ctx(&window)
        .unwrap_or_else(|msg| msg.abort());

    // (한국어) 렌더 커맨드 채널을 생성합니다.
    // (English Translation) Create a render command channel.
    let (render_cmd_sender, render_cmd_receiver) = mpsc::channel();

    // (한국어) 게임 로직 스레드를 생성합니다.
    // (English Translation) Create a game logic thread.
    let window_cloned = window.clone();
    let asset_bundle_cloned = asset_bundle.clone();
    let (logic_event_sender, event_receiver) = mpsc::channel();
    thread::spawn(|| game_logic_loop(
        window_cloned, 
        asset_bundle_cloned, 
        Box::new(InitScene::new()), 
        event_receiver,
        render_cmd_sender,
    ));

    // (한국어) 게임 렌더 스레드를 생성합니다.
    // (English Translation) Create a game render thread.
    let window_cloned = window.clone();
    let (render_event_sender, event_receiver) = mpsc::channel();
    thread::spawn(|| game_render_loop(
        window_cloned, 
        event_receiver, 
        render_cmd_receiver, 
        instance, 
        surface, 
        adapter, 
        device, 
        queue
    ));


    // (한국어) 윈도우 이벤트 루프를 실행합니다.
    // (English ) Executes the window event loop.
    log::info!("Run event loop.");
    let mut timer = GameTimer::new();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        // (한국어) 에셋 파일에 문제가 있는지 확인합니다. 
        // (English Translation) Check if there is a problem with asset file of application.
        if !asset_bundle.check_integrity() {
            panic_msg!(
                "Asset file corruption detection", 
                "Corruption in the asset file has been detected and the program will terminate."
            ).abort()
        }

        // (한국어) 전달받은 명령을 처리합니다.
        // (English Translation) Processes the received command.
        while let Some(cmd) = AppCommandChannel::recv() {
            match cmd {
                AppCommand::Panic(msg) => msg.abort(),
                AppCommand::Terminate => control_flow.set_exit(),
            }
        }

        // (한국어) 어플리케이션 이벤트를 처리합니다.
        // (English Translation) Handles application events.
        match event {
            Event::NewEvents(_) => {
                timer.tick(None);
                send(GameLogicEvent::NextMainEvents(timer.elapsed_time_sec()), &logic_event_sender)
                    .unwrap_or_else(|msg| msg.abort());
            }
            Event::MainEventsCleared => {
                send(GameLogicEvent::MainEventsCleared, &logic_event_sender)
                    .unwrap_or_else(|msg| msg.abort());
            },
            Event::LoopDestroyed => log::info!("❖ Application finish. ❖"),
            Event::WindowEvent { window_id, event } 
            if window.id() == window_id => match event {
                WindowEvent::Resized(_) => {
                    send(GameLogicEvent::WindowResized, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                    send(GameRenderEvent::WindowResized, &render_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                },
                WindowEvent::Moved(p) => {
                    send(GameLogicEvent::WindowMoved { x: p.x , y: p.y }, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                },
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    send(GameLogicEvent::ApplicationTerminate, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                    send(GameRenderEvent::ApplicationTerminate, &render_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                    RunningFlag::set_exit();
                    control_flow.set_exit();
                },
                WindowEvent::Focused(focuse) => match focuse {
                    true => send(GameLogicEvent::ApplicationResumed, &logic_event_sender),
                    false => send(GameLogicEvent::ApplicationPaused, &logic_event_sender),
                }.unwrap_or_else(|msg| msg.abort()),
                WindowEvent::KeyboardInput { input, .. } => 
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            KeyboardState::on_pressed(&keycode);
                            send(GameLogicEvent::KeyPressed(keycode), &logic_event_sender)
                        },
                        ElementState::Released => {
                            KeyboardState::on_released(&keycode);
                            send(GameLogicEvent::KeyReleased(keycode), &logic_event_sender)
                        }
                    }.unwrap_or_else(|msg| msg.abort());
                },
                WindowEvent::CursorMoved { position, .. } => {
                    send(GameLogicEvent::CursorMoved { x: position.x as f32, y: position.y as f32 }, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                },
                WindowEvent::MouseWheel { delta, .. } => 
                if let MouseScrollDelta::LineDelta(horizontal, vertical) = delta {
                    send(GameLogicEvent::MouseWheel { horizontal , vertical }, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                },
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        MouseButtonState::on_pressed(&button);
                        send(GameLogicEvent::MousePressed(button), &logic_event_sender)
                            .unwrap_or_else(|msg| msg.abort());
                    }
                    ElementState::Released => {
                        MouseButtonState::on_released(&button);
                        send(GameLogicEvent::MouseReleased(button), &logic_event_sender)
                            .unwrap_or_else(|msg| msg.abort());
                    }
                },
                WindowEvent::ScaleFactorChanged { .. } => {
                    send(GameLogicEvent::WindowResized, &logic_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                    send(GameRenderEvent::WindowResized, &render_event_sender)
                        .unwrap_or_else(|msg| msg.abort());
                }
                _ => { }
            },
            Event::Suspended => {
                send(GameLogicEvent::ApplicationPaused, &logic_event_sender)
                    .unwrap_or_else(|msg| msg.abort());
            },
            Event::Resumed => {
                send(GameLogicEvent::ApplicationResumed, &logic_event_sender)
                    .unwrap_or_else(|msg| msg.abort());
            }
            _ => { }
        }
    });
}
