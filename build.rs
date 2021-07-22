use glob::glob;
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

const RESOURCES_ROOT: &str = "res";
const SHADERS_DIR: &str = "shaders";

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
struct ShaderDesc {
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderDesc {
    pub fn load(src_path: PathBuf) -> Result<Self, Error> {
        let extension = src_path.extension().map_or("", |x| x.to_str().unwrap());

        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => panic!("Unsupported shader: {}", src_path.display()),
        };

        let cwd = std::env::var("CARGO_MANIFEST_DIR")?;
        let out = std::env::var("OUT_DIR")?;
        let dst_path = Path::new(&out)
            .strip_prefix(cwd)?
            .join(src_path.strip_prefix(RESOURCES_ROOT)?);
        let spv_path = dst_path.with_extension(format!("{}.spv", extension));

        Ok(Self {
            src_path,
            spv_path,
            kind,
        })
    }
}

fn build_shaders() -> Result<(), Error> {
    // Collect all shaders
    let shader_root = PathBuf::from(RESOURCES_ROOT).join(SHADERS_DIR);
    println!("cargo:rerun-if-changed={}", shader_root.to_str().unwrap());
    let mut shader_paths: Vec<_> = ["vert", "frag", "comp"]
        .iter()
        .map(|x| glob(&format!("{}/**/*.{}", shader_root.to_str().unwrap(), x)).unwrap())
        .collect();

    // Load all shader data
    let shaders: Result<Vec<_>, Error> = shader_paths
        .iter_mut()
        .flatten()
        .map(|x| ShaderDesc::load(x?))
        .into_iter()
        .collect();

    // Compiler setup
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_target_env(
        shaderc::TargetEnv::Vulkan,
        shaderc::EnvVersion::Vulkan1_0 as _,
    );
    options.set_include_callback(|name, include_type, parent, _depth| {
        // Build path for include type
        let path = match include_type {
            shaderc::IncludeType::Relative => PathBuf::from(parent).parent().unwrap().join(name),
            shaderc::IncludeType::Standard => shader_root.join("inc").join(name),
        }
        .with_extension("glsl");

        // Notify cargo for rebuilds on change
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

        // Load include
        let src = read_to_string(&path).map_err(|e| e.to_string())?;
        Ok(shaderc::ResolvedInclude {
            resolved_name: name.to_string(),
            content: src,
        })
    });

    // Compile shaders
    for shader in shaders? {
        // Notify cargo for rebuilds on change
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.to_str().unwrap()
        );

        // Load shader
        let src = read_to_string(&shader.src_path)?;

        // Compile
        let compiled = compiler.compile_into_spirv(
            &src,
            shader.kind,
            shader.src_path.to_str().unwrap(),
            "main",
            Some(&options),
        );

        // Handle result
        match compiled {
            Ok(c) => {
                // Write
                create_dir_all(&shader.spv_path.parent().unwrap())?;
                write(&shader.spv_path, c.as_binary_u8())?;
            }
            Err(e) => match e {
                // Pretty panic
                shaderc::Error::CompilationError(_, ce) => panic!("{}", ce),
                _ => panic!("{}", e),
            },
        }
    }

    Ok(())
}

fn main() {
    build_shaders().unwrap();
}
