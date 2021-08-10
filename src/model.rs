//
// model.rs
//

use super::mesh::{Mesh, Vertex};
use genmesh::{Indexer, LruIndexer, Triangulate, Vertices};
use obj::{IndexTuple, Obj, ObjData};
use std::{collections::HashMap, ffi::OsStr, io::BufRead, path::Path};

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

pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn from_buffers<R: BufRead + Copy>(name: &str, buffers: HashMap<String, R>) -> Self {
        let file = buffers
            .keys()
            .filter(|f| extension_from_filename(f) == "obj")
            .next()
            .unwrap();
        let mut obj = Obj {
            data: ObjData::load_buf(buffers[file]).unwrap(),
            path: file.into(),
        };
        obj.load_mtls_fn(|_, mtllib| Result::Ok(buffers[mtllib]))
            .unwrap();

        let meshes = obj
            .data
            .objects
            .iter()
            .flat_map(|o| &o.groups)
            .map(|g| {
                let mut vertices = vec![];
                let mut indexer = LruIndexer::new(16, |_, t: IndexTuple| {
                    let pos = obj.data.position[t.0];
                    let vtx = Vertex::new(pos.into());
                    vertices.push(vtx)
                });
                let indices = g
                    .polys
                    .iter()
                    .cloned()
                    .map(|p| p.into_genmesh())
                    .triangulate()
                    .vertices()
                    .map(|v| indexer.index(v) as _)
                    .collect::<Vec<_>>();

                Mesh { vertices, indices }
            })
            .collect::<Vec<_>>();

        Model {
            name: name.to_owned(),
            meshes,
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
