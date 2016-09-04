extern crate multimap;
extern crate rand;

use rand::Rng;

use gm2::*;
use std::collections::HashMap;
use super::*;

fn place(id: TileId) -> PlacedTile {
    PlacedTile { 
        id: id, 
        snow: 0, 
    } 
}

pub fn create_world<R : Rng>(tiles:&Tiles, rng: &mut R, size:Vec2Size) -> World  {
    let pt = PlacedTile { 
        id: tiles.default.id, 
        snow: 0, 
    };

    let mut placed_tiles: WorldGrid = vec![vec![pt; size.y as usize]; size.x as usize];

    let tile_count = tiles.safe.len();

    for x in 0..size.x {
        for y in 0..size.y {
            let tile = &tiles.safe[rng.gen_range(0, tile_count)];
            let tile_id = tile.id;
            placed_tiles[x][y] = PlacedTile { id: tile_id, snow: 0 };
        }
    }

    let climbers_by_tile: multimap::MultiMap<BlockLocation, ClimberId> = multimap::MultiMap::new();
    let climbers_by_id : HashMap<ClimberId, Climber> = HashMap::new();

    // place objectives + spawn points
    let spawner = tiles.with_name("starting_steps").unwrap();
    let flat = tiles.with_name("flat").unwrap();
    let stone_head = tiles.with_name("stone_head").unwrap();

    placed_tiles[0][0] = place(spawner.id);
    placed_tiles[1][0] = place(flat.id);

    let top_row = size.y - 1;

    placed_tiles[rng.gen_range(0, size.x)][top_row] = place(stone_head.id); 
    
    World {
        tick: Tick { at: 0 },
        size: size,
        tiles: placed_tiles,
        climbers_by_tile : climbers_by_tile,
        climbers_by_id: climbers_by_id,
    }
}

