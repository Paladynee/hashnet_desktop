use std::{ffi::c_void, mem, ptr, time::Instant};

use voxell_rng::{getrandom::MagicSeed, rng::XorShift128};

use crate::opengl::program::Program;
use crate::opengl::render::particle::RenderData;
use crate::vec2::Vector2;

pub struct RenderState<const OBJECTS_AMT: usize> {
    pub buffer: RenderData<OBJECTS_AMT>,
    pub rng: XorShift128,

    pub last_update: Instant,
    pub start: Instant,

    pub cursor_position: (f32, f32),
    pub unit_vec: Vector2,

    pub can_w: usize,
    pub can_h: usize,
    pub vao: u32,
    pub vbo: u32,
}

impl<const LEN: usize> RenderState<LEN> {
    pub fn new(can_w: usize, can_h: usize, draw_program: &Program, compute_program: &Program) -> Self {
        let seed = [
            MagicSeed::u64().expect("fix your OS, couldn't get OS entropy"),
            MagicSeed::u64().expect("fix your OS, couldn't get OS entropy"),
        ];
        let mut rng = XorShift128::wrap(seed);

        let data = RenderData::new(&mut rng);

        let mut vao = 0;
        let mut vbo = 0;

        initialize_buffers(draw_program, &data, &mut vao, &mut vbo);

        let start = Instant::now();
        let unit_vec = Vector2::new(0.1f32 / can_w as f32, 0.1f32 / can_w as f32);
        Self {
            buffer: data,
            vao,
            vbo,
            rng,
            start,
            can_w,
            unit_vec,
            can_h,
            cursor_position: (0.0, 0.0),
            last_update: Instant::now(),
        }
    }
    pub const fn count(&self) -> usize {
        LEN
    }

    pub const fn update_canvas_size(&mut self, w: usize, h: usize) {
        self.can_w = w;
        self.can_h = h;
        self.unit_vec = Vector2::new(0.1f32 / w as f32, 0.1f32 / h as f32);
    }

    pub fn dispatch_compute_call(&self) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, self.vbo);
            let num_groups = (LEN as f32 / 64.0).ceil() as u32;
            gl::DispatchCompute(num_groups, 1, 1);
            gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
        }
    }
}

pub fn initialize_buffers<const LEN: usize>(draw_program: &Program, data: &RenderData<LEN>, vao: &mut u32, vbo: &mut u32) {
    unsafe {
        gl::UseProgram(draw_program.handle());

        gl::GenVertexArrays(1, vao);
        gl::BindVertexArray(*vao);

        gl::GenBuffers(1, vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(data.data().as_slice()) as isize,
            data.data().as_ptr().cast::<c_void>(),
            gl::DYNAMIC_DRAW,
        )
    }
}

//
// pub fn update_buffer_data(&self) {
//     unsafe {
//         gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
//         gl::BufferSubData(
//             gl::ARRAY_BUFFER,
//             0,
//             mem::size_of_val(self.buf.data.as_slice()) as isize,
//             self.buf.data.as_ptr().cast::<c_void>(),
//         );
//     }
// }
//
// pub fn update_physics(&mut self, dt: f32) {
//     self.buf.data.par_iter_mut().for_each(|obj| {
//         obj.acc.set_vec(&obj.pos);
//         obj.acc.one_over_d_sq(
//             &Vector2 {
//                 x: self.cursor_position.0,
//                 y: self.cursor_position.1,
//             },
//             &self.unit_vec,
//         );
//         obj.vel.add(obj.acc.x * dt, obj.acc.y * dt);
//         obj.pos.add(obj.vel.x * dt, obj.vel.y * dt);
//     });
//     self.update_buffer_data();
// }

impl<const LEN: usize> Drop for RenderState<LEN> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
