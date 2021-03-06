
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

use ears::{Sound, Music, AudioController};

pub const LEVELS : [LevelState; 5] = [
    LevelState {
        size: Vec2Size { x: 4, y: 4 },
        climbers: 40,
        spawn_every: 60,
        spawn_climber_in: 0,
        speed: 1,
    },
    LevelState {
        size: Vec2Size { x: 8, y: 8 },
        climbers: 40,
        spawn_every: 60,
        spawn_climber_in: 0,
        speed: 1,
    },
    LevelState {
        size: Vec2Size { x: 16, y: 16 },
        climbers: 60,
        spawn_every: 60,
        spawn_climber_in: 0,
        speed: 2,
    },
    LevelState {
        size: Vec2Size { x: 32, y: 32 },
        climbers: 60,
        spawn_every: 60,
        spawn_climber_in: 0,
        speed: 4,
    },
    LevelState {
        size: Vec2Size { x: 64, y: 64 },
        climbers: 600,
        spawn_every: 5,
        spawn_climber_in: 0,
        speed: 8,
    }
];

use std::env;
use std::str::FromStr;

use glutin::VirtualKeyCode;
use cgmath::InnerSpace;

fn main() {
    let mut string_args : Vec<String> = Vec::new();
    for argument in env::args() {
        string_args.push(argument);
    }

    let first_opt : Vec<String> = string_args.into_iter().skip(1).collect();
    let level : u32 = first_opt.first().and_then(|my_str| {  
        u32::from_str(my_str).ok()        
    }).unwrap_or(0);

    let mut music = Music::new("snd/come.and.find.me.ogg").unwrap();
    music.set_volume(0.5);
    music.play();
    

    let mut threaded_rng = rand::thread_rng();
    let random_seed = [threaded_rng.next_u32(), threaded_rng.next_u32(), threaded_rng.next_u32(), threaded_rng.next_u32()];
    // let manual_seed = [1_u32, 2, 3, 4];
    let mut rng = rand::XorShiftRng::from_seed(random_seed);

    let level_idx = level;
    let starting_level_state = LEVELS[level_idx as usize];
    
    let window = gm2::render::build_window(String::from("Grom"), starting_level_state.speed == 1);

    let mut tile_placement = Sound::new("snd/place_tile.ogg").unwrap();
    let mut walk_sound = Sound::new("snd/walk.ogg").unwrap();
    walk_sound.set_looping(true);
    walk_sound.set_volume(0.0);
    walk_sound.play();

    let tiles = grom::game::tile::produce_tile_set();
    
    let mut render_state = grom::render::render_state::init(&window, &tiles);

    for _ in 0..200 {
        let x = rng.next_f64() * 60.0 - 30.0;
        let y = rng.next_f64() * 60.0 - 30.0;
        let z = rng.next_f64() * 3.0;
        render_state.cloud_positions.push(Vec3::new(x, y, z));
    }
    
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

    // println!(" speed is -> {:?}", starting_level_state.speed);

    simple::start_loop(starting_level_state.speed, || {
        time = time + (1.0 / 60.0);
        
        render_state.camera.viewport = window.get_framebuffer_dimensions();
        
        grom::game::advance_game(&mut game_state, &tiles, &mut rng);
         

        render_state.zoom.advance(0.15, 1.0 / 60.0);
        render_state.camera_target.advance(0.07, 1.0 / 60.0);
        render_state.update_camera_from_springs();

        let camera_at = render_state.camera_target.position;

        // ears::listener::set_position([camera_at.x as f32, camera_at.y as f32, camera_at.z as f32]);

        let mut total_vel = 0.0_f64;
        for (_,css) in &render_state.entity_springs {
            total_vel += css.spring.velocity.magnitude();
        }
        let volume = total_vel.log(2.0) / 4.0;
        walk_sound.set_volume(clamp(volume as f32, 0.0, 0.45));
        
        grom::render::render(&window, &mut render_state, &game_state, &tiles, time, color, &intersection);

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

        let zoom_adjust : f64 = if new_input_state.mouse.mouse_wheel_delta > 0 || new_input_state.keys.pushed.contains(&VirtualKeyCode::Z) {
            1.0
        } else if new_input_state.mouse.mouse_wheel_delta < 0 || new_input_state.keys.pushed.contains(&VirtualKeyCode::X) {
            -1.0
        } else {
            0.0
        };
        
        let new_zoom : f64 = clamp(render_state.zoom.target + zoom_adjust, 1.0_f64, 64.0_f64);
        render_state.zoom.target = new_zoom;
        

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
        render_state.camera_target.target += camera_vector * 30.0 / 60.0 * 1.0 / render_state.zoom.target;
        
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