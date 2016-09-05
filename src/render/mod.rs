extern crate glium;
extern crate cgmath;

pub mod render_state;

use cgmath::Rad;
use gm2::*;
use gm2::render::*;
use gm2::camera::*;
use gm2::color::*;
use glium::index;
use glium::{Surface};
use game::*;
use self::render_state::*;
use cgmath::InnerSpace;



pub fn render(display: &glium::Display, rs:&mut render_state::RenderState, game_state:&GameState, tiles:&Tiles, time:f64, color: [f32; 4], intersection: &Option<Vec2i>) {
    let time_delta = 1.0 / 60.0;
    let tesselator_scale = Vec3::new(rs.base_units_per_pixel(), rs.base_units_per_pixel(), rs.base_units_per_pixel());

    let mut tesselator = GeometryTesselator::new(tesselator_scale);
    let ok_indicator = rs.texture.at(0, 0);
    let bad_indicator = rs.texture.at(0, 1);

    let world_size = game_state.world.size;
    let now = game_state.world.tick;
    
    for x in 0..(world_size.x as usize) {
        for y in 0..(world_size.y as usize) {
            let tile_id = game_state.world.tiles[x][y].id as usize;
            let texture_region = &rs.tile_renderers[tile_id];
            tesselator.draw_wall_tile(&texture_region, 0, x as f64, y as f64, 0.0, 0.0, false);
            tesselator.draw_wall_tile(&texture_region, 1, x as f64, y as f64, 0.0, 0.1, false);
            tesselator.draw_wall_tile(&texture_region, 2, x as f64, y as f64, 0.0, 0.2, false);    
        }
    }

    for (i, &pos) in rs.cloud_positions.iter().enumerate() {
        let cloud_renderer = rs.cloud_renderers[i % 4];
        let mut render_pos = Vec3::new(pos.x, pos.y, -8.0 + pos.z);
        render_pos -= rs.camera_target.position * pos.z * 0.25; // flexible parallax
        tesselator.draw_wall_centre_anchored_at(&cloud_renderer, 0, render_pos, 0.0, false)
    }
    
    
    for (_, climber) in game_state.world.climbers_by_id.iter() {
        let exact_location = climber.exact_location_at(now, 0.0);
        let climber_idx = (climber.id as usize) % rs.climber_renderers.len();
        let climber_region = &rs.climber_renderers[climber_idx][0];

        let climber_state = rs.entity_springs.entry(climber.id).or_insert(ClimberRenderState::new(exact_location));
    
        climber_state.spring.target = exact_location;
        climber_state.spring.advance(1.0, time_delta);

        let seed = ((climber.id as f64 * 1732.0) % 17.0) / 17.0;

        let mut pos = climber_state.spring.position;

        
        if climber_state.spring.velocity.x.abs() * 1.2 > climber_state.spring.velocity.y.abs() { // so we're still "walking" on 45 degree steps
             climber_state.walk_progress += climber_state.spring.velocity.magnitude() * time_delta;
            let up = ((climber_state.walk_progress + seed) % 0.05) < 0.025;
            if up {
                pos.y += 0.02;
            }
        } 
        pos.x += (climber_state.walk_progress * 64.0).sin() * 0.0025;
     
       
        let depth_offset = seed * 0.20 + 0.11;
        tesselator.draw_wall_base_anchored_at(climber_region, 0, pos, depth_offset, false);
    }

    if let &Some(its) = intersection {
        // show held tile
        if let Some(&(tile_id, _)) = game_state.tile_queue.front() {
            let texture_region = &rs.tile_renderers[tile_id as usize];
            let x = its.x as f64;
            let y = its.y as f64;
            tesselator.draw_wall_tile(&texture_region, 0, x as f64, y as f64, 0.0, 0.3, false);
            tesselator.draw_wall_tile(&texture_region, 1, x as f64, y as f64, 0.0, 0.4, false);
            tesselator.draw_wall_tile(&texture_region, 2, x as f64, y as f64, 0.0, 0.5, false);    
        }

        let indicator = if game_state.world.can_place_at(tiles, its) {
            ok_indicator
        } else {
            bad_indicator
        };
        tesselator.draw_wall_tile(&indicator, 0, its.x as f64, its.y as f64, 0.0, 0.55, false);
    }

    let vertex_buffer = glium::VertexBuffer::persistent(display,&tesselator.tesselator.vertices).unwrap();

    let mvp_raw : [[f64; 4]; 4] = rs.camera.view_projection().into();
    let mvp_raw_downsized = down_size_m4(mvp_raw);

    let nearest_neighbour_texture = rs.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).minify_filter(glium::uniforms::MinifySamplerFilter::Nearest);

    let sun_rotation = Mat3::from_angle_x(Rad(time));
    let sun_direction = Vec3::new(0.0, 1.0, 0.0);
    let adjusted_sun_direction = sun_rotation * sun_direction;
    let adjusted_sun_direction_raw = down_size_v3(adjusted_sun_direction.into());

    let uniforms = uniform! {
        matrix: mvp_raw_downsized,
        u_texture_array: nearest_neighbour_texture,
        u_color: color,
        u_alpha_minimum: 0.05_f32,
        u_sun_direction: adjusted_sun_direction_raw,
    };

    let mut target = display.draw();

    let (width, height) = target.get_dimensions();

    let sky_blue = rgb(132, 193, 255);

    target.clear_color_and_depth(sky_blue.float_tup(), 1.0);
    target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &opaque_draw_params()).unwrap();

    let interface_raw : [[f64; 4]; 4] = interface(width, height).into();
    let interface_raw_downsized = down_size_m4(interface_raw);

    let interface_uniforms = uniform! {
        matrix: interface_raw_downsized,
        u_texture_array: nearest_neighbour_texture,
        u_color: color,
        u_alpha_minimum: 0.05_f32,
        u_sun_direction: adjusted_sun_direction_raw,
    };

    let mut tesselator = GeometryTesselator::new(tesselator_scale);

    let ui_z = 90.0;

    let tile_pixel_scale = 6.0_f64; 

    for (i, tile_id) in game_state.tile_queue.iter().enumerate() {
        let texture_region = &rs.tile_renderers[tile_id.0 as usize];
        
        let x = 10.0_f64;
        let y = 10.0 + (tile_pixel_scale * texture_region.height() as f64 + 10.0) * (i as f64);

        tesselator.color = [0.80, 0.80, 0.80, 1.0];

        tesselator.draw_ui(&texture_region, 0, x, y, ui_z, false, tile_pixel_scale);
        tesselator.draw_ui(&texture_region, 1, x, y, ui_z + 0.1, false, tile_pixel_scale);
        tesselator.draw_ui(&texture_region, 2, x, y, ui_z + 0.2, false, tile_pixel_scale);
    }
    // draw ui
    tesselator.draw_ui(&ok_indicator, 0, 10.0, 10.0, ui_z + 0.5, false, tile_pixel_scale);

    let vertex_buffer = glium::VertexBuffer::persistent(display,&tesselator.tesselator.vertices).unwrap();
    target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &interface_uniforms, &opaque_draw_params()).unwrap();
    target.finish().unwrap();
}