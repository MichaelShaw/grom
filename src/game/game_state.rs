extern crate gm2;
extern crate multimap;

use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

// use super::{ExactLocation, BlockLocation};
use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tick {
    pub at: u64
}

impl Tick {
    pub fn succ(&self) -> Tick {
        tick(self.at + 1)
    }

    pub fn pred(&self) -> Tick {
        tick(self.at - 1)
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
    pub tile_queue: Vec<TileId>,
    pub place_tile_in: Tick, 
}


pub const WORLD_SIZE : usize = 100;
pub type WorldGrid = [[PlacedTile; WORLD_SIZE];WORLD_SIZE]; 

pub struct World {
    pub tick: Tick,
    pub tiles: WorldGrid,
    pub climbers_by_tile : multimap::MultiMap<BlockLocation, ClimberId>,
    pub climbers_by_id: HashMap<ClimberId, Climber>,
}

pub fn advance(world:&World) -> World {
    World {
        tick: world.tick.succ(),
        tiles: world.tiles,
        climbers_by_tile: world.climbers_by_tile.clone(),
        climbers_by_id: world.climbers_by_id.clone(),
    }
}