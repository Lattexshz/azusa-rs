use azusa::{Azusa, Color};
use azusa::window::WindowSurface;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut surface = WindowSurface::new(&window);
    let azusa = Azusa::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::RedrawRequested(_)=> {
                azusa.clear(Color::Rgba(255,255,255,255));
                azusa.flush(&mut surface);
            }
            _ => (),
        }
    });
}