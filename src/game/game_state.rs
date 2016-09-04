extern crate gm2;
extern crate multimap;

use std::collections::{HashMap, VecDeque};

use super::*;
use gm2::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RunState {
    Paused,
    Running
}

pub type UniqGameId = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LevelState {
    pub size: Vec2Size,
    pub climbers: u32,
    pub spawn_every: u32,
    pub spawn_climber_in: u32,
}

pub struct GameState {
    pub world:World,
    pub run_state: RunState,
    pub tile_queue: VecDeque<(TileId, UniqGameId)>,
    pub place_tile_in: Tick,
    pub uniq_id: UniqGameId,
    pub level_state: LevelState,
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
    pub id: TileId,
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


const ADJACENT_TILES : [Vec2i; 8] = [
    Vec2i { x:-1, y: -1 },
    Vec2i { x:-1, y: 0 },
    Vec2i { x:-1, y: 1 },
    Vec2i { x:0, y: -1 },
    Vec2i { x:0, y: 1 },
    Vec2i { x:1, y: -1 },
    Vec2i { x:1, y: 0 },
    Vec2i { x:1, y: 1 },
];

pub fn tiles_adjacent_to(v:Vec2i) -> Vec<Vec2i> {
    ADJACENT_TILES.into_iter().map(|t| t + v).collect()
}

pub fn absolute_location(bl:Vec2i, ibl:Vec2i) -> Vec2i {
    ibl + bl * 16
}

pub fn can_travel(from:Vec2i, from_tile:&Tile, to:Vec2i, to_tile:&Tile) -> Option<InnerBlockLocation> {
    for tl in &to_tile.nodes {
        let target_abs = absolute_location(to, *tl);
        if from_tile.nodes.iter().any(|fl| absolute_location(from, *fl) == target_abs) {
            return Some(*tl)
        }
    } 

    None
}

impl World {
    pub fn travellable_locations(&self, from:Vec2i, tiles:&Tiles) -> Vec<(Vec2i, Vec2i)> {
        let from_tile = tiles.with_id(self.tile_at(from).id);

        self.adjacent_locations(from).into_iter().filter_map( |tl| {
            let to_tile = tiles.with_id(self.tile_at(tl).id);
            can_travel(from, from_tile, tl, to_tile).map(|il| (tl, il))
        }).collect()
    }

    pub fn adjacent_locations(&self, loc:Vec2i) -> Vec<Vec2i> {
        let adjacent_tiles = tiles_adjacent_to(loc); 
        adjacent_tiles.into_iter().filter(|&l| self.in_bounds(l)).collect()
    }

    pub fn can_place_at(&self, tiles:&Tiles, v:Vec2i) -> bool {
        if !self.climbers_by_tile.contains_key(&v) {
            let tile = self.tile_at(v);
            // check tile for safety
            if tiles.is_safe(tile.id) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn tile_at(&self, v:Vec2i) -> PlacedTile {
        self.tiles[v.x as usize][v.y as usize]
    }

    pub fn register_climber(&mut self, climber:Climber) {
        self.register_climber_locations(&climber);
        self.climbers_by_id.insert(climber.id, climber);
    }

    pub fn unregister_climber_locations(&mut self, climber:&Climber) {
        let mut remove_next : bool = false;
        if let Some(climbers) = self.climbers_by_tile.get_vec_mut(&climber.next.loc) {
            climbers.retain(|&e| e != climber.id);
            remove_next = climbers.is_empty();
        }
        if remove_next {
            self.climbers_by_tile.remove(&climber.next.loc);
        }

        let mut remove_prev : bool = false;
        if let Some(climbers) = self.climbers_by_tile.get_vec_mut(&climber.prev.loc) {
            climbers.retain(|&e| e != climber.id);
            remove_prev = climbers.is_empty();
        }
        if remove_prev {
            self.climbers_by_tile.remove(&climber.prev.loc);
        }
    }

    pub fn register_climber_locations(&mut self, climber: &Climber) {
        self.climbers_by_tile.insert(climber.next.loc, climber.id);
        self.climbers_by_tile.insert(climber.prev.loc, climber.id);
    }

    pub fn in_bounds(&self, v:BlockLocation) -> bool {
        v.x >= 0 && v.x < (self.size.x as i32) && v.y >= 0 && v.y < (self.size.y as i32)
    }
}

