use super::*;

use gm2::*;
use rand::SeedableRng;
use rand::Rng;

pub fn advance_game<R : Rng>(game_state: &mut GameState, tiles:&Tiles, rng: &mut R) {
    let new_world = advance_world(&game_state.world);

    game_state.world = new_world;

    // handle giving hte player tiles
    if game_state.running() {
        if game_state.place_tile_in.at == 0 {
            if game_state.tile_queue.len() < 5 {
                let tile = &tiles.safe[rng.gen_range(0, tiles.safe.len())];
                let next_uniq = game_state.next_id();
                game_state.tile_queue.push_back((tile.id, next_uniq));
            }
            game_state.place_tile_in = tick(300);
        } else {
            game_state.place_tile_in = game_state.place_tile_in.pred();
        }
    }
}

pub fn spawn_climber(game_state: &mut GameState) {
    let climber_id = game_state.next_id();
    let now = game_state.world.tick;
    game_state.world.register_climber(climber_spawning_at(climber_id, vec2i(0, 0), now));
}

pub fn advance_world(world:&World) -> World {
    let now = world.tick;

    World {
        tick: now.succ(),
        size: world.size,
        tiles: world.tiles.clone(),
        climbers_by_tile: world.climbers_by_tile.clone(),
        climbers_by_id: world.climbers_by_id.clone(),
    }
}