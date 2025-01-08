use anyhow::Result;
use gl::types::{GLchar, GLint};

use crate::opengl::shader::Shader;

use core::error::Error;
use core::ffi::CStr;
use core::fmt::{self, Display};
use core::ptr;

#[derive(Debug)]
#[non_exhaustive]
pub enum ProgramError {
    LinkError(String),
    EmptyLog,
    ErrorHandlerError(&'static str),
}

impl Error for ProgramError {}

impl Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LinkError(ref log) => write!(f, "Link error: {}", log),
            Self::EmptyLog => write!(f, "Empty error log"),
            Self::ErrorHandlerError(s) => write!(f, "Program error handler error: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    handle: u32,
}

impl Program {
    pub fn new_empty() -> Self {
        Self {
            handle: unsafe { gl::CreateProgram() },
        }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.handle, shader.handle());
        }
    }

    pub fn link(&self) -> Result<(), ProgramError> {
        unsafe {
            gl::LinkProgram(self.handle);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.handle, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(self.handle, gl::INFO_LOG_LENGTH, &mut len);

                if len != 0 {
                    let mut log = vec![0; len as usize];
                    gl::GetProgramInfoLog(self.handle, len, ptr::null_mut(), log.as_mut_ptr().cast::<GLchar>());

                    let s = match String::from_utf8(log) {
                        Ok(s) => s,
                        Err(_) => return Err(ProgramError::ErrorHandlerError("Failed to convert log to string")),
                    };

                    return Err(ProgramError::LinkError(s));
                } else {
                    return Err(ProgramError::EmptyLog);
                }
            }
        }

        Ok(())
    }

    pub const fn handle(&self) -> u32 {
        self.handle
    }

    pub fn try_from_shaders(shaders: &[&Shader]) -> Result<Self, ProgramError> {
        let prog = Self::new_empty();

        for shader in shaders {
            prog.attach_shader(shader);
        }

        prog.link()?;

        Ok(prog)
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn get_uniform_location(&self, name: &CStr) -> i32 {
        unsafe { gl::GetUniformLocation(self.handle(), name.as_ptr()) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
