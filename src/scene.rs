//
// scene.rs
//

use super::{
    mesh::Mesh,
    renderer::{RendererScene, RendererSceneObject},
    uniform::TransformUniform,
};
use glam::Mat4;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub view: Mat4,
}

pub struct SceneObject {
    pub meshes: Vec<Mesh>,
    pub transform: Mat4,
}

impl RendererScene {
    pub fn create(
        device: &wgpu::Device,
        scene: &Scene,
        transform_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let objects = scene
            .objects
            .iter()
            .map(|o| RendererSceneObject::create(device, o, transform_layout))
            .collect();
        let view = scene.view;
        RendererScene { objects, view }
    }
}

impl RendererSceneObject {
    pub fn create(
        device: &wgpu::Device,
        object: &SceneObject,
        transform_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let meshes = object
            .meshes
            .iter()
            .map(|m| m.create_buffers(device))
            .collect();
        let transform = TransformUniform {
            model: object.transform,
        }
        .create_bind_group(device, transform_layout);
        RendererSceneObject { meshes, transform }
    }
}
