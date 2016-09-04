pub mod game_state;
pub mod tile;
pub mod world_gen;
pub mod climber;
pub mod update;
pub mod tick;

use gm2::{Vec2i};

pub type BlockLocation = Vec2i; 
pub type InnerBlockLocation = Vec2i;

pub const INNER_BLOCK_RESOLUTION : i32 = 16; // that's down to the pixel you fool ... interesting ... why not?!

pub type TileId = u8;

pub use self::game_state::*;
pub use self::tile::*;
pub use self::world_gen::*;
pub use self::climber::*;
pub use self::update::*;
pub use self::tick::*;