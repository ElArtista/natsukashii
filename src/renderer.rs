pub trait Renderer {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

pub struct DefaultRenderer {
    pipeline: wgpu::RenderPipeline,
}

impl DefaultRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let vsrc = include_shader!("fullscreen.vert");
        let fsrc = include_shader!("demo.frag");
        let vshader = device.create_shader_module(&vsrc);
        let fshader = device.create_shader_module(&fsrc);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vshader,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fshader,
                entry_point: "main",
                targets: &[target_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        DefaultRenderer { pipeline }
    }
}

impl Renderer for DefaultRenderer {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&self.pipeline);
        rpass.draw(0..3, 0..1);
    }
}
