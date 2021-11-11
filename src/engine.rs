//
// engine.rs
//

use super::{
    camera::{Camera, CameraMoveDirection},
    input::Input,
    renderer::{Renderer, RendererScene},
    scene::Scene,
};

use glam::Vec3;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

/// The Engine
///
/// Manages initialization, lifetime and plumbing of
/// the window, the event loop and the renderer
pub struct Engine {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub input: Input,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub surface_conf: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub renderer: Renderer,
    pub renderer_scene: RendererScene,
    pub scene: Scene,
    pub camera: Camera,
    pub state: EngineState,
}

/// Initialization parameters for Engine
pub struct EngineParams {
    pub window: WindowParams,
}

/// Initialization parameters for Window
pub struct WindowParams {
    pub size: (u32, u32),
}

/// Supplemental engine state
pub struct EngineState {
    pub cursor_grabbed: bool,
}

impl Engine {
    pub async fn new(params: &EngineParams) -> Self {
        // Create window and its event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::<u32>::from(params.window.size))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        // Create input cache
        let input = Input::new();

        // Create wgpu instance, surface and adapter
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY | wgpu::Backends::SECONDARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
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

        // Configure surface
        let size = window.inner_size();
        let surface_format = surface.get_preferred_format(&adapter).unwrap();
        let surface_conf = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_conf);

        // Create the renderer
        let renderer = Renderer::new(&device, &surface_conf);

        // Create default empty scene
        let scene = Scene::default();

        // Create default empty renderer scene
        let renderer_scene = RendererScene::default();

        // Create the camera
        let camera = Camera::new();

        // Initialize supplemental engine state
        let state = EngineState {
            cursor_grabbed: false,
        };

        // Store objects
        Self {
            event_loop: Some(event_loop),
            window,
            input,
            instance,
            surface,
            surface_conf,
            adapter,
            device,
            queue,
            renderer,
            renderer_scene,
            scene,
            camera,
            state,
        }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        // Configure the surface with the new size
        self.surface_conf.width = size.0;
        self.surface_conf.height = size.1;
        self.surface.configure(&self.device, &self.surface_conf);
        // Resize renderer resources
        self.renderer.resize(&self.device, &self.surface_conf);
    }

    pub fn update(&mut self) {
        let dt = 1.0 / 60.0; // TODO: Calculate this

        let camkeys = [
            (VirtualKeyCode::W, CameraMoveDirection::Forward),
            (VirtualKeyCode::A, CameraMoveDirection::Left),
            (VirtualKeyCode::S, CameraMoveDirection::Backward),
            (VirtualKeyCode::D, CameraMoveDirection::Right),
        ];
        let dirs = camkeys
            .iter()
            .filter(|k| self.input.key_held(k.0))
            .map(|k| k.1)
            .collect::<Vec<_>>();
        self.camera.move_to(&dirs, dt);

        if self.state.cursor_grabbed {
            let look_diff = self.input.mouse_diff();
            self.camera.look(look_diff.into(), dt);
        }

        self.camera.update(dt);
        self.scene.view = self.camera.matrix();
        self.renderer_scene.view = self.scene.view;
    }

    pub fn render(&self) {
        // Acquire frame
        let (surface, device, queue) = (&self.surface, &self.device, &self.queue);
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next surface texture");

        // Create encoder
        let encoder_desc = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = device.create_command_encoder(&encoder_desc);

        // Create output view
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render and submit the queue
        self.renderer
            .render(&mut encoder, queue, &view, &self.renderer_scene);
        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn run(mut self) {
        // Workaround the static lifetime requirements of event_loop
        let event_loop = self.event_loop.take().unwrap();

        // Run the mainloop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            self.input.update(&event);
            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::Resized(size) => self.resize(size.into()),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.resize((*new_inner_size).into())
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::MouseInput { button, state, .. }
                            if state == ElementState::Pressed =>
                        {
                            match button {
                                MouseButton::Left => {
                                    self.window.set_cursor_grab(true).unwrap();
                                    self.window.set_cursor_visible(false);
                                    self.state.cursor_grabbed = true;
                                }
                                _ => (),
                            }
                        }
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::RControl),
                                ..
                            } => {
                                self.window.set_cursor_grab(false).unwrap();
                                self.window.set_cursor_visible(true);
                                self.state.cursor_grabbed = false;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                Event::RedrawRequested(_) => {
                    self.update();
                    self.render();
                }
                Event::MainEventsCleared => self.window.request_redraw(),
                _ => (),
            }
        });
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = scene;
        self.renderer_scene = self.renderer.create_scene(&self.device, &self.scene);
    }

    pub fn set_camera_position(&mut self, position: Vec3) {
        self.camera.set_position(position);
    }
}
