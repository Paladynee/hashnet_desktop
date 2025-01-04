#![allow(unused)]
use gl::types::{GLchar, GLenum, GLint};

use crate::shader::Shader;

use std::fmt::{self, Display};
use std::ptr;

#[derive(Debug)]
pub struct Program {
    handle: u32,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ProgramError {
    LinkError(String),
    EmptyLog,
    ErrorHandlerError(&'static str),
}

impl core::error::Error for ProgramError {}

impl Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProgramError::LinkError(ref log) => write!(f, "Link error: {}", log),
            ProgramError::EmptyLog => write!(f, "Empty error log"),
            ProgramError::ErrorHandlerError(s) => write!(f, "Program error handler error: {}", s),
        }
    }
}

impl Program {
    #[inline]
    pub fn new_empty() -> Self {
        Self {
            handle: unsafe { gl::CreateProgram() },
        }
    }

    #[inline]
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.handle, shader.handle());
        }
    }

    #[inline]
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

    #[inline]
    pub fn try_from_shaders(shaders: &[&Shader]) -> Result<Program, ProgramError> {
        let prog = Program::new_empty();

        for shader in shaders {
            prog.attach_shader(shader);
        }

        prog.link()?;

        Ok(prog)
    }

    #[inline]
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
