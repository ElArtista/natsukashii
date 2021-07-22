pub trait Renderer {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

pub struct DefaultRenderer {}

impl DefaultRenderer {
    pub fn new() -> Self {
        DefaultRenderer {}
    }
}

impl Renderer for DefaultRenderer {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
    }
}
