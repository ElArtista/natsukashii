extern crate log;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn run() {
    // Create window and its event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Run window even loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                // Handle window close event
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                // Handle key escape event
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            },
            // Handle events processed event
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}

fn main() {
    let log_env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(log_env);
    log::info!("Hello there!");
    run();
}
