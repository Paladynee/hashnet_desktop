use core::{ptr, time::Duration};
use std::time::Instant;

use crate::opengl::uniform::SetAllUniformLocations;
use anyhow::{Context as AnyhowContextTrait, Result};
use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use super::{
    debugging::gl_initialize_debugging,
    program::Program,
    render::renderstate::RenderState,
    shader::{get_all_shaders, Shader},
    uniform::UniformLocations,
};

pub struct GlobalState<const LEN: usize> {
    pub triplet: Option<GLFWTriplet>,

    pub vshader: Shader,
    pub fshader: Shader,
    pub gshader: Shader,
    pub cshader: Shader,

    pub draw_program: Program,
    pub compute_program: Program,

    pub draw_uniforms: UniformLocations,
    pub compute_uniforms: UniformLocations,

    pub render_state: RenderState<LEN>,
}

impl<const LEN: usize> GlobalState<LEN> {
    pub fn new() -> Result<Self> {
        let triplet = init_glfw()?;

        let [vshader, fshader, gshader, cshader] = get_all_shaders()?;

        let draw_program = Program::try_from_shaders(&[&vshader, &fshader, &gshader])?;
        let compute_program = Program::try_from_shaders(&[&cshader])?;

        let draw_uniforms = UniformLocations::new(&draw_program)?;
        let compute_uniforms = UniformLocations::new(&compute_program)?;

        let render_state = RenderState::new(1280, 720, &draw_program, &compute_program);

        Ok(Self {
            triplet: Some(triplet),
            vshader,
            fshader,
            gshader,
            cshader,
            draw_program,
            compute_program,
            draw_uniforms,
            compute_uniforms,
            render_state,
        })
    }

    pub fn all_uniforms(&self) -> [UniformLocations; 2] {
        [self.draw_uniforms.clone(), self.compute_uniforms.clone()]
    }

    /// # Safety
    ///
    /// gs.triplet must be Some
    pub unsafe fn main_loop(&mut self) {
        unsafe { main_loop(self) }
    }
}

pub fn initialize_opengl() {
    unsafe {
        gl::Viewport(0, 0, 1280, 720);
    }

    gl_initialize_debugging();
}

/// # Safety
///
/// gs.triplet must be Some
unsafe fn main_loop<const LEN: usize>(gs: &mut GlobalState<LEN>) {
    debug_assert!(gs.triplet.is_some(), "UNDEFINED BEHAVIOR: triplet must be Some");
    let GLFWTriplet {
        mut glfw,
        mut window,
        events,
    } = unsafe { gs.triplet.take().unwrap_unchecked() };

    let mut fps_counter = 0;
    let mut fps_counter_last_printed = Instant::now();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(&mut window, event, &mut gs.render_state);
        }

        window.swap_buffers();

        let dt = gs.render_state.last_update.elapsed();
        gs.render_state.last_update = Instant::now();

        gs.all_uniforms().as_slice().set_dt(dt.as_secs_f32());
        gs.all_uniforms().as_slice().set_mouse_pos(gs.render_state.cursor_position);
        gs.all_uniforms().as_slice().set_quad_size(0.03);
        gs.all_uniforms().as_slice().set_time(gs.render_state.start.elapsed().as_secs_f32());

        gs.compute_program.use_program();
        gs.render_state.dispatch_compute_call();
        gs.draw_program.use_program();

        // render_state.update_physics(dt.as_secs_f32());

        fps_counter += 1;
        if fps_counter_last_printed.elapsed() >= Duration::from_secs(1) {
            println!("FPS: {}", fps_counter);
            fps_counter = 0;
            fps_counter_last_printed = Instant::now();
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(gs.render_state.vao);
            gl::DrawArrays(gl::POINTS, 0, gs.render_state.count() as i32);
        }
    }

    gs.triplet = Some(GLFWTriplet { glfw, window, events });
}

fn init_glfw() -> Result<GLFWTriplet> {
    let mut glfw = glfw::init(fail_on_errors).context("Failed to initialize GLFW")?;

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rust Game", glfw::WindowMode::Windowed)
        .context("Failed to create GLFW window")?;

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| {
        let addr = glfw.get_proc_address_raw(symbol);

        if addr.is_null() {
            println!("Failed to load symbol: {}", symbol);
            ptr::null()
        } else {
            addr as *const _
        }
    });

    Ok(GLFWTriplet { glfw, window, events })
}

pub struct GLFWTriplet {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

fn handle_event<const LEN: usize>(window: &mut glfw::Window, event: glfw::WindowEvent, render_state: &mut RenderState<LEN>) {
    match event {
        #[allow(unused_variables)]
        WindowEvent::Key(key, scode, action, modif) => {}

        WindowEvent::Close => {
            window.set_should_close(true);
        }

        WindowEvent::FramebufferSize(w, h) => {
            unsafe {
                gl::Viewport(0, 0, w, h);
            }
            render_state.update_canvas_size(w as usize, h as usize);
        }

        WindowEvent::CursorPos(x, y) => {
            render_state.cursor_position = (x as f32, y as f32);
        }

        _ => {}
    }
}
