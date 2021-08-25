//
// main.rs
//

extern crate log;

#[macro_use]
mod shader;
mod camera;
mod engine;
mod geometry;
mod input;
mod mesh;
mod model;
mod renderer;
mod scene;
mod uniform;

use engine::{Engine, EngineParams, WindowParams};
use geometry::Centered;
use glam::{Mat4, Vec3};
use mesh::{Index, Mesh, Vertex};
use model::Model;
use scene::{Scene, SceneObject};

#[allow(dead_code)]
fn demo_mesh() -> Mesh {
    use genmesh::{
        generators::{IcoSphere, IndexedPolygon, SharedVertex},
        Triangulate,
    };

    let prim = IcoSphere::subdivide(1);
    let vertices: Vec<Vertex> = prim
        .shared_vertex_iter()
        .map(|v| Vertex::new(v.pos.into()))
        .collect();
    let indices: Vec<Index> = prim
        .indexed_polygon_iter()
        .triangulate()
        .map(|f| vec![f.x as _, f.y as _, f.z as _])
        .flatten()
        .collect();

    Mesh { vertices, indices }
}

fn main() {
    // Initialize logging
    let log_env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(log_env);
    log::info!("Hello there!");

    // Prepare the engine params
    let params = EngineParams {
        window: WindowParams { size: (1280, 720) },
    };

    // Create the engine
    let mut engine = futures::executor::block_on(Engine::new(&params));

    // Create cornell box
    let model = Model::cornell_box();
    let meshes = model.meshes.centered();
    let materials = model
        .mesh_materials
        .iter()
        .map(|m| {
            m.as_ref()
                .map(|m| {
                    model
                        .materials
                        .iter()
                        .filter(|x| *m == x.name)
                        .next()
                        .map(|m| (m.albedo,))
                })
                .flatten()
        })
        .collect();

    // Create demo scene
    let cpos = (0.0, 0.0, -3.5).into();
    let view = Mat4::look_at_lh(cpos, Vec3::ZERO, Vec3::Y);
    let scene = Scene {
        objects: vec![SceneObject {
            meshes,
            materials,
            transform: Mat4::IDENTITY,
        }],
        view,
    };
    engine.set_scene(scene);
    engine.set_camera_position(cpos);

    // Run
    engine.run();
}
