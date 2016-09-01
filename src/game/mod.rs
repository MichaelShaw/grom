pub mod game_state;
pub mod tile;

use gm2::{Vec2i};

pub type InnerBlockLocation = Vec2i;

pub const INNER_BLOCK_RESOLUTION : i32 = 16;