extern crate gm2;
extern crate multimap;

use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

// use super::{ExactLocation, BlockLocation};
use super::*;
use gm2::*;

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

pub type UniqGameId = u64;

pub struct GameState {
    pub world:World,
    pub run_state: RunState,
    pub tile_queue: VecDeque<(TileId, UniqGameId)>,
    pub place_tile_in: Tick,
    pub uniq_id: UniqGameId,
}

impl GameState {
    pub fn next_id(&mut self) -> UniqGameId {
        let next_id = self.uniq_id;
        self.uniq_id += 1;
        next_id
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlacedTile {
    pub tile_id: TileId,
    pub snow: u8, // counter of dispatching rocks?
}

pub type WorldGrid = Vec<Vec<PlacedTile>>; 

pub struct World {
    pub tick: Tick,
    pub size : Vec2Size,
    pub tiles: WorldGrid,
    pub climbers_by_tile : multimap::MultiMap<BlockLocation, ClimberId>,
    pub climbers_by_id: HashMap<ClimberId, Climber>,
}

impl World {
    pub fn in_bounds(&self, v:Vec2i) -> bool {
        v.x >= 0 && v.x < (self.size.x as i32) && v.y >= 0 && v.y < (self.size.y as i32)
    }
}

pub fn advance(world:&World) -> World {
    World {
        tick: world.tick.succ(),
        size: world.size,
        tiles: world.tiles.clone(),
        climbers_by_tile: world.climbers_by_tile.clone(),
        climbers_by_id: world.climbers_by_id.clone(),
    }
}