extern crate cgmath;

use super::{InnerBlockLocation, INNER_BLOCK_RESOLUTION, TileId};
use cgmath::vec2;
use gm2::*;
use std::collections::HashMap;



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tile {
    pub name: String, 
    pub id: TileId,
    pub nodes: Vec<InnerBlockLocation>, // connected
    pub preferred_idle: Option<(InnerBlockLocation, InnerBlockLocation)>,
}

pub struct Tiles {
    pub all: Vec<Tile>,
    pub safe: Vec<Tile>,
    pub by_name: HashMap<String, Tile>,
    pub default: Tile,
}

impl Tiles {
    pub fn with_id(&self,id:TileId) -> &Tile {
        &self.all[id as usize]
    } 

    pub fn is_safe(&self, id: TileId) -> bool {
        self.safe.iter().any(|t| t.id == id)
    }

    pub fn with_name(&self, name: &str) -> Option<&Tile> {
        self.by_name.get(name)
    }
}

pub fn produce_tile_set() -> Tiles {
    let half_resolution = INNER_BLOCK_RESOLUTION / 2;

    let base_height = 3;

    let middle_idle = Some((vec2(2, base_height), vec2(6, base_height)));
    let up_right_idle = Some((vec2(2, 2), vec2(6, 6)));
    let down_right_idle = Some((vec2(2, 6), vec2(6, 2)));

    // common node locations
    let bottom_left = vec2(0, base_height);
    let bottom_middle = vec2(half_resolution, base_height);
    let bottom_right = vec2(INNER_BLOCK_RESOLUTION, base_height);

    let top_left = vec2(0, INNER_BLOCK_RESOLUTION + base_height);
    let top_middle = vec2(half_resolution, INNER_BLOCK_RESOLUTION + base_height);
    let top_right = vec2(INNER_BLOCK_RESOLUTION, INNER_BLOCK_RESOLUTION + base_height);

    let all_bottom = vec![bottom_left, bottom_middle, bottom_right];
    let all_top = vec![top_left, top_middle, top_right];
    // let mut all = all_bottom.clone();
    // all.extend(all_top.iter().cloned());

    let mut available_id : u8 = 0;
    let mut next_id = || -> u8 {
        let v = available_id;
        available_id += 1;
        v
    };



    let empty = Tile { name: st("empty"), id: next_id(), nodes: Vec::new(), preferred_idle: None };
    let starting_steps = Tile { name: st("starting_steps"), id: next_id(), nodes: all_bottom.clone(), preferred_idle: middle_idle };
    let flat = Tile { name: st("flat"), id: next_id(), nodes: all_bottom.clone(), preferred_idle: middle_idle };
    let ramp_up_right = Tile { name: st("ramp_up_right"), id: next_id(), nodes: vec![bottom_left, top_right], preferred_idle: up_right_idle};
    let ramp_down_right = Tile { name: st("ramp_down_right"), id: next_id(), nodes: vec![top_left, bottom_right], preferred_idle: down_right_idle };
    let climb = Tile { name: st("climb"), id: next_id(), nodes: vec![bottom_left, bottom_right, bottom_middle, top_middle], preferred_idle: middle_idle};
    let tents = Tile { name: st("tents"), id: next_id(), nodes: all_bottom.clone(), preferred_idle: middle_idle};
    let trees = Tile { name: st("trees"), id: next_id(), nodes: all_bottom.clone(), preferred_idle: middle_idle};
    let stone_head = Tile { name: st("stone_head"), id: next_id(), nodes: all_bottom.clone(), preferred_idle: middle_idle};

    // no stone head or starting_steps
    let safe : Vec<Tile> = vec![empty.clone(), flat.clone(), ramp_up_right.clone(), ramp_down_right.clone(), climb.clone(), tents.clone(), trees.clone()];

    let all : Vec<Tile> = vec![
        empty.clone(), 
        starting_steps, 
        flat, 
        ramp_up_right, 
        ramp_down_right,
        climb,
        tents,
        trees,
        stone_head,
    ];

    let by_name : HashMap<String, Tile> = all.iter().map(|tile| (tile.name.clone(), tile.clone())).collect();

    Tiles {
        all: all,
        safe: safe,
        by_name: by_name,
        default: empty,
    }
}