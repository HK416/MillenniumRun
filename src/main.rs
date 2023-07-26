use winit::{
    event_loop::EventLoop,
    window::WindowBuilder, 
    event::{Event, WindowEvent},
};


fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Temp")
        .build(&event_loop)
        .expect("Window creation failed!");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

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
