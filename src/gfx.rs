use winit::window::Window;

#[allow(dead_code)]
pub struct Gfx {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swapchain: wgpu::SwapChain,
    pub swapchain_desc: wgpu::SwapChainDescriptor,
}

impl Gfx {
    pub async fn new(window: &Window) -> Self {
        // Create wgpu instance, surface and adapter
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY | wgpu::BackendBit::SECONDARY);
        let surface = unsafe { instance.create_surface(window) };
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

        // Store objects
        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swapchain,
            swapchain_desc,
        }
    }

    pub fn configure_swapchain(&mut self, size: (u32, u32)) {
        // Recreate the swap chain with the new size
        self.swapchain_desc.width = size.0;
        self.swapchain_desc.height = size.1;
        self.swapchain = self
            .device
            .create_swap_chain(&self.surface, &self.swapchain_desc);
    }
}
