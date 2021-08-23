//
// renderer.rs
//

use crate::{
    mesh::{Index, IndexFormat, MeshBuffers, Vertex},
    uniform::ViewProjUniform,
};
use glam::Mat4;

/// The Renderer
///
/// Manages GPU specific objects and performs the rendering
pub struct Renderer {
    view_proj: ViewProj,
    demo_pass: DemoPass,
}

#[derive(Default)]
pub struct RendererScene {
    pub meshes: Vec<MeshBuffers>,
    pub view: Mat4,
}

#[allow(dead_code)]
struct ViewProj {
    data: ViewProjUniform,
    buffer: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

struct DemoPass {
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, surface_conf: &wgpu::SurfaceConfiguration) -> Self {
        // Setup view projetion uniform
        let view_proj_data = ViewProjUniform {
            proj: Mat4::perspective_lh(
                (45.0f32).to_radians(),
                surface_conf.width as f32 / surface_conf.height as f32,
                0.1,
                100.0,
            ),
            ..Default::default()
        };
        let view_proj_layout = ViewProjUniform::layout(&device);
        let view_proj_buffer = view_proj_data.create_buffer(&device);
        let view_proj_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &view_proj_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_proj_buffer.as_entire_binding(),
            }],
        });

        // Setup demo pass
        let demo_pass = DemoPass::new(device, surface_conf, &view_proj_layout);

        Renderer {
            view_proj: ViewProj {
                data: view_proj_data,
                buffer: view_proj_buffer,
                layout: view_proj_layout,
                bind_group: view_proj_bind_group,
            },
            demo_pass,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, surface_conf: &wgpu::SurfaceConfiguration) {
        // Recreate surface dependent passes
        self.demo_pass = DemoPass::new(device, surface_conf, &self.view_proj.layout);
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        scene: &RendererScene,
    ) {
        // Update view projection uniform
        let vp = &self.view_proj;
        queue.write_buffer(
            &vp.buffer,
            0,
            bytemuck::bytes_of(&ViewProjUniform {
                view: scene.view,
                ..vp.data
            }),
        );

        // Make demo pass
        self.demo_pass.execute(encoder, view, &vp.bind_group, scene);
    }
}

impl DemoPass {
    pub fn new(
        device: &wgpu::Device,
        surface_conf: &wgpu::SurfaceConfiguration,
        view_proj_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let vsrc = include_shader!("demo.vert");
        let fsrc = include_shader!("demo.frag");
        let vshader = device.create_shader_module(&vsrc);
        let fshader = device.create_shader_module(&fsrc);

        let depth_format = wgpu::TextureFormat::Depth32Float;
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: surface_conf.width,
                height: surface_conf.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: depth_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[view_proj_layout],
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
                targets: &[surface_conf.format.into()],
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

        DemoPass {
            pipeline,
            depth_texture_view,
        }
    }

    fn execute(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        color_texture_view: &wgpu::TextureView,
        view_proj_bind_group: &wgpu::BindGroup,
        scene: &RendererScene,
    ) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: color_texture_view,
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

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &view_proj_bind_group, &[]);

        for mesh in &scene.meshes {
            rpass.set_vertex_buffer(0, mesh.vbuf.slice(..));
            rpass.set_index_buffer(mesh.ibuf.slice(..), Index::format());
            rpass.draw_indexed(0..mesh.nelems, 0, 0..1);
        }
    }
}
