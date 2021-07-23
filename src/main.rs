extern crate log;

#[macro_use]
mod shader;
mod gfx;
mod mesh;
mod renderer;
mod uniform;

use gfx::Gfx;
use glam::{Mat4, Vec3};
use mesh::{Index, Mesh, Vertex};
use renderer::{DefaultRenderer, Renderer, RendererScene};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[allow(dead_code)]
fn demo_mesh() -> Mesh {
    use genmesh::{
        generators::{IcoSphere, IndexedPolygon, SharedVertex},
        Triangulate,
    };

    let prim = IcoSphere::subdivide(1);
    let vertices: Vec<Vertex> = prim
        .shared_vertex_iter()
        .map(|v| Vertex::new(v.pos.into()))
        .collect();
    let indices: Vec<Index> = prim
        .indexed_polygon_iter()
        .triangulate()
        .map(|f| vec![f.x as _, f.y as _, f.z as _])
        .flatten()
        .collect();

    Mesh { vertices, indices }
}

fn render(gfx: &Gfx, renderer: &dyn Renderer) {
    // Acquire frame
    let (swapchain, device, queue) = (&gfx.swapchain, &gfx.device, &gfx.queue);
    let frame = swapchain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;

    // Create encoder
    let encoder_desc = wgpu::CommandEncoderDescriptor { label: None };
    let mut encoder = device.create_command_encoder(&encoder_desc);

    // Render and submit the queue
    renderer.render(&mut encoder, queue, &frame.view);
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
    let mut renderer = DefaultRenderer::new(&gfx.device, &gfx.swapchain_desc);

    // Create demo mesh and buffers
    let mesh = demo_mesh();
    let buffers = mesh.create_buffers(&gfx.device);

    // Create demo scene
    renderer.set_scene(RendererScene {
        meshes: vec![buffers],
        view: Mat4::look_at_rh((0.0, 0.0, 4.0).into(), Vec3::ZERO, Vec3::Y),
        proj: Mat4::perspective_rh_gl(
            (45.0 as f32).to_radians(),
            gfx.swapchain_desc.width as f32 / gfx.swapchain_desc.height as f32,
            0.1,
            100.0,
        ),
    });

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
    // Initialize logging
    let log_env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(log_env);
    log::info!("Hello there!");

    // Run real entrypoint
    run();
}
