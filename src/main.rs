mod assets;
mod components;
mod nodes;
mod render;
mod scene;
mod system;

use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as MemOrdering};
use std::collections::VecDeque;

use crossbeam_queue::SegQueue;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy, EventLoopBuilder, ControlFlow},
    window::{Window, WindowBuilder},
    dpi::PhysicalPosition,
};

use crate::{
    assets::bundle::AssetBundle,
    components::camera::GameCamera,
    nodes::setup::SetupScene,
    render::depth::DepthBuffer,
    scene::{
        node::SceneNode,
        state::SceneState,
    },
    system::{
        error::GameError,
        event::AppEvent,
        shared::Shared,
        timer::GameTimer,
    },
};


/// #### 한국어 </br>
/// 애플리케이션의 실행 여부를 나타냅니다. </br>
/// 
/// #### English (Translation) </br>
/// Indicates whether the application is running. </br>
/// 
static RUNNING_FLAG: AtomicBool = AtomicBool::new(true);

/// #### 한국어 </br>
/// 애플리케이션 윈도우 이벤트 대기열 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Application window event queue. </br>
/// 
static EVENT_QUEUE: SegQueue<Event<AppEvent>> = SegQueue::new();



/// #### 한국어 </br>
/// 애플리케이션 게임 장면을 실행하는 함수입니다. </br>
/// 이 함수를 실행하는 도중 오류가 발생한 경우 에러 메시지를 이벤트 루프에 전달하고 종료합니다. </br>
/// 
/// #### English (Translation) </br>
/// This function runs the application game scene. </br>
/// If an error occurs while executing this function, the error message is passed to the event loop and exits. </br>
/// 
fn game_loop(
    window: Arc<Window>,
    event_loop_proxy: EventLoopProxy<AppEvent>,
    asset_bundle: AssetBundle,
    instance: Arc<wgpu::Instance>,
    surface: Arc<wgpu::Surface>,
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    depth_buffer: Arc<DepthBuffer>
) {
    use crate::system::error::send_panic_msg_and_abort;

    const MAX_UPDATE_COUNT: usize = 30;
    const MAX_FRAMERATE: u64 = 60;
    const FIXED_TIME_SEC: f64 = 1.0 / MAX_FRAMERATE as f64;

    // (한국어) wgpu 프레임 버퍼를 설정합니다.
    // (English Translation) Set the wgpu framebuffer.
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let swapchain_alpha_mode = swapchain_capabilities.alpha_modes[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: swapchain_alpha_mode,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    // (한국어) 공유할 객체들을 등록합니다.
    // (English Translation) Register shared objects.
    let mut shared = Shared::new();
    shared.push(window.clone());
    shared.push(event_loop_proxy.clone());
    shared.push(asset_bundle);
    shared.push(instance);
    shared.push(surface);
    shared.push(adapter);
    shared.push(device);
    shared.push(queue);
    shared.push(depth_buffer);
    shared.push(config);
    shared.push(PhysicalPosition::new(0.0, 0.0));

    // (한국어) 장면 상태를 공유 객체로 등록합니다.
    // (English Translation) Register the scene state as a shared object.
    shared.push(SceneState::default());

    // (한국어) 장면 스택을 생성합니다.
    // (English) Create a scene stack.
    let mut scene_stack = VecDeque::new();

    // (한국어) 게임 장면을 생성하고 진입합니다.
    // (English Translation) Create and enter the game scene.
    let mut entry_scene: Box<dyn SceneNode> = Box::new(SetupScene::default());
    entry_scene.enter(&mut shared)
        .unwrap_or_else(|err| 
            send_panic_msg_and_abort(&event_loop_proxy, err)
        );

    // (한국어) 장면 스택에 장면을 추가합니다.
    // (English Translation) Add a scene to the scene stack.
    scene_stack.push_back(entry_scene);

    // (한국어) 게임 루프를 실행합니다.
    // (English Translation) Run the game loop.
    log::info!("Run game loop.");
    let mut timer = GameTimer::new();
    let mut elapsed_time_sec = 0.0;
    while RUNNING_FLAG.load(MemOrdering::Acquire) {
        // (한국어) 타이머를 갱신합니다.
        // (English Translation) Update the timer.
        timer.tick(None);
        elapsed_time_sec += timer.elapsed_time_sec();

        // (한국어) 윈도우 이벤트를 처리합니다.
        // (English Translation) Handles window events.
        while let Some(event) = EVENT_QUEUE.pop() {
            let event_cloned = event.clone();
            match event_cloned {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                        let instance = shared.get::<Arc<wgpu::Instance>>().unwrap().clone();
                        let surface = shared.get::<Arc<wgpu::Surface>>().unwrap().clone();
                        let device = shared.get::<Arc<wgpu::Device>>().unwrap().clone();
                        let queue = shared.get::<Arc<wgpu::Queue>>().unwrap().clone();
                        let config = shared.get_mut::<wgpu::SurfaceConfiguration>().unwrap();
    
                        let width = window.inner_size().width;
                        let height = window.inner_size().height;
    
                        if width > 0 && height > 0 {
                            instance.poll_all(true);
                            config.width = width;
                            config.height = height;
                            surface.configure(&device, config);
                            shared.push(Arc::new(DepthBuffer::new(&window, &device)));
                            if let Some(camera) = shared.get_mut::<GameCamera>() {
                                camera.viewport.width = width as f32;
                                camera.viewport.height = height as f32;
                                camera.scale_factor = window.current_monitor().map_or(1.0, |monitor| monitor.scale_factor() as f32);
                                camera.update_buffer(&queue);
                            }
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        let height = shared.get::<Arc<Window>>().unwrap().inner_size().height as f64;
                        let cursor = shared.get_mut::<PhysicalPosition<f64>>().unwrap();
                        cursor.x = position.x;
                        cursor.y = height - position.y;
                    },
                    _ => { /* empty */ }
                },
                _ => { /* empty */ }
            };

            // (한국어) 게임 장면에 이벤트를 전달합니다.
            // (English Translation) Passes events to the game scene.
            scene_stack.back_mut()
                .unwrap()
                .handle_events(&mut shared, event)
                .unwrap_or_else(|err| send_panic_msg_and_abort(&event_loop_proxy, err));
        }

        let mut update_cnt = 0;
        while elapsed_time_sec >= FIXED_TIME_SEC && update_cnt < MAX_UPDATE_COUNT {
            // (한국어) 게임 장면을 갱신합니다.
            // (English Translation) Update the game scene.
            scene_stack.back_mut()
                .unwrap()
                .update(
                    &mut shared, 
                    timer.total_time_sec(), 
                    FIXED_TIME_SEC
                )
                .unwrap_or_else(|err|
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );

            elapsed_time_sec -= FIXED_TIME_SEC;
            update_cnt += 1;
        }
        

        // (한국어) 게임 장면을 그립니다.
        // (English Translation) Draw the game scene.
        window.pre_present_notify();
        scene_stack.back()
            .unwrap()
            .draw(&mut shared)
            .unwrap_or_else(|err| 
                send_panic_msg_and_abort(&event_loop_proxy, err)
            );

        // (한국어) 게임 장면 상태에 따라 게임 장면을 갱신합니다.
        // (English Translation) Updates the game scene according to the game scene state.
        match shared.pop::<SceneState>().unwrap() {
            SceneState::Keep => { /* pass */ },
            SceneState::Pop => { 
                // (한국어) 가장 최근의 게임 장면을 장면 스택에서 제거하고 종료합니다.
                // (English Translation) Remove the most recent game scene from the scene stack and exits.
                let mut old = scene_stack.pop_back().unwrap();
                old.exit(&mut shared).unwrap_or_else(|err| 
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );

                // (한국어) 장면 스택이 비어있는 경우 애플리케이션을 종료합니다.
                // (English Translation) Terminates the application if the scene stack is empty.
                if scene_stack.is_empty() {
                    RUNNING_FLAG.store(false, MemOrdering::Release);
                    event_loop_proxy.send_event(AppEvent::Terminate).unwrap();
                    break;
                }
            },
            SceneState::Push(mut new) => {
                // (한국어) 새로운 게임 장면에 진입하고 장면 스택에 추가합니다.
                // (English Translation) Enters a new game scene and adds it to the scene stack.
                new.enter(&mut shared).unwrap_or_else(|err| 
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );
                scene_stack.push_back(new);
            },
            SceneState::Change(mut new) => {
                // (한국어) 가장 최근의 게임 장면을 장면 스택에서 제거하고 종료합니다.
                // (English Translation) Remove the most recent game scene from the scene stack and exits.
                let mut old = scene_stack.pop_back().unwrap();
                old.exit(&mut shared).unwrap_or_else(|err|
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );

                // (한국어) 새로운 게임 장면에 진입하고 장면 스택에 추가합니다.
                // (English Translation) Enters a new game scene and adds it to the scene stack.
                new.enter(&mut shared).unwrap_or_else(|err|
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );
                scene_stack.push_back(new);
            },
            SceneState::Reset(mut new) => {
                while let Some(mut old) = scene_stack.pop_back() {
                    // (한국어) 가장 최근의 게임 장면을 장면 스택에서 제거하고 종료합니다.
                    // (English Translation) Remove the most recent game scene from the scene stack and exits.
                    old.exit(&mut shared).unwrap_or_else(|err|
                        send_panic_msg_and_abort(&event_loop_proxy, err)
                    );
                };

                // (한국어) 새로운 게임 장면에 진입하고 장면 스택에 추가합니다.
                // (English Translation) Enters a new game scene and adds it to the scene stack.
                new.enter(&mut shared).unwrap_or_else(|err|
                    send_panic_msg_and_abort(&event_loop_proxy, err)
                );
                scene_stack.push_back(new);
            }
        };

        shared.push(SceneState::default());
    }
}




