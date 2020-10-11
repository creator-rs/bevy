use std::io::Read;

fn main() {    
    let verts = [
        "./crates/bevy_ui/src/render/ui.vert",
        "./crates/bevy_sprite/src/render/sprite.vert",
        "./crates/bevy_sprite/src/render/sprite_sheet.vert",
        "./crates/bevy_pbr/src/render_graph/forward_pipeline/forward.vert",
    ];
    let frags = [
        "./crates/bevy_ui/src/render/ui.frag",
        "./crates/bevy_sprite/src/render/sprite.frag",
        "./crates/bevy_sprite/src/render/sprite_sheet.frag",
        "./crates/bevy_pbr/src/render_graph/forward_pipeline/forward.frag",
    ];
    build_shaders(verts.to_vec(), bevy_glsl_to_spirv::ShaderType::Vertex);
    build_shaders(frags.to_vec(), bevy_glsl_to_spirv::ShaderType::Fragment);
}

fn build_shaders(paths: Vec<&str>, shader_type: bevy_glsl_to_spirv::ShaderType) {
    for path in paths.to_vec() {
        let content = std::fs::read_to_string(path).unwrap();
        let mut output = bevy_glsl_to_spirv::compile(&content, shader_type.clone(), None).unwrap();
        let mut spv_bytes = Vec::new();
        output.read_to_end(&mut spv_bytes).unwrap();
        std::fs::write(format!("{}.spv", path), spv_bytes.as_slice()).unwrap();
    }
}
