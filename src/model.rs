//
// model.rs
//

use super::mesh::{Mesh, Vertex};
use glam::Vec3;
use std::{collections::HashMap, ffi::OsStr, io::BufRead, path::Path};
use tobj::{load_mtl_buf, load_obj_buf, LoadOptions};

macro_rules! model_file {
    ($x:expr) => {
        &include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/models/", $x))[..]
    };
}

macro_rules! model_buffers {
    ($d:expr, $($f:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($f.to_owned(), model_file!(concat!($d, "/", $f)));)*
            map
        }
    };
}

#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub mesh_materials: Vec<Option<String>>,
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub albedo: Vec3,
}

impl Model {
    pub fn from_buffers<R: BufRead + Copy>(name: &str, buffers: HashMap<String, R>) -> Self {
        let file = buffers
            .keys()
            .filter(|f| extension_from_filename(f) == "obj")
            .next()
            .unwrap();

        let mut buf = buffers[file];
        let (mdls, mats) = load_obj_buf(
            &mut buf,
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| {
                let mut buf = buffers[p.to_str().unwrap()];
                load_mtl_buf(&mut buf)
            },
        )
        .unwrap();
        let mats = mats.unwrap_or_default();

        let meshes = mdls
            .iter()
            .map(|m| {
                let mesh = &m.mesh;
                let indices = mesh
                    .indices
                    .chunks(3)
                    .map(|i| [i[2], i[1], i[0]])
                    .flatten()
                    .collect();
                let vertices = (0..(mesh.positions.len() / 3))
                    .map(|i| {
                        let pos = &mesh.positions[(i * 3)..(i * 3 + 3)];
                        let invert_z = Vec3::new(1.0, 1.0, -1.0);
                        Vertex::new(Vec3::from_slice(pos) * invert_z)
                    })
                    .collect();
                Mesh { vertices, indices }
            })
            .collect();

        let materials = mats
            .iter()
            .map(|m| Material {
                name: m.name.clone(),
                albedo: Vec3::from_slice(&m.diffuse),
            })
            .collect();

        let mesh_materials = mdls
            .iter()
            .map(|m| {
                let mesh = &m.mesh;
                mesh.material_id.map(|id| mats[id].name.clone())
            })
            .collect();

        Model {
            name: name.to_owned(),
            meshes,
            materials,
            mesh_materials,
        }
    }

    pub fn cornell_box() -> Self {
        let buffers = model_buffers!("cornell_box", "cornell_box.obj", "cornell_box.mtl");
        Self::from_buffers("cornell_box", buffers)
    }
}

fn extension_from_filename(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
}
