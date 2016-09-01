pub mod game_state;
pub mod tile;
pub mod world_gen;

use gm2::{Vec2i};

pub type BlockLocation = Vec2i; 
pub type InnerBlockLocation = Vec2i;
pub type ExactLocation = (BlockLocation, InnerBlockLocation); 

pub const INNER_BLOCK_RESOLUTION : i32 = 16;

pub type TileId = u8;