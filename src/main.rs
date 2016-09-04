
extern crate gm2;
extern crate grom;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;
extern crate rand;
extern crate ears;

use gm2::*;
use gm2::game::simple;
use glutin::Event;

use grom::game::*;

use rand::SeedableRng;
use rand::Rng;

use std::collections::{VecDeque};

use ears::{Sound, AudioController};

pub const LEVELS : [LevelState; 2] = [
    LevelState {
        size: Vec2Size { x: 4, y: 4 },
        climbers: 40,
        spawn_every: 60,
        spawn_climber_in: 0,
    },
    LevelState {
        size: Vec2Size { x: 8, y: 8 },
        climbers: 40,
        spawn_every: 60,
        spawn_climber_in: 0,
    }
];

use std::env;
use std::str::FromStr;

use glutin::VirtualKeyCode;


fn main() {
    let mut string_args : Vec<String> = Vec::new();
    for argument in env::args() {
        string_args.push(argument);
    }

    let first_opt : Vec<String> = string_args.into_iter().skip(1).collect();
    let level : u32 = first_opt.first().and_then(|my_str| {  
        u32::from_str(my_str).ok()        
    }).unwrap_or(0);
    

    let mut rng = rand::thread_rng();
    let random_seed = [rng.next_u32(), rng.next_u32(), rng.next_u32(), rng.next_u32()];
    // let manual_seed = [1_u32, 2, 3, 4];
    let mut rng = rand::XorShiftRng::from_seed(random_seed);

    let mut level_idx = level;
    let starting_level_state = LEVELS[level_idx as usize];
    
    let window = gm2::render::build_window(String::from("Grom"));

    let mut tile_placement = Sound::new("snd/place_tile.ogg").unwrap();

    let tiles = grom::game::tile::produce_tile_set();
    
    let mut render_state = grom::render::render_state::init(&window, &tiles);
    let mut input_state = input::InputState::default();

     
    let mut game_state = GameState {
        world: create_world(&tiles, &mut rng, starting_level_state.size),
        run_state: RunState::Running,
        tile_queue: VecDeque::new(),
        place_tile_in: Tick { at: 0 },
        uniq_id: 0,
        level_state: starting_level_state,
    };
    
    render_state.camera.at = Vec3::new(game_state.world.size.x as f64 / 2.0, game_state.world.size.y as f64 / 2.0, 0.0);

    let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let mut time = 0.0_f64;

    use cgmath::{Zero};

    let wall_plane = geometry::Plane::from_origin_normal(Vec3::zero(), Vec3::unit_z()); 

    let mut intersection : Option<Vec2i> = None;

    simple::start_loop(|| {
        time = time + (1.0 / 60.0);
        
        render_state.camera.viewport = window.get_framebuffer_dimensions();

        grom::game::advance_game(&mut game_state, &tiles, &mut rng);

        render_state.zoom.advance(0.15, 1.0 / 60.0);
        render_state.camera_target.advance(0.07, 1.0 / 60.0);
        render_state.update_camera_from_springs();
        
        // println!("camera spring {:?}", render_state.camera_target);
        
        grom::render::render(&window, &render_state, &game_state, &tiles, time, color, &intersection);

        let evs : Vec<glutin::Event> = window.poll_events().collect();

        // determining intersection
        let new_input_state = input::produce(&input_state, &evs);
        let (mouse_x, mouse_y) = input_state.mouse.at;
        let line = render_state.camera.ray_for_mouse_position(mouse_x, mouse_y);
        intersection = line.and_then(|l| l.intersects(wall_plane) ).and_then (|v3| {
            let v3i = gm2::round_down_v3(v3);
            let v2i = v3i.truncate();
            if game_state.world.in_bounds(v2i) {
                Some(v2i)
            } else {
                None
            }
        });

        if new_input_state.mouse.mouse_wheel_delta != 0 {
            let zoom_adjust : f64 = if new_input_state.mouse.mouse_wheel_delta > 0 {
                1.1
            } else {
                0.9
            };
            let new_zoom = render_state.zoom.target * zoom_adjust;
            render_state.zoom.target = new_zoom;
        }

        let mut camera_vector : Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if new_input_state.keys.down.contains(&VirtualKeyCode::W) {
            camera_vector.y += 1.
        }
        if new_input_state.keys.down.contains(&VirtualKeyCode::S) {
            camera_vector.y -= 1.
        }
        if new_input_state.keys.down.contains(&VirtualKeyCode::A) {
            camera_vector.x -= 1.
        }
        if new_input_state.keys.down.contains(&VirtualKeyCode::D) {
            camera_vector.x += 1.
        }
        render_state.camera_target.target += camera_vector * 3.0 / 60.0;
        
        // place tile
        if let (Some(is), true) = (intersection, new_input_state.mouse.left_pushed())  {
            // can't place on top of climbers
            if game_state.world.can_place_at(&tiles, is) {
                if let Some((tile_id, _)) = game_state.tile_queue.pop_front() {
                    game_state.world.tiles[is.x as usize][is.y as usize] = PlacedTile {
                        id: tile_id,
                        snow: 0,
                    };
                    tile_placement.play();
                }
            }
        }

        if input_state != new_input_state {
        // println!("input state -> {:?}", new_input_state);
        // println!("line -> {:?} intersection {:?}", line, intersection);
        }
        input_state = new_input_state;

        for event in evs {
            match event {
                Event::Closed | Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return simple::Action::Stop,
                _ => (), // println!("got {:?}", e),
            }
        }

        simple::Action::Continue
    });

}