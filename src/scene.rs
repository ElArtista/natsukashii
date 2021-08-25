//
// scene.rs
//

use super::mesh::Mesh;
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
