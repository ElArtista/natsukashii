//
// mesh.rs
//

use super::geometry::Positions;
use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use std::mem::size_of;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

pub struct MeshBuffers {
    pub vbuf: wgpu::Buffer,
    pub ibuf: wgpu::Buffer,
    pub nelems: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    _pad0: f32,
}

pub type Index = u32;

#[allow(dead_code)]
impl Mesh {
    pub fn create_buffers(&self, device: &wgpu::Device) -> MeshBuffers {
        let vbuf = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_SRC,
        });
        let ibuf = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_SRC,
        });
        let nelems = self.indices.len() as _;
        MeshBuffers { vbuf, ibuf, nelems }
    }
}

impl Positions for Mesh {
    fn iter_pos(&self) -> Box<dyn Iterator<Item = &Vec3> + '_> {
        Box::new(self.vertices.iter().map(|v| &v.position))
    }

    fn iter_pos_mut(&mut self) -> Box<dyn Iterator<Item = &mut Vec3> + '_> {
        Box::new(self.vertices.iter_mut().map(|v| &mut v.position))
    }
}

#[allow(dead_code)]
impl Vertex {
    #[rustfmt::skip]
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
    ];

    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            _pad0: 0.0,
        }
    }

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            step_mode: wgpu::InputStepMode::Vertex,
            array_stride: size_of::<Self>() as _,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub trait IndexFormat {
    fn format() -> wgpu::IndexFormat {
        panic!("Invalid index type");
    }
}

impl IndexFormat for u16 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}

impl IndexFormat for u32 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint32
    }
}
