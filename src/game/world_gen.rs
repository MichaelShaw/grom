extern crate multimap;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use super::game_state::*;
use super::tile::*;
use std::collections::HashMap;
use super::{BlockLocation};
// use rand::Rng;

pub fn create_world<R : Rng>(tiles:Tiles, bullshit: &mut R) -> World  {
    let pt = PlacedTile { 
        tile_id: tiles.empty.id, 
        snow: 0, 
    };
    let mut placed_tiles: [[PlacedTile; 100]; 100] = [[pt; 100]; 100];

    let tile_count = tiles.all.len();
    

    // let mut shit = rand::thread_rng();

    // let between = Range::new(0, tiles.all.len());

    for x in 0..100 {
        for y in 0..100 {
            let tile_id = bullshit.gen_range(0, tile_count) as u8;
            placed_tiles[x][y] = PlacedTile { tile_id: tile_id, snow: 0 };
            // let xyz = between.ind_sample(mut rr);
            
        }
    }

    let climbers_by_tile: multimap::MultiMap<BlockLocation, ClimberId> = multimap::MultiMap::new();
    let climbers_by_id : HashMap<ClimberId, Climber> = HashMap::new();

    World {
        tick: Tick { at: 0 },
        tiles: placed_tiles,
        climbers_by_tile : climbers_by_tile,
        climbers_by_id: climbers_by_id,
    }
}

