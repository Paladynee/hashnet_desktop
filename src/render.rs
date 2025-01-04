use std::{
    ffi::c_void,
    mem,
    time::{Duration, Instant},
};

pub struct RenderState {
    data: Vec<f32>,
    vao: u32,
    vbo: u32,
    last_update: Instant,
}

impl RenderState {
    #[inline]
    pub fn from_data(data: Vec<f32>) -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(data.as_slice()) as isize,
                data.as_ptr().cast::<c_void>(),
                gl::STREAM_DRAW,
            );
        }

        Self {
            data,
            vao,
            vbo,
            last_update: Instant::now(),
        }
    }

    #[inline]
    pub fn vbo(&self) -> u32 {
        self.vbo
    }

    #[inline]
    pub fn vao(&self) -> u32 {
        self.vao
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.data.len() / 3
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.last_update.elapsed()
    }

    #[inline]
    pub fn update_last_update(&mut self) {
        self.last_update = Instant::now();
    }

    #[inline]
    pub fn update_buffer_data(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                mem::size_of_val(self.data.as_slice()) as isize,
                self.data.as_ptr().cast::<c_void>(),
            );
        }
    }

    #[inline]
    pub fn update_physics(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        // let mut i = 0;
        // while i < self.data.len() {
        //     self.data[i] += 0.1 * dt;
        //     i += 3;
        // }

        self.update_buffer_data();
    }
}

impl Drop for RenderState {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
