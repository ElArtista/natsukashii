extern crate log;

#[macro_use]
mod shader;
mod gfx;
mod mesh;
mod renderer;

use gfx::Gfx;
use renderer::{DefaultRenderer, Renderer};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn render(gfx: &Gfx, renderer: &dyn Renderer) {
    let (swapchain, device, queue) = (&gfx.swapchain, &gfx.device, &gfx.queue);
    let frame = swapchain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;

    let encoder_desc = wgpu::CommandEncoderDescriptor { label: None };
    let mut encoder = device.create_command_encoder(&encoder_desc);

    renderer.render(&mut encoder, &frame.view);
    queue.submit(Some(encoder.finish()));
}

fn run() {
    // Create window and its event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Setup graphics
    let mut gfx = futures::executor::block_on(Gfx::new(&window));

    // Setup renderer
    let renderer = DefaultRenderer::new(&gfx.device, gfx.swapchain_desc.format);

    // Run window even loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                // Handle resize event
                WindowEvent::Resized(size) => gfx.configure_swapchain(size.into()),
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
            // Handle redraw event
            Event::RedrawRequested(_) => render(&gfx, &renderer),
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
