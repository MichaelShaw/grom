extern crate gm2;

use std::collections::{HashMap, HashSet};
use gm2::{Vec2i};
use cgmath::vec2;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlacedTile {
    tile_id: u8,
    snow: u8, // counter of dispatching rocks?
    climbers: HashSet<Climber>,
}

pub type ExactLocation = (Vec2i, Vec2i); // block location, inner-block location

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

const WORLD_SIZE: usize = 100;

pub struct GameState {
    pub tick: Tick,
    pub tiles: [[PlacedTile; 40]; 100],
    pub guys: HashMap<ClimberId, Climber>, 
}

pub fn update(starting:GameState) -> GameState {
    let abc = GameState { tick: starting.tick.succ(), .. starting };
    abc
}
