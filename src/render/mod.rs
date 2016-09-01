extern crate glium;
extern crate cgmath;

pub mod render_state;

use cgmath::Rad;
use gm2::{Vec3, Mat3};
use gm2::render::*;
use gm2::render::texture::*;
use gm2::render::quads::*;
use glium::index;
use glium::{Surface};


// pub use backend::glutin_backend::GlutinFacade as Display;
// pub fn render<F>(display: &F, rs:&RenderState, time:f64, color: [f32; 4], intersection: &Option<Vec3>) where F : glium::backend::Facade {
pub fn render(display: &glium::Display, rs:&render_state::RenderState, time:f64, color: [f32; 4], intersection: &Option<Vec3>) {
    let tesselator_scale = Vec3::new(rs.base_units_per_pixel(), rs.base_units_per_pixel(), rs.base_units_per_pixel());

    let mut tesselator = GeometryTesselator::new(tesselator_scale);
    let wall_tile = TextureRegion::at(&rs.texture, 2, 0);
    let man = TextureRegion::at(&rs.texture, 1, 0);
    let man_shadow = TextureRegion::at(&rs.texture, 2, 0);
    let indicator = TextureRegion::at(&rs.texture, 0, 0);

    for x in 0..16 {
        for y in 0..16 {
        // tesselator.color = [(x as f32) * 1.0 / 16.0, (z as f32) * 1.0 / 16.0, 1.0, 1.0];
            tesselator.draw_wall_tile(&wall_tile, 0, x as f64, y as f64, 0.0, 0.0, false);
            tesselator.draw_wall_tile(&wall_tile, 1, x as f64, y as f64, 0.0, 0.1, false);
            tesselator.draw_wall_tile(&wall_tile, 2, x as f64, y as f64, 0.0, 0.2, false);    
        }
    }

    // tesselator.draw_wall_base_anchored_at(&man, 0, Vec3::new(1.5, 0.0, 1.5), 0.0, false);
    // tesselator.draw_floor_centre_anchored_at(&man_shadow, 0, Vec3::new(1.5, 0.0, 1.5), 0.01, false);
    if let &Some(its) = intersection {
        let x = round_down(its.x);
        let y = round_down(its.y);
        tesselator.draw_wall_tile(&indicator, 0, x as f64, y as f64, 1.0 / 16.0 * 2.0, 0.55, false);
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
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &translucent_draw_params()).unwrap();
    target.finish().unwrap();
}