/// #### 한국어 </br>
/// 애플리케이션의 진입점 입니다. </br>
/// <b>대상 플랫폼이 `Windows` 또는 `Linux` 또는 `macOS`가 아닐 경우 애플리케이션이 동작하지 않습니다.</b></br>
/// 
/// #### English (Translation) </br>
/// This is the entry point to the application </br>
/// <b>If the target platform is not `Windows` or `Linux` or `macOS`, the application will not work.</b></br>
/// 
fn main() {
    use crate::{
        render::setup_render_ctx,
        system::error::popup_err_msg_and_abort
    };

    // (한국어) 로그 시스템을 초기화 합니다.
    // (English Translation) Initialize log system.
    env_logger::init();
    log::info!("❖ Application Launching. ❖");

    if cfg!(not(any(target_os = "macos", target_os = "windows", target_os = "linux"))) {
        panic!("❗️❗️❗️ This platform is not supported. ❗️❗️❗️")
    };

    // (한국어) 애플리케이션 에셋 관리자를 생성합니다. 
    // (English Translation) Create an application asset manager.
    let asset_bundle = AssetBundle::new()
        .unwrap_or_else(|err| popup_err_msg_and_abort(err));

    // (한국어) 애플리케이션 윈도우를 생성합니다.
    // (English Translation) Creates an application window.
    let event_loop: EventLoop<AppEvent> = EventLoopBuilder::with_user_event()
        .build()
        .map_err(|err| game_err!(
            "Window system initialization failed",
            "Application running failed for the following reasons: {}",
            err.to_string()
        ))
        .unwrap_or_else(|err| popup_err_msg_and_abort(err));
    let window = Arc::new(
        WindowBuilder::new()
            .with_visible(false)
            .with_resizable(false)
            .with_window_icon(None)
            .with_title("Application Initialize...")
            .build(&event_loop)
            .map_err(|err| game_err!(
                "Window system initialization failed",
                "Application running failed for the following reasons: {}",
                err.to_string()
            ))
            .unwrap_or_else(|err| popup_err_msg_and_abort(err))
    );

    // (한국어) 렌더링 컨텍스트들을 생성합니다.
    // (English Translation) Create rendering contexts.
    let (
        instance,
        surface,
        adapter,
        device,
        queue,
        depth_buffer,
    ) = setup_render_ctx(&window)
        .unwrap_or_else(|err| popup_err_msg_and_abort(err));


    // (한국어) 새로운 스레드를 생성하고, 게임 루프를 실행시킵니다.
    // (English Translation) Create a new thread and run the game loop.
    let window_cloned = window.clone();
    let event_loop_proxy = event_loop.create_proxy();
    let asset_bundle_cloned = asset_bundle.clone();
    thread::spawn(move || game_loop(
        window_cloned, 
        event_loop_proxy,
        asset_bundle_cloned, 
        instance, 
        surface, 
        adapter, 
        device, 
        queue,
        depth_buffer
    ));

    // (한국어) 윈도우 메시지 루프를 실행합니다.
    // (English Translation) Executes the window message loop.
    log::info!("Run window message loop.");
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(move |event, elwt| {
        // (한국어) 애플리케이션 에셋 파일의 무결성을 검사합니다.
        // (English Translation) Check the integrity of application asset files.
        if !asset_bundle.check_integrity() {
            popup_err_msg_and_abort(game_err!(
                "Asset file corruption detected",
                "Application running failed for the following reasons: {}",
                "Corruption in the asset file has been detected."
            ));
        }

        // (한국어) 윈도우 이벤트를 처리합니다.
        // (English Translation) Handles window events.
        let event_cloned = event.clone();
        if let Event::NewEvents(_) = event_cloned {
            return;
        } else if let Event::AboutToWait = event_cloned {
            return;
        } else if let Event::WindowEvent { window_id, event } = event_cloned {
            if window_id == window.id() && (event == WindowEvent::CloseRequested || event == WindowEvent::Destroyed) {
                RUNNING_FLAG.store(false, MemOrdering::Release);
                elwt.exit();
                return;
            } else if window_id != window.id() {
                return;
            }
        } else if let Event::UserEvent(event) = event_cloned {
            match event {
                AppEvent::GameError(err) => {
                    popup_err_msg_and_abort(err);
                },
                AppEvent::Terminate => {
                    elwt.exit();
                },
            };
            return;
        };

        // (한국어) 윈도우 이벤트를 이벤트 대기열에 추가합니다.
        // (English Translation) Add a window event to the event queue.
        EVENT_QUEUE.push(event.clone());
    }).map_err(|err| game_err!(
        "Window system running failed",
        "Application running failed for the following reasons: {}",
        err.to_string()
    ))
    .unwrap_or_else(|err| popup_err_msg_and_abort(err));

    log::info!("❖ Application finish. ❖");
}
