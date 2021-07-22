#[macro_export]
macro_rules! shader_file {
    ($x:expr) => {
        concat!(env!("OUT_DIR"), "/shaders/", $x, ".spv")
    };
}

#[macro_export]
macro_rules! include_shader {
    ($x:expr) => {
        wgpu::include_spirv!(concat!(env!("OUT_DIR"), "/shaders/", $x, ".spv"))
    };
}
