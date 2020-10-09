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
    for vert in verts.to_vec() {
        let content = std::fs::read_to_string(vert).unwrap();

        let mut output = bevy_glsl_to_spirv::compile(&content, bevy_glsl_to_spirv::ShaderType::Vertex, None).unwrap();
        let mut spv_bytes = Vec::new();
        output.read_to_end(&mut spv_bytes).unwrap();

        std::fs::write(format!("{}.spv", vert), spv_bytes.as_slice()).unwrap();
    }
    for frag in frags.to_vec() {
        let content = std::fs::read_to_string(frag).unwrap();

        let mut output = bevy_glsl_to_spirv::compile(&content, bevy_glsl_to_spirv::ShaderType::Fragment, None).unwrap();
        let mut spv_bytes = Vec::new();
        output.read_to_end(&mut spv_bytes).unwrap();

        std::fs::write(format!("{}.spv", frag), spv_bytes.as_slice()).unwrap();
    }
}
