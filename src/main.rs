
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

use grom::game::world_gen::*;
use grom::game::game_state::*;

use rand::SeedableRng;
use rand::Rng;

use std::collections::{VecDeque};

use ears::{Sound, AudioController};

fn main() {
    let world_size = Vec2Size::new(4, 4);

    let window = gm2::render::build_window(String::from("Grom"));

    let mut tile_placement = Sound::new("snd/place_tile.ogg").unwrap();

    let tiles = grom::game::tile::produce_tile_set();

    
    let mut render_state = grom::render::render_state::init(&window, &tiles);
    let mut input_state = input::InputState::default();

    let mut rng = rand::XorShiftRng::from_seed([1_u32, 2, 3, 4]); 
    let world = create_world(&tiles, &mut rng, world_size);
    let mut game_state = GameState {
        world:world,
        run_state: RunState::Running,
        tile_queue: VecDeque::new(),
        place_tile_in: Tick { at: 0 },
        uniq_id: 0,
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
        
        render_state.camera.viewport = window.get_framebuffer_dimensions();

        // push tile
        game_state.world = advance(&game_state.world);
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

        // place tile
        if let (Some(is), true) = (intersection, new_input_state.mouse.left_pushed())  {
            if let Some((tile_id, _)) = game_state.tile_queue.pop_front() {
                game_state.world.tiles[is.x as usize][is.y as usize] = PlacedTile {
                    tile_id: tile_id,
                    snow: 0,
                };
                tile_placement.play();
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