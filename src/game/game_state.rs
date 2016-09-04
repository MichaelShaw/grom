extern crate gm2;
extern crate multimap;

use std::collections::{HashMap, VecDeque};

use super::*;
use gm2::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tick {
    pub at: u64
}

impl Tick {
    pub fn plus(&self, n:u64) -> Tick {
        tick(self.at + n)
    }

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    pub fn running(&self) -> bool {
        self.run_state == RunState::Running
    }

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
    pub fn register_climber(&mut self, climber:Climber) {
        self.register_climber_locations(&climber);
        self.climbers_by_id.insert(climber.id, climber);
    }

    pub fn unregister_climber_locations(&mut self, climber:&Climber) {
        // self.climbers_by_tile.remove(climber.next.loc, climber.id);
        // self.climbers_by_tile.insert(climber.prev.loc, climber.id);
    }

    pub fn register_climber_locations(&mut self, climber: &Climber) {
        self.climbers_by_tile.insert(climber.next.loc, climber.id);
        self.climbers_by_tile.insert(climber.prev.loc, climber.id);
    }

    pub fn in_bounds(&self, v:BlockLocation) -> bool {
        v.x >= 0 && v.x < (self.size.x as i32) && v.y >= 0 && v.y < (self.size.y as i32)
    }
}

