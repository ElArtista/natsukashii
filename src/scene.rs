//
// scene.rs
//

use super::mesh::Mesh;
use glam::{Mat4, Vec3};

#[derive(Default, Debug)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub view: Mat4,
}

#[derive(Default, Debug)]
pub struct SceneObject {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Option<(Vec3,)>>,
    pub transform: Mat4,
}
