mod assets;

use async_std::task;
use lazy_static::lazy_static;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder, 
    event::{Event, WindowEvent},
};
use self::{
    assets::AssetBundle,
};


fn main() {
    env_logger::init();
    log::info!("Application Start.");

    let bundle = AssetBundle::new().unwrap();
    let handle0 = task::block_on(bundle.load_asset("user.setting")).unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Temp")
        .build(&event_loop)
        .expect("Window creation failed!");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        assert!(handle0.is_available(), "!!!");

        match event {
            Event::WindowEvent { window_id, event } 
            if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                },
                _ => { }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // TODO
            },
            _ => { },
        }
    });
}
