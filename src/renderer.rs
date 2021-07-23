use crate::{
    mesh::{Index, IndexFormat, MeshBuffers, Vertex},
    uniform::ViewProjUniform,
};
use glam::Mat4;

pub trait Renderer {
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
    );
}

#[derive(Default)]
pub struct RendererScene {
    pub meshes: Vec<MeshBuffers>,
    pub view: Mat4,
    pub proj: Mat4,
}

pub struct DefaultRenderer {
    pipeline: wgpu::RenderPipeline,
    vp_buffer: wgpu::Buffer,
    vp_bind_group: wgpu::BindGroup,
    depth_texture_view: wgpu::TextureView,
    scene: RendererScene,
}

impl DefaultRenderer {
    pub fn new(device: &wgpu::Device, swapchain_desc: &wgpu::SwapChainDescriptor) -> Self {
        // Setup view projetion uniform
        let vp_data = ViewProjUniform::default();
        let vp_layout = ViewProjUniform::layout(&device);
        let vp_buffer = vp_data.create_buffer(&device);
        let vp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &vp_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vp_buffer.as_entire_binding(),
            }],
        });

        // Setup depth texture
        let depth_format = wgpu::TextureFormat::Depth32Float;
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: swapchain_desc.width,
                height: swapchain_desc.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: depth_format,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            label: None,
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Setup forward pass
        let pipeline =
            Self::build_forward_pass(device, &vp_layout, swapchain_desc.format, depth_format);

        DefaultRenderer {
            pipeline,
            vp_buffer,
            vp_bind_group,
            depth_texture_view,
            scene: RendererScene::default(),
        }
    }

    fn build_forward_pass(
        device: &wgpu::Device,
        vp_layout: &wgpu::BindGroupLayout,
        color_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let vsrc = include_shader!("demo.vert");
        let fsrc = include_shader!("demo.frag");
        let vshader = device.create_shader_module(&vsrc);
        let fshader = device.create_shader_module(&fsrc);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[vp_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vshader,
                entry_point: "main",
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fshader,
                entry_point: "main",
                targets: &[color_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
        });

        pipeline
    }

    pub fn set_scene(&mut self, scene: RendererScene) {
        self.scene = scene;
    }
}

impl Renderer for DefaultRenderer {
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
    ) {
        // Update view projection uniform
        let scene = &self.scene;
        queue.write_buffer(
            &self.vp_buffer,
            0,
            bytemuck::bytes_of(&ViewProjUniform {
                view: scene.view,
                proj: scene.proj,
            }),
        );

        {
            // Begin render pass
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            // Set pipeline and bind groups
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.vp_bind_group, &[]);

            // Draw meshes
            for mesh in &scene.meshes {
                rpass.set_vertex_buffer(0, mesh.vbuf.slice(..));
                rpass.set_index_buffer(mesh.ibuf.slice(..), Index::format());
                rpass.draw_indexed(0..mesh.nelems, 0, 0..1);
            }
        }
    }
}
