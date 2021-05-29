use std::{ffi::OsStr, fs};

use shaderc::{Compiler, ShaderKind};

fn compile_shaders() {
    let mut compiler = Compiler::new().expect("Unable to create shader compiler");

    for entry in fs::read_dir("src/graphics/shaders/")
        .expect("Unable to read the src/graphics/shaders/ directory")
    {
        let entry = entry.expect("Unable to read entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                let shader_kind = match extension {
                    "frag" => Some(ShaderKind::Fragment),
                    "vert" => Some(ShaderKind::Vertex),
                    "comp" => Some(ShaderKind::Compute),
                    _ => None,
                };

                if let Some(shader_kind) = shader_kind {
                    let code = fs::read_to_string(path.clone())
                        .expect("Unable to read shader source code");

                    let new_path = path.with_extension(format!("{}.spv", extension));
                    let src_path = path
                        .to_str()
                        .expect("Unable to convert shader path to string");

                    let compiled = compiler
                        .compile_into_spirv(&code, shader_kind, src_path, "main", None)
                        .expect("Shader compilation failed");

                    fs::write(new_path, compiled.as_binary_u8())
                        .expect("Failed to write the compiled shader");
                }
            }
        }
    }
}

fn main() {
    compile_shaders();
}
