use super::*;

use gm2::*;
use rand::Rng;

pub fn advance_game<R : Rng>(game_state: &mut GameState, tiles:&Tiles, rng: &mut R) {
    let new_world = advance_world(&game_state.world, tiles, rng);

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

        if game_state.level_state.climbers > 0 { 
            if game_state.level_state.spawn_climber_in == 0 {
                spawn_climber(game_state);
                game_state.level_state.spawn_climber_in = game_state.level_state.spawn_every;
                game_state.level_state.climbers -= 1;
            } else {
                game_state.level_state.spawn_climber_in -= 1;
            }
        }

    }
}

pub fn spawn_climber(game_state: &mut GameState) {
    let climber_id = game_state.next_id();
    let now = game_state.world.tick;
    game_state.world.register_climber(climber_spawning_at(climber_id, vec2i(0, 0), now));
}

pub fn advance_world<R : Rng>(world:&World, tiles: &Tiles, rng: &mut R) -> World {
    let now = world.tick;
    // println!("advance the world now -> {:?}", now);
    // println!("climber registration -> {:?}", world.climbers_by_tile);

    let mut new_world = World {
        tick: now.succ(),
        size: world.size,
        tiles: world.tiles.clone(),
        climbers_by_tile: world.climbers_by_tile.clone(),
        climbers_by_id: world.climbers_by_id.clone(),
    };

    for (_, climber) in &world.climbers_by_id {
        if climber.next.at <= now {
            let current_loc = climber.next.loc;
            let tile_id = new_world.tile_at(current_loc).id;
            let tile = tiles.with_id(tile_id);

            let mut travellable_adjacents : Vec<(Vec2i, Vec2i)> = new_world.travellable_locations(current_loc, tiles);

            println!("travellables -> {:?}", travellable_adjacents);

            new_world.unregister_climber_locations(climber);

            let new_climber = if !travellable_adjacents.is_empty() {
                travellable_adjacents.sort_by_key(|&(bl, il)| {
                    (bl != current_loc, il.y < current_loc.y) // not my current loc is good
                });

                println!("sorted -> {:?}", travellable_adjacents);

                // we have somewhere to travel to if we want
                let (nbl, nil) = travellable_adjacents[rng.gen_range(0, travellable_adjacents.len())];

                println!("travelling to {:?} {:?}", nbl, nil);
                travel_to(climber, now, nbl, nil)
            } else {
                // take the preferred thingy
                println!("idling");
                idle(climber, now, 60)
            };
             
            new_world.register_climber_locations(&new_climber);
            new_world.climbers_by_id.insert(new_climber.id, new_climber);
        }
    }

    new_world
}

extern crate cgmath;
use cgmath::InnerSpace;

fn travel_to(climber: &Climber, at:Tick, loc:Vec2i, inner_loc: Vec2i) -> Climber {
    let from_abs = absolute_location(climber.next.loc, climber.next.inner_loc);
    let to_abs = absolute_location(loc, inner_loc);

    println!("from_abs {:?} to_abs {:?}", from_abs, to_abs);

    let diff = to_abs - from_abs;
    let v = Vec2::new(diff.x as f64, diff.y as f64).magnitude();
    let duration : u64 = (v / 4.0 * 60.0) as u64;

    println!("travel distance {:?}, duration {:?}", v, duration);     

    let next = TimedLocation {
        loc: loc ,
        inner_loc: inner_loc,
        at: tick(at.at + duration),
    };

    Climber {
        prev: climber.next,
        next: next,
        .. *climber
    }
}

fn idle(climber:&Climber, at:Tick, duration:u64) -> Climber {
    let mut next = climber.next;
    next.at = tick(at.at + duration); // set next in the duration

    Climber {
        prev: climber.next,
        next: next,
        .. *climber
    }
}
