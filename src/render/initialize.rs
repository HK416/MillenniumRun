use std::sync::Arc;

use async_std::task;
use winit::window::Window;

use crate::{
    panic_msg, 
    app::abort::{PanicMsg, AppResult},
};



/// #### 한국어 </br>
/// `wgpu` 랜더링 컨텍스트를 생성하는 비동기 함수입니다. </br>
/// <b>메모: 이 함수는 반드시 메인 스레드에서만 호출되어야 합니다.</b></br>
/// 
/// #### English (Translation) </br>
/// A asynchronous function that creates a `wgpu` rendering context. </br>
/// <b>Note: This function must be called only from the main thread.</b></br>
/// 
/// <br>
/// 
/// # Errors </br>
/// #### 한국어 </br>
/// `wgpu` 랜더링 컨텍스트를 생성하기에 실패할 경우 `PanicMsg`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// `PanicMsg`is returned if it fails to create a `wgpu` rendering context. </br>
/// 
pub fn create_render_ctx(window: &Window) -> AppResult<(
    Arc<wgpu::Instance>,
    Arc<wgpu::Surface>,
    Arc<wgpu::Adapter>,
    Arc<wgpu::Device>,
    Arc<wgpu::Queue>,
)> {
    const INIT_ERR_TITLE: &'static str = "Render Context initialization failed";

    let instance = create_wgpu_instance();
    let surface = unsafe { instance.create_surface(window) }
        .map_err(|e| panic_msg!(INIT_ERR_TITLE, "{}", e.to_string()))?;
    let adapter = task::block_on(instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }
    ))
    .ok_or_else(|| panic_msg!(INIT_ERR_TITLE, "Failed to get wgpu adapter."))?;

    let (device, queue) = task::block_on(adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Render Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits())
            }, 
            None
        ))
        .map_err(|e| panic_msg!(INIT_ERR_TITLE, "{}", e.to_string()))?;

    Ok((instance.into(), surface.into(), adapter.into(), device.into(), queue.into()))
}



/// #### 한국어 </br>
/// 플랫폼마다 지정된 `wgpu` 인스턴스를 생성합니다. </br>
/// <b>메모: 지원하지 않는 플랫폼인 경우 프로그램 실행을 중단시킵니다.</b></br>
/// 
/// #### English (Translation)
/// Creates a specified `wgpu` instance per platform.
/// <b>Note: If the platform is not supported, the program will stop running.</b></br>
/// 
/// <br>
/// 
/// # Supported Platforms </br>
/// - <b>Windows: DirectX12</b></br>
/// - <b>linux: Vulkan</b></br>
/// - <b>macOS: Metal</b></br>
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
