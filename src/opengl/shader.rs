use anyhow::Result;
use gl::types::{GLchar, GLenum, GLint};

use crate::include_cstr;
use core::error::Error;
use core::ffi::CStr;
use core::fmt::{self, Display};
use core::ptr;

pub const V_SOURCE: &CStr = include_cstr!("./shader_source/vertex.glsl");
pub const F_SOURCE: &CStr = include_cstr!("./shader_source/frag.glsl");
pub const G_SOURCE: &CStr = include_cstr!("./shader_source/geometry.glsl");
pub const C_SOURCE: &CStr = include_cstr!("./shader_source/compute.glsl");

#[derive(Debug)]
#[non_exhaustive]
pub enum ShaderCompileError {
    Compile(GLenum, String),
    EmptyLog(GLenum),
    ErrorHandlerError(GLenum),
}

impl Error for ShaderCompileError {}

impl Display for ShaderCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Compile(ty, ref s) => write!(f, "Error while compiling {} shader:\n\t{}", get_shader_type_name(ty), s),
            Self::EmptyLog(ty) => write!(f, "Error while compiling {} shader:\n\t<empty log>", get_shader_type_name(ty)),
            Self::ErrorHandlerError(ty) => write!(f, "Error while compiling {} shader:\n\t<error handler error>", get_shader_type_name(ty)),
        }
    }
}

pub const fn get_shader_type_name(ty: GLenum) -> &'static str {
    match ty {
        gl::VERTEX_SHADER => "Vertex",
        gl::FRAGMENT_SHADER => "Fragment",
        gl::GEOMETRY_SHADER => "Geometry",
        gl::COMPUTE_SHADER => "Compute",
        _ => "invalid shader type passed to get_shader_type_name",
    }
}

#[derive(Debug)]
pub struct Shader {
    handle: u32,
    ty: GLenum,
}

impl Shader {
    pub fn try_from_source(source: &CStr, shader_type: GLenum) -> Result<Self, ShaderCompileError> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0 as GLint;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                if len != 0 {
                    let mut log = vec![0; len as usize];
                    gl::GetShaderInfoLog(shader, log.capacity() as i32, ptr::null_mut(), log.as_mut_ptr().cast::<GLchar>());
                    let s = match String::from_utf8(log) {
                        Ok(s) => s,
                        Err(_) => return Err(ShaderCompileError::ErrorHandlerError(shader_type)),
                    };
                    return Err(ShaderCompileError::Compile(shader_type, s));
                } else {
                    return Err(ShaderCompileError::EmptyLog(shader_type));
                }
            }

            Ok(Self {
                handle: shader,
                ty: shader_type,
            })
        }
    }

    pub const fn handle(&self) -> u32 {
        self.handle
    }

    pub const fn ty(&self) -> GLenum {
        self.ty
    }
}

pub fn get_all_shaders() -> Result<[Shader; 4]> {
    let vertex_shader = Shader::try_from_source(V_SOURCE, gl::VERTEX_SHADER)?;
    let frag_shader = Shader::try_from_source(F_SOURCE, gl::FRAGMENT_SHADER)?;
    let geometry_shader = Shader::try_from_source(G_SOURCE, gl::GEOMETRY_SHADER)?;
    let compute_shader = Shader::try_from_source(C_SOURCE, gl::COMPUTE_SHADER)?;

    Ok([vertex_shader, frag_shader, geometry_shader, compute_shader])
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}
