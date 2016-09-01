use std::collections;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    id: u32,
    // all unified data
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PlacedTile {
    tile_id: u8,
    snow: u8, // counter of dispatching rocks?
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Climber {
    id: u32,
    morale: i8, // regenerates with time or tent, degrades on tragedy, negative is panicced
}

const WORLD_SIZE: usize = 100;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameState {
    pub tick: Tick,
    // pub tiles: [PlacedTile; 100],
}

pub fn update(starting:GameState) -> GameState {
    let abc = GameState { tick: starting.tick.succ(), .. starting };
    abc
}
