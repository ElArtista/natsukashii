extern crate log;

mod gfx;

use gfx::Gfx;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn render(gfx: &Gfx) {
    let frame = gfx
        .swapchain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;

    let mut encoder = gfx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
    }

    gfx.queue.submit(Some(encoder.finish()));
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
            Event::RedrawRequested(_) => render(&gfx),
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
