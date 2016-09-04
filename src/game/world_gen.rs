extern crate multimap;
extern crate rand;

use rand::Rng;

use gm2::*;
use std::collections::HashMap;
use super::*;

pub fn create_world<R : Rng>(tiles:&Tiles, rng: &mut R, size:Vec2Size) -> World  {
    let pt = PlacedTile { 
        tile_id: tiles.empty.id, 
        snow: 0, 
    };

    let mut placed_tiles: WorldGrid = vec![vec![pt; size.y as usize]; size.x as usize];

    let tile_count = tiles.all.len();

    for x in 0..size.x {
        for y in 0..size.y {
            let tile_id = rng.gen_range(0, tile_count) as u8;
            placed_tiles[x][y] = PlacedTile { tile_id: tile_id, snow: 0 };
        }
    }

    let climbers_by_tile: multimap::MultiMap<BlockLocation, ClimberId> = multimap::MultiMap::new();
    let climbers_by_id : HashMap<ClimberId, Climber> = HashMap::new();

    World {
        tick: Tick { at: 0 },
        size: size,
        tiles: placed_tiles,
        climbers_by_tile : climbers_by_tile,
        climbers_by_id: climbers_by_id,
    }
}

