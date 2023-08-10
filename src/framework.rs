use winit::{
    window::{Window, WindowBuilder},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};
use crate::{
    assets::AssetBundle, 
    locale::get_wnd_title, 
    resolution::set_fullscreen,
    timer::GameTimer,
    user_setting::UserSetting, 
};


/// #### 한국어
/// 어플리케이션 프레임워크 입니다.
/// 어플리케이션에서 사용하는 윈도우, 랜더링 컨텍스트를 생성하고 이벤트 루프를 실행합니다.
/// 
/// #### English (Translation)
/// It is an application framework.
/// Creates a window and rendering context used by the application and executes a event loop.
/// 
#[derive(Debug)]
pub struct Framework {
    timer: GameTimer,
    bundle: AssetBundle,
    window: Window,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Framework {
    /// #### 한국어
    /// 새로운 어플리케이션 프레임워크를 생성합니다. 
    /// 어플리케이션에서 사용하는 데이터를 준비합니다.
    /// 이 함수는 비동기적으로 작동합니다.
    /// 
    /// #### English (Translation)
    /// Create a new application framework. 
    /// Prepare the data used by the application. 
    /// This function works asynchronously.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 어플리케이션에서 사용할 데이터를 준비하는 도중 에러가 발생하면 
    /// 오류 메시지를 띄우고 프로그램 실행을 중단시킵니다.
    /// 
    /// #### English (Translation)
    /// If an error occurs while preparing data to be used in the application, 
    /// an error message is displayed and abort program execution.
    /// 
    pub async fn new() -> Self {
        log::debug!("Create a new framework");

        let timer = GameTimer::new();

        let bundle = AssetBundle::new()
            .unwrap_or_else(|e| panic_msg("Asset system error", e.msg()));
        let handle = bundle.load_asset(UserSetting::ASSETS_PATH)    
            .unwrap_or_else(|e| panic_msg("Asset load error", e.msg()));
        let user_data = handle.get::<UserSetting>()
            .unwrap_or_else(|e| panic_msg("Asset load error", e.msg()));


        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_active(true)
            .with_visible(true)
            .with_resizable(false)
            .with_window_icon(None)
            .with_title(get_wnd_title(&user_data.locale))
            .with_inner_size(user_data.resolution.as_logical_size())
            .build(&event_loop)
            .unwrap_or_else(|e| panic_msg("Window system error", e.to_string()));

        set_fullscreen(&window, &user_data.screen_mode);

        let instance = create_wgpu_instance();
        let surface = unsafe { instance.create_surface(&window) }
            .unwrap_or_else(|e| panic_msg("Render system error", e.to_string()));
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface)
            }
        )
        .await
        .unwrap_or_else(|| panic_msg("Render system error", "Failed to get wgpu adapter."));

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None
            )
            .await
            .unwrap_or_else(|e| panic_msg("Render system error", e.to_string()));

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
            view_formats: vec![]
        };
        surface.configure(&device, &config);

        Self {
            timer,
            bundle,
            window,
            event_loop,
            instance,
            surface,
            adapter,
            device,
            queue
        }
    }

    /// #### 한국어
    /// 어플리케이션 프레임워크를 동작시킵니다.
    /// 어플리케이션이 종료될때 까지 윈도우 이벤트를 가져와 처리합니다.
    /// 
    /// #### English (Translation)
    /// Run the application framework.
    /// Get the process window events until the application is terminated.
    /// 
    /// <br>
    /// 
    /// # Panics
    /// #### 한국어
    /// 윈도우 이벤트를 처리하는 도중 에러가 발생하면 오류 메시지를 띄우고 프로그램 실행을 중단합니다.
    /// 
    /// #### English (Translation)
    /// If an error occurs while processing a window event, 
    /// an error message is displayed and abort program execution.
    /// 
    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            self.timer.tick(None);
            #[cfg(debug_assertions)] {
                use std::sync::Once;
                static ONCE: Once = Once::new();
                static mut TITLE_STR: String = String::new();
                ONCE.call_once(|| unsafe {
                    TITLE_STR = self.window.title();
                });

                self.window.set_title(&format!("{} (FPS:{})", unsafe { &TITLE_STR }, self.timer.current_frame_rate()));
            }
            

            match event {
                Event::WindowEvent { window_id, event }
                if self.window.id() == window_id => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => { },
                },
                Event::LoopDestroyed => log::info!("❖ Application finish. ❖"),
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(window_id)
                if self.window.id() == window_id => {
                    // TODO
                }
                _ => { }
            }
        })
    }
}


/// #### 한국어
/// 각 플랫폼에 맞는 wgpu 인스턴스를 생성합니다.
/// 
/// #### English (Translation)
/// Create a wgpu instance for each platform.
/// 
/// <br>
/// 
/// # Panics
/// #### 한국어
/// 어플리케이션이 플랫폼을 지원하지 않는 경우 프로그램 실행을 중단시킵니다.
/// 
/// #### English (Translation)
/// Abort program execution if the application does not support the platform.
/// 
fn create_wgpu_instance() -> wgpu::Instance {
    #[cfg(target_os = "macos")] {
        let backends = wgpu::Backends::METAL;
        let dx12_shader_compiler = wgpu::Dx12Compiler::Fxc;
        return wgpu::Instance::new(wgpu::InstanceDescriptor { backends, dx12_shader_compiler });
    }
    #[cfg(target_os = "windows")] {
        let backends = wgpu::Backends::DX12;
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        return wgpu::Instance::new(wgpu::InstanceDescriptor { backends, dx12_shader_compiler });
    }
    #[cfg(target_os = "linux")] {
        let backends = wgpu::Backends::VULKAN;
        let dx12_shader_compiler = wgpu::Dx12Compiler::Fxc;
        return wgpu::Instance::new(wgpu::InstanceDescriptor { backends, dx12_shader_compiler });
    }

    #[allow(unreachable_code)]
    panic_msg("Renderer error", "This platform is not supported.")
}

/// #### 한국어
/// 오류 메시지를 화면에 출력하고 프로그램 실행을 중단합니다.
/// 
/// #### English (Translation)
/// Prints an error message to the screen and abort program execution.
/// 
pub fn panic_msg<T: AsRef<str>, M: AsRef<str>>(title: T, message: M) -> ! {
    use std::process::abort;
    use native_dialog::{
        MessageDialog,
        MessageType,
    };

    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(title.as_ref())
        .set_text(message.as_ref())
        .show_alert()
        .unwrap();

    abort()
}
