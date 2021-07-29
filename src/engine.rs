//
// engine.rs
//

use super::renderer::{DefaultRenderer, Renderer, RendererScene};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

/// Initialization parameters for Engine
pub struct EngineParams {
    pub window: WindowParams,
}

/// Initialization parameters for Window
pub struct WindowParams {
    pub size: (u32, u32),
}

/// The Engine
///
/// Manages initialization, lifetime and plumbing of
/// the window, the event loop and the renderer
pub struct Engine {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swapchain: wgpu::SwapChain,
    pub swapchain_desc: wgpu::SwapChainDescriptor,
    pub renderer: DefaultRenderer,
}

impl Engine {
    pub async fn new(params: &EngineParams) -> Self {
        // Create window and its event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::<u32>::from(params.window.size))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        // Create wgpu instance, surface and adapter
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY | wgpu::BackendBit::SECONDARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Create swapchain
        let size = window.inner_size();
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();
        let swapchain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swapchain = device.create_swap_chain(&surface, &swapchain_desc);

        // Create the renderer
        let renderer = DefaultRenderer::new(&device, &swapchain_desc);

        // Store objects
        Self {
            event_loop: Some(event_loop),
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            swapchain,
            swapchain_desc,
            renderer,
        }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        // Recreate the swap chain with the new size
        self.swapchain_desc.width = size.0;
        self.swapchain_desc.height = size.1;
        self.swapchain = self
            .device
            .create_swap_chain(&self.surface, &self.swapchain_desc);
    }

    pub fn render(&self) {
        // Acquire frame
        let (swapchain, device, queue) = (&self.swapchain, &self.device, &self.queue);
        let frame = swapchain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;

        // Create encoder
        let encoder_desc = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = device.create_command_encoder(&encoder_desc);

        // Render and submit the queue
        self.renderer.render(&mut encoder, queue, &frame.view);
        queue.submit(Some(encoder.finish()));
    }

    pub fn run(mut self) {
        // Workaround the static lifetime requirements of event_loop
        let event_loop = self.event_loop.take().unwrap();

        // Run the mainloop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::Resized(size) => self.resize(size.into()),
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => (),
                        },
                        _ => (),
                    }
                }
                Event::RedrawRequested(_) => self.render(),
                Event::MainEventsCleared => self.window.request_redraw(),
                _ => (),
            }
        });
    }

    pub fn set_scene(&mut self, scene: RendererScene) {
        self.renderer.set_scene(scene);
    }
}
