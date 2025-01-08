#![allow(unused_imports)]
extern crate anyhow;
extern crate gl;
extern crate glfw;
extern crate rayon;

use anyhow::Result;
use opengl::global_state::GlobalState;
use rayon::{prelude::*, ThreadPoolBuilder};

pub mod macros;
pub mod opengl;
pub mod vec2;

fn main() -> Result<()> {
    let mut global_state: GlobalState<128> = GlobalState::new()?;
    // Safety: we just initialized global_state, triplet can't be None.
    unsafe { global_state.main_loop() };

    Ok(())
}
