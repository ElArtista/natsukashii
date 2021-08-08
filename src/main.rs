//
// main.rs
//

extern crate log;

#[macro_use]
mod shader;
mod engine;
mod input;
mod mesh;
mod renderer;
mod uniform;

use engine::{Engine, EngineParams, WindowParams};
use glam::{Mat4, Vec3};
use mesh::{Index, Mesh, Vertex};
use renderer::RendererScene;

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

    // Create demo mesh and buffers
    let mesh = demo_mesh();
    let buffers = mesh.create_buffers(&engine.device);

    // Create demo scene
    let cpos = (0.0, 0.0, 4.0).into();
    let view = Mat4::look_at_rh(cpos, Vec3::ZERO, Vec3::Y);
    let scene = RendererScene {
        meshes: vec![buffers],
        view,
    };
    engine.set_scene(scene);

    // Run
    engine.run();
}
