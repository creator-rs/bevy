use super::ShaderLayout;
use bevy_asset::Handle;
use std::marker::Copy;

/// The stage of a shader
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

#[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
impl Into<bevy_glsl_to_spirv::ShaderType> for ShaderStage {
    fn into(self) -> bevy_glsl_to_spirv::ShaderType {
        match self {
            ShaderStage::Vertex => bevy_glsl_to_spirv::ShaderType::Vertex,
            ShaderStage::Fragment => bevy_glsl_to_spirv::ShaderType::Fragment,
            ShaderStage::Compute => bevy_glsl_to_spirv::ShaderType::Compute,
        }
    }
}

#[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
fn glsl_to_spirv(
    glsl_source: &str,
    stage: ShaderStage,
    shader_defs: Option<&[String]>,
) -> Vec<u32> {
    bevy_glsl_to_spirv::compile(glsl_source, stage.into(), shader_defs).unwrap()
}

#[cfg(any(target_os = "ios", target_os = "android"))]
impl Into<shaderc::ShaderKind> for ShaderStage {
    fn into(self) -> shaderc::ShaderKind {
        match self {
            ShaderStage::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStage::Fragment => shaderc::ShaderKind::Fragment,
            ShaderStage::Compute => shaderc::ShaderKind::Compute,
        }
    }
}

// impl Into<naga::ShaderStage> for ShaderStage {
//     fn into(self) -> naga::ShaderStage {
//         match self {
//             ShaderStage::Vertex => naga::ShaderStage::Vertex,
//             ShaderStage::Fragment => naga::ShaderStage::Fragment,
//             ShaderStage::Compute => naga::ShaderStage::Compute,
//         }
//     }
// }

// fn glsl_to_spirv(
//     glsl_source: &str,
//     stage: ShaderStage,
//     shader_defs: Option<&[String]>,
// ) -> Vec<u32> {
//     let mut options: naga::FastHashMap<String, String> = Default::default();
//     if let Some(shader_defs) = shader_defs {
//         for def in shader_defs.iter() {
//             options.insert(def.to_owned(), "".to_owned());
//         }
//     }
//     let module = naga::front::glsl::parse_str(glsl_source, "main", stage.into(), options).unwrap();
//     naga::back::spv::Writer::new(&module.header, naga::back::spv::WriterFlags::NONE).write(&module)
// }

#[cfg(any(target_os = "ios", target_os = "android"))]
fn glsl_to_spirv(
    glsl_source: &str,
    stage: ShaderStage,
    shader_defs: Option<&[String]>,
) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    if let Some(shader_defs) = shader_defs {
        for def in shader_defs.iter() {
            options.add_macro_definition(def, None);
        }
    }

    let binary_result = compiler
        .compile_into_spirv(
            glsl_source,
            stage.into(),
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap();

    binary_result.as_binary().to_vec()
}

fn bytes_to_words(bytes: &[u8]) -> Vec<u32> {
    let mut words = Vec::new();
    for bytes4 in bytes.chunks(4) {
        words.push(u32::from_le_bytes([
            bytes4[0], bytes4[1], bytes4[2], bytes4[3],
        ]));
    }

    words
}

/// The full "source" of a shader
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShaderSource {
    Spirv(Vec<u32>),
    Glsl(String),
}

impl ShaderSource {
    pub fn spirv_from_bytes(bytes: &[u8]) -> ShaderSource {
        ShaderSource::Spirv(bytes_to_words(bytes))
    }
}

/// A shader, as defined by its [ShaderSource] and [ShaderStage]
#[derive(Clone, Debug)]
pub struct Shader {
    pub source: ShaderSource,
    pub stage: ShaderStage,
}

impl Shader {
    pub fn new(stage: ShaderStage, source: ShaderSource) -> Shader {
        Shader { stage, source }
    }

    pub fn from_spirv(stage: ShaderStage, spirv: &[u8]) -> Shader {
        Shader {
            source: ShaderSource::Spirv(bytes_to_words(spirv)),
            stage,
        }
    }

    pub fn from_glsl(stage: ShaderStage, glsl: &str) -> Shader {
        Shader {
            source: ShaderSource::Glsl(glsl.to_string()),
            stage,
        }
    }

    pub fn get_spirv(&self, macros: Option<&[String]>) -> Vec<u32> {
        match self.source {
            ShaderSource::Spirv(ref bytes) => bytes.clone(),
            ShaderSource::Glsl(ref source) => glsl_to_spirv(&source, self.stage, macros),
        }
    }

    pub fn get_spirv_shader(&self, macros: Option<&[String]>) -> Shader {
        Shader {
            source: ShaderSource::Spirv(self.get_spirv(macros)),
            stage: self.stage,
        }
    }

    pub fn reflect_layout(&self, enforce_bevy_conventions: bool) -> Option<ShaderLayout> {
        if let ShaderSource::Spirv(ref spirv) = self.source {
            Some(ShaderLayout::from_spirv(
                spirv.as_slice(),
                enforce_bevy_conventions,
            ))
        } else {
            panic!("Cannot reflect layout of non-SpirV shader. Try compiling this shader to SpirV first using self.get_spirv_shader()");
        }
    }
}

/// All stages in a shader program
#[derive(Clone, Debug)]
pub struct ShaderStages {
    pub vertex: Handle<Shader>,
    pub fragment: Option<Handle<Shader>>,
}

impl ShaderStages {
    pub fn new(vertex_shader: Handle<Shader>) -> Self {
        ShaderStages {
            vertex: vertex_shader,
            fragment: None,
        }
    }
}
