extern crate glium;
extern crate cgmath;
extern crate image;

use gm2::render::{texture, shader};
use gm2::render::texture::TextureRegion;
use gm2::camera;
use cgmath::Rad;
use gm2::Vec3;
use gm2::spring::*;
use std::collections::HashMap;
use game::*;

pub struct RenderState {
    pub program: glium::Program,
    pub texture: texture::TiledTexture,
    pub camera: camera::Camera,
    pub base_pixels_per_unit: f64,
    pub tile_renderers : Vec<texture::TextureRegion>, 
    pub climber_renderers: [[TextureRegion; 4] ; 4],
    pub cloud_renderers: [TextureRegion; 4],

    pub cloud_positions: Vec<Vec3>,

    pub zoom : SpringState1,
    pub camera_target : SpringState3,

    pub entity_springs : HashMap<ClimberId, ClimberRenderState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClimberRenderState {
    pub spring: SpringState3,
    pub walk_progress: f64,
} 

impl ClimberRenderState {
    pub fn new(position:Vec3) -> ClimberRenderState {
        ClimberRenderState {
            spring: SpringState3::new(position),
            walk_progress: 0.0,
        }
    } 
}

impl RenderState {
    pub fn pixels_per_unit(&self) -> f64 {
        self.base_pixels_per_unit * self.zoom.position 
    }

    pub fn base_units_per_pixel(&self) -> f64 {
        1.0 / self.base_pixels_per_unit
    }

    pub fn update_camera_from_springs(&mut self) {
        self.camera.pixels_per_unit = self.pixels_per_unit();
        self.camera.at = self.camera_target.position;
    }
}

pub fn init<F>(display: &F, tiles:&Tiles) -> RenderState where F : glium::backend::Facade {
    use std::path::{PathBuf, Path};
    use std::f64::consts::PI;
    use std::fs;

    let root_path = Path::new("img/tiles");

    let raw : Vec<PathBuf> = fs::read_dir(root_path).unwrap().filter_map(|entry| {
        let p = entry.unwrap().path();
        if p.extension().and_then(|ext| ext.to_str()) == Some("png") {
            Some(p)
        } else {
            None
        }
    }).collect();

    let paths : Vec<&Path> = raw.iter().map(|p| p.as_path()).collect();

    println!("texture paths for array is -> {:?}", paths);

    let tiled_texture = texture::load_tiled_texture(display, &paths, 32);
  
    let (width, height) = display.get_context().get_framebuffer_dimensions();

    let base_pixels_per_unit = 16.0_f64;
    let zoom = 10.0_f64;

    let camera_pixels_per_unit = base_pixels_per_unit * zoom;

    let tile_texture_regions : Vec<TextureRegion> = tiles.all.iter().map (|tile| {
        tiled_texture.at((tile.id + 2) as u32, 0) 
    }).collect();

    let climber_renderers = tiled_texture.at_d4(1, 0);
    let cloud_renderers = [tiled_texture.at(2, 3), tiled_texture.at(2, 4), tiled_texture.at(3, 3), tiled_texture.at(3, 4)];

    RenderState {
        program: shader::simple_program(display),
        texture: tiled_texture,
        camera: camera::Camera {
            at: Vec3::new(0.0, 0.0, 0.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: (width, height),
            pixels_per_unit: camera_pixels_per_unit,
        },
        base_pixels_per_unit: base_pixels_per_unit, // fixed for a game, really ...
        
        tile_renderers: tile_texture_regions,
        climber_renderers: climber_renderers,
        cloud_renderers: cloud_renderers,

        cloud_positions: Vec::new(),

        zoom: SpringState1::new(zoom),
        camera_target: SpringState3::new(Vec3::new(0.0, 0.0, 0.0)),

        entity_springs: HashMap::new(),
    }
}




