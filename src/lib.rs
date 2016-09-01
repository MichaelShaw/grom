#![crate_name="grom"]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;
extern crate gm2;

pub mod game {
    pub mod game_state;
}
pub mod render;
