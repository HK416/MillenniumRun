use winit::window::Window;

use crate::{
    panic_err, 
    app::{AppResult, PanicErr},
};



/// #### 한국어
/// `wgpu` 랜더링 컨텍스트를 생성하는 비동기 함수입니다.
/// 
/// #### English (Translation)
/// A asynchronous function that creates a `wgpu` rendering context.
/// 
/// <br>
/// 
/// # Notes
/// #### 한국어
/// 이 함수는 반드시 메인 스레드에서 호출되어야 합니다. 
/// 그렇지 않을 경우 일부 플랫폼에서 문제가 발생할 수 있습니다.
/// 
/// #### English (Translation)
/// This function must be called from the main thread.
/// Failure to do so may cause issues on some platforms.
/// 
/// <br>
/// 
/// # Errors
/// #### 한국어
/// `wgpu` 랜더링 컨텍스트를 생성하기에 실패할 경우 `PanicErr`를 반환합니다.
/// 
/// #### English (Translation)
/// `PanicErr`is returned if it fails to create a `wgpu` rendering context.
/// 
pub async fn create_render_ctx(window: &Window) -> AppResult<(
    wgpu::Instance,
    wgpu::Surface,
    wgpu::Adapter,
    wgpu::Device,
    wgpu::Queue,
)> {
    const INIT_ERR_TITLE: &'static str = "Render Context initialization failed";

    let instance = create_wgpu_instance();
    let surface = unsafe { instance.create_surface(window) }
        .map_err(|e| panic_err!(INIT_ERR_TITLE, "{}", e.to_string()))?;
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }
    )
    .await
    .ok_or_else(|| panic_err!(INIT_ERR_TITLE, "Failed to get wgpu adapter."))?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Render Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits())
            }, 
            None
        )
        .await
        .map_err(|e| panic_err!(INIT_ERR_TITLE, "{}", e.to_string()))?;

    Ok((instance, surface, adapter, device, queue))
}



/// #### 한국어
/// 플랫폼마다 지정된 `wgpu` 인스턴스를 생성합니다.
/// 
/// #### English (Translation)
/// Creates a specified `wgpu` instance per platform.
/// 
/// <br>
/// 
/// # Panics
/// #### 한국어
/// 지원하지 않는 플랫폼인 경우 프로그램 실행을 중단시킵니다.
/// 
/// #### English (Translation)
/// Abort program execution if the platform is not supported.
/// 
/// <br>
/// 
/// # Supported Platforms
/// - `Windows: `DirectX 12`
/// - `linux`: `Vulkan`
/// - `macOS`: `Metal`
/// 
#[inline]
#[allow(unreachable_code)]
fn create_wgpu_instance() -> wgpu::Instance {
    #[cfg(target_os = "windows")]
    return wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default()
    });

    #[cfg(target_os = "linux")] 
    return wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
    });

    #[cfg(target_os = "macos")]
    return wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::METAL,
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
    });

    panic!("The platform is not supported.");
}
