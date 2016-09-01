extern crate gm2;
extern crate grom;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use gm2::{Vec3};
use gm2::{input, geometry};
use gm2::game::simple;

use glutin::Event;

fn main() {
  let window = gm2::render::build_window();
  let mut render_state = grom::render::render_state::init(&window);
  let mut input_state = input::InputState::default();
  
  // let mut state = game::GameState { tick: 12 };
  // state = game::update(state);
  let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


  let mut time = 0.0_f64;

  use cgmath::{Zero};

  let wall_plane = geometry::Plane::from_origin_normal(Vec3::zero(), Vec3::unit_z()); 
  
  let mut intersection : Option<Vec3> = None;

  simple::start_loop(|| {
    time = time + (1.0 / 60.0);
    // let cyclical_time = (time % 8.0) / 8.0;
    // println!("cyclical time -> {}", cyclical_time);
    render_state.camera.viewport = window.get_framebuffer_dimensions();
    // render_state.camera.pitch = Rad(cyclical_time);

    grom::render::render(&window, &render_state, time, color, &intersection);

    let evs : Vec<glutin::Event> = window.poll_events().collect();

    let new_input_state = input::produce(&input_state, &evs);
    let (mouse_x, mouse_y) = input_state.mouse.at;
    let line = render_state.camera.ray_for_mouse_position(mouse_x, mouse_y);
    intersection = line.and_then(|l| l.intersects(wall_plane) );

    if input_state != new_input_state {
      // println!("input state -> {:?}", new_input_state);
      // println!("line -> {:?} intersection {:?}", line, intersection);
    }
    input_state = new_input_state;

    for event in evs {
        match event {
            Event::Closed | Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return simple::Action::Stop,
            _ => (),// println!("got {:?}", e)
        }
    }

    simple::Action::Continue
  });
}