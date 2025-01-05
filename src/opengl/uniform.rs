use anyhow::Result;

use crate::opengl::program::Program;

#[derive(Debug, Clone)]
pub struct UniformLocations {
    program: Program,
    time: i32,
    dt: i32,
    quad_size: i32,
    mouse_pos: i32,
}

impl UniformLocations {
    pub fn new(program: &Program) -> Result<Self> {
        let time = program.get_uniform_location(c"uTime");
        let quad_size = program.get_uniform_location(c"uQuadSize");
        let dt = program.get_uniform_location(c"uDt");
        let mouse_pos = program.get_uniform_location(c"uMousePos");

        Ok(Self {
            program: program.clone(),
            time,
            quad_size,
            dt,
            mouse_pos,
        })
    }

    pub fn get_time_handle(&self) -> i32 {
        self.time
    }

    pub fn get_dt_handle(&self) -> i32 {
        self.dt
    }

    pub fn get_quad_size_handle(&self) -> i32 {
        self.quad_size
    }

    pub fn get_mouse_pos_handle(&self) -> i32 {
        self.mouse_pos
    }

    pub fn set_time(&self, val: f32) {
        unsafe { gl::UseProgram(self.program.handle()) };
        unsafe { gl::Uniform1f(self.time, val) };
    }

    pub fn set_dt(&self, val: f32) {
        unsafe { gl::UseProgram(self.program.handle()) };
        unsafe { gl::Uniform1f(self.dt, val) };
    }

    pub fn set_quad_size(&self, val: f32) {
        unsafe { gl::UseProgram(self.program.handle()) };
        unsafe { gl::Uniform1f(self.quad_size, val) };
    }

    pub fn set_mouse_pos(&self, val: (f32, f32)) {
        unsafe { gl::UseProgram(self.program.handle()) };
        unsafe { gl::Uniform2f(self.mouse_pos, val.0, val.1) };
    }
}

pub trait SetAllUniformLocations {
    fn set_time(&self, val: f32);
    fn set_dt(&self, val: f32);
    fn set_quad_size(&self, val: f32);
    fn set_mouse_pos(&self, val: (f32, f32));
}

impl SetAllUniformLocations for &[UniformLocations] {
    fn set_time(&self, val: f32) {
        for unif in self.iter() {
            unsafe { gl::UseProgram(unif.program.handle()) };
            unsafe { gl::Uniform1f(unif.time, val) };
        }
    }

    fn set_dt(&self, val: f32) {
        for unif in self.iter() {
            unsafe { gl::UseProgram(unif.program.handle()) };
            unsafe { gl::Uniform1f(unif.time, val) };
        }
    }

    fn set_quad_size(&self, val: f32) {
        for unif in self.iter() {
            unsafe { gl::UseProgram(unif.program.handle()) };
            unsafe { gl::Uniform1f(unif.time, val) };
        }
    }

    fn set_mouse_pos(&self, val: (f32, f32)) {
        for unif in self.iter() {
            unsafe { gl::UseProgram(unif.program.handle()) };
            unsafe { gl::Uniform2f(unif.mouse_pos, val.0, val.1) };
        }
    }
}
