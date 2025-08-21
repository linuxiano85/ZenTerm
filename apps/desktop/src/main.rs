use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // Event loop e finestra base
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_title("Warp-like Terminal (Bootstrap)")
        .build(&event_loop)
        .expect("Failed to build window");

    println!("Engine says: {}", engine::hello());

    // Loop eventi (placeholder per renderer GPU e input handling)
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(_size) => {
                    // Qui in futuro: resize del renderer
                }
                WindowEvent::RedrawRequested => {
                    // Qui in futuro: render GPU (wgpu)
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Qui in futuro: scheduling render/frame
                window.request_redraw();
            }
            _ => {}
        })
        .expect("Event loop error");
}
