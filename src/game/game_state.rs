extern crate gm2;
extern crate multimap;

use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

use super::{ExactLocation, BlockLocation};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tick {
    pub at: u64
}

impl Tick {
    pub fn succ(&self) -> Tick {
        tick(self.at + 1)
    }
}

pub fn tick(at:u64) -> Tick {
    Tick { at: at }
}

// pub const GROUND_TILE : Tile = Tile { id: 0, nodes: vec![Vec2i { x: 0, y: 0 }] };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlacedTile {
    pub tile_id: u8,
    pub snow: u8, // counter of dispatching rocks?
}

// block location, inner-block location

pub type ClimberId = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Climber {
    id: ClimberId,
    morale: i8, // regenerates with time or tent, degrades on tragedy, negative is paniced
    at: ExactLocation,
}

impl Hash for Climber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub enum RunState {
    Paused,
    Running
}

pub struct GameState {
    pub world:World,
    pub run_state: RunState,
}

pub struct World {
    pub tick: Tick,
    pub tiles: [[PlacedTile; 100]; 100],
    pub climbers_by_tile : multimap::MultiMap<BlockLocation, ClimberId>,
    pub climbers_by_id: HashMap<ClimberId, Climber>,
}