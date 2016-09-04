
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
use gm2::audio::*;
use glutin::Event;

use grom::game::world_gen::*;
use grom::game::game_state::*;

use rand::SeedableRng;
use rand::Rng;

use std::collections::{VecDeque, HashSet};
// use std::cell::RefCell;
// use std::rc::Rc;

use ears::{Sound, AudioController, SoundData};

fn play_file(file: &str) {
    // Create a new Sound.
    let mut snd = Sound::new(file).unwrap();

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}

fn main() {
    let window = gm2::render::build_window(String::from("Grom"));

    let mut tile_placement = Sound::new("snd/place_tile.ogg").unwrap();

    let tiles = grom::game::tile::produce_tile_set();

    
    let mut render_state = grom::render::render_state::init(&window, &tiles);
    let mut input_state = input::InputState::default();

    let mut rng = rand::XorShiftRng::from_seed([1_u32, 2, 3, 4]); 
    let world = create_world(&tiles, &mut rng);
    let mut game_state = GameState {
        world:world,
        run_state: RunState::Running,
        tile_queue: VecDeque::new(),
        place_tile_in: Tick { at: 0 },
    };
  
    // let mut state = game::GameState { tick: 12 };
    // state = game::update(state);
    let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


    let mut time = 0.0_f64;

    use cgmath::{Zero};

    let wall_plane = geometry::Plane::from_origin_normal(Vec3::zero(), Vec3::unit_z()); 

    let mut intersection : Option<Vec2i> = None;

    simple::start_loop(|| {
        time = time + (1.0 / 60.0);
        // let cyclical_time = (time % 8.0) / 8.0;
        // println!("cyclical time -> {}", cyclical_time);


        render_state.camera.viewport = window.get_framebuffer_dimensions();

        game_state.world = advance(&game_state.world);
        if game_state.place_tile_in.at == 0 {
            if game_state.tile_queue.len() < 5 {
                let tile_id = rng.gen_range(0, tiles.all.len()) as u8;
                game_state.tile_queue.push_back(tile_id);
            }
            game_state.place_tile_in = tick(300);
        } else {
            game_state.place_tile_in = game_state.place_tile_in.pred();
        }

        // println!("time is {:?}", game_state.world.tick);

        grom::render::render(&window, &render_state, &game_state, time, color, &intersection);

        let evs : Vec<glutin::Event> = window.poll_events().collect();

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

        if let (Some(is), true) = (intersection, new_input_state.mouse.left_pushed())  {
            if let Some(tile_id) = game_state.tile_queue.pop_front() {
                game_state.world.tiles[is.x as usize][is.y as usize] = PlacedTile {
                    tile_id: tile_id,
                    snow: 0,
                };
                tile_placement.play();
            }
        }

        // let tile_id = game_state.tile_queue.remove(0);

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