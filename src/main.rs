extern crate anyhow;
extern crate gl;
extern crate glfw;
extern crate libc;

use std::ptr;

use anyhow::{Context as AnyhowContextTrait, Result};
use glfw::{fail_on_errors, Context, WindowEvent};
use program::Program;
use render::RenderState;
use shader::{Shader, F_SOURCE, V_SOURCE};

mod macros;
mod program;
mod render;
mod shader;

fn main() -> Result<()> {
    let mut glfw = glfw::init(fail_on_errors).context("Failed to initialize GLFW").unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Rust Game", glfw::WindowMode::Windowed)
        .context("Failed to create GLFW window")
        .unwrap();

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

    let vertex_shader = Shader::try_from_source(V_SOURCE, gl::VERTEX_SHADER)?;
    let frag_shader = Shader::try_from_source(F_SOURCE, gl::FRAGMENT_SHADER)?;
    let program = Program::try_from_shaders(&[&vertex_shader, &frag_shader])?;

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let mut render_state = RenderState::from_data(vertices.to_vec());

    // Main loop
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        window.swap_buffers();

        let dt = render_state.elapsed();
        render_state.update_last_update();
        render_state.update_physics(dt);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            program.use_program();
            gl::BindVertexArray(render_state.vao());
            gl::DrawArrays(gl::TRIANGLES, 0, render_state.count() as i32);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        #[allow(unused)]
        WindowEvent::Key(key, scode, action, modif) => {}

        WindowEvent::Close => {
            window.set_should_close(true);
        }

        _ => {}
    }
}
