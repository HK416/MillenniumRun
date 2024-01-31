pub mod depth;
pub mod shader;
pub mod texture;



use std::sync::Arc;

use winit::window::Window;

use crate::{
    game_err,
    system::error::{
        AppResult,
        GameError,
    },
};



/// #### 한국어 </br>
/// `wgpu` 렌더링 컨텍스트들을 생성합니다. </br>
/// 이 함수를 실행하는 중에 오류가 발생한 경우 `GameError`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create `wgpu` rendering contexts. </br>
/// If an error occurs while executing this function, it returns `GameError`. </br>
/// 
#[inline]
pub fn setup_render_ctx(window: Arc<Window>) -> AppResult<(
    Arc<wgpu::Instance>,
    Arc<wgpu::Surface<'static>>,
    Arc<wgpu::Adapter>,
    Arc<wgpu::Device>,
    Arc<wgpu::Queue>,
    Arc<depth::DepthBuffer>
)> {
    let instance = create_render_instance();
    let surface = create_render_surface(&instance, window.clone())?;
    let adapter = create_render_adapter(&instance, &surface)?;
    let (device, queue) = create_render_device_and_queue(&adapter)?;
    let depth_buffer = create_depth_buffer(&window, &device);
    Ok((instance, surface, adapter, device, queue, depth_buffer))
}


/// #### 한국어 </br>
/// `wgpu` 렌더링 인스턴스를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a `wgpu` rendering instance. </br>
/// 
#[inline]
fn create_render_instance() -> Arc<wgpu::Instance> {
    let instance_desc = if cfg!(target_os = "windows") {
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
            ..Default::default()
        }
    } else if cfg!(target_os = "linux") {
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        }
    } else if cfg!(target_os = "macos") {
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::METAL,
            ..Default::default()
        }
    } else {
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        }
    };

    wgpu::Instance::new(instance_desc).into()
}


/// #### 한국어 </br>
/// `wgpu` 렌더링 표면을 생성합니다. </br>
/// 이 함수를 실행하는 중에 오류가 발생한 경우 `GameError`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a `wgpu` rendering surface. </br>
/// If an error occurs while executing this function, it returns `GameError`.
/// 
#[inline]
fn create_render_surface(
    instance: &wgpu::Instance, 
    window: Arc<Window>
) -> AppResult<Arc<wgpu::Surface<'static>>> {
    instance.create_surface(wgpu::SurfaceTarget::from(window))
        .map(|surface| surface.into())
        .map_err(|err| game_err!(
            "Failed to create rendering context",
            "Creating a rendering context failed for the following reasons: {}",
            err.to_string()
        ))
}


/// #### 한국어 </br>
/// `wgpu` 렌더링 장치 어뎁터를 생성합니다. </br>
/// 적절한 장치를 찾지 못한 경우 `GameError`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a `wgpu` rendering device adapter. </br>
/// Returns `GameError` if no suitable device is found. </br>
/// 
#[inline]
fn create_render_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface
) -> AppResult<Arc<wgpu::Adapter>> {
    pollster::block_on(
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::default()
        })
    )
    .map(|adapter| adapter.into())
    .ok_or_else(|| game_err!(
        "Failed to create rendering context",
        "No suitable device was found."
    ))
}


/// #### 한국어 </br>
/// `wgpu` 논리적 장치와 명령어 대기열을 생성합니다. </br>
/// 이 함수를 실행하는 중에 오류가 발생한 경우 `GameError`를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a `wgpu` logical device and command queue. </br>
/// If an error occurs while executing this function, it returns `GameError`. </br>
/// 
#[inline]
fn create_render_device_and_queue(
    adapter: &wgpu::Adapter
) -> AppResult<(Arc<wgpu::Device>, Arc<wgpu::Queue>)> {
    pollster::block_on(
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Rendering device"),
                required_features: wgpu::Features::TEXTURE_COMPRESSION_BC,
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits())
            }, 
            None
        )
    )
    .map(|(device, queue)| (device.into(), queue.into()))
    .map_err(|err| game_err!(
        "Failed to create rendering context",
        "Creating a rendering context failed for the following reasons: {}",
        err.to_string()
    ))
}


/// #### 한국어 </br>
/// 깊이 테스트에 사용되는 깊이 버퍼를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a depth buffer used for the depth testing. </br>
/// 
#[inline]
fn create_depth_buffer(window: &Window, device: &wgpu::Device,) -> Arc<depth::DepthBuffer>  {
    Arc::new(depth::DepthBuffer::new(window, device))
}
