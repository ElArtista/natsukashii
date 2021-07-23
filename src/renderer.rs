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
    scene: RendererScene,
}

impl DefaultRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let vsrc = include_shader!("demo.vert");
        let fsrc = include_shader!("demo.frag");
        let vshader = device.create_shader_module(&vsrc);
        let fshader = device.create_shader_module(&fsrc);

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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&vp_layout],
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
                targets: &[target_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        DefaultRenderer {
            pipeline,
            vp_buffer,
            vp_bind_group,
            scene: RendererScene::default(),
        }
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
        let scene = &self.scene;
        queue.write_buffer(
            &self.vp_buffer,
            0,
            bytemuck::bytes_of(&ViewProjUniform {
                view: scene.view,
                proj: scene.proj,
            }),
        );

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
        rpass.set_bind_group(0, &self.vp_bind_group, &[]);

        for mesh in &scene.meshes {
            rpass.set_vertex_buffer(0, mesh.vbuf.slice(..));
            rpass.set_index_buffer(mesh.ibuf.slice(..), Index::format());
            rpass.draw_indexed(0..mesh.nelems, 0, 0..1);
        }
    }
}
