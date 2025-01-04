#![allow(unused)]
use gl::types::{GLchar, GLenum, GLint};

use super::include_cstr;
use std::ffi::CStr;
use std::fmt::{self, Display};
use std::ptr;

pub const V_SOURCE: &CStr = include_cstr!("./vertex.glsl");
pub const F_SOURCE: &CStr = include_cstr!("./frag.glsl");

#[derive(Debug)]
#[non_exhaustive]
pub enum ShaderCompileError {
    Compile(String),
    EmptyLog,
    ErrorHandlerError,
}

impl core::error::Error for ShaderCompileError {}

impl Display for ShaderCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Compile(ref s) => write!(f, "Compile error: {}", s),
            Self::EmptyLog => write!(f, "Empty error log"),
            Self::ErrorHandlerError => write!(f, "Shader compile error handler error"),
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    handle: u32,
    ty: GLenum,
}

impl Shader {
    #[inline]
    pub fn try_from_source(source: &CStr, shader_type: GLenum) -> Result<Shader, ShaderCompileError> {
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
                        Err(_) => return Err(ShaderCompileError::ErrorHandlerError),
                    };
                    return Err(ShaderCompileError::Compile(s));
                } else {
                    return Err(ShaderCompileError::EmptyLog);
                }
            }

            Ok(Shader::new_unchecked(shader, shader_type))
        }
    }

    /// # Safety
    ///
    /// Created handle must be a valid shader object.
    #[inline]
    pub const unsafe fn new_unchecked(handle: u32, ty: GLenum) -> Self {
        debug_assert!(matches!(ty, gl::VERTEX_SHADER | gl::FRAGMENT_SHADER | gl::GEOMETRY_SHADER));
        Self { handle, ty }
    }

    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn ty(&self) -> GLenum {
        self.ty
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}
