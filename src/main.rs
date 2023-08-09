mod assets;
mod locale;
mod resolution;
mod user_setting;

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder, 
    event::{Event, WindowEvent},
};
use self::{
    assets::AssetBundle,
    locale::get_wnd_title,
    user_setting::UserSetting,
};


fn main() {
    env_logger::init();
    log::info!("Application Start.");

    let bundle = AssetBundle::new().unwrap();
    let handle = bundle.load_asset(UserSetting::ASSETS_PATH).unwrap();
    let user_data = handle.get::<UserSetting>().unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(get_wnd_title(&user_data.locale))
        .with_inner_size(user_data.resolution.as_logical_size())
        .build(&event_loop)
        .expect("Window creation failed!");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        assert!(handle.is_available(), "!!!");

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
