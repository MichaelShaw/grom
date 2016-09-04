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
}

pub struct Tiles {
    pub all: Vec<Tile>,
    pub safe: Vec<Tile>,
    pub by_name: HashMap<String, Tile>,
    pub default: Tile,
}

impl Tiles {
    pub fn with_name(&self, name: &str) -> Option<&Tile> {
        self.by_name.get(name)
    }
}

pub fn produce_tile_set() -> Tiles {
    let half_resolution = INNER_BLOCK_RESOLUTION / 2;

    // common node locations
    let bottom_left = vec2(0, 0);
    let bottom_middle = vec2(half_resolution, 0);
    let bottom_right = vec2(INNER_BLOCK_RESOLUTION, 0);
    let top_left = vec2(0, INNER_BLOCK_RESOLUTION);
    let top_middle = vec2(half_resolution, INNER_BLOCK_RESOLUTION);
    let top_right = vec2(INNER_BLOCK_RESOLUTION, INNER_BLOCK_RESOLUTION);

    let all_bottom = vec![bottom_left, bottom_middle, bottom_right];

    let mut available_id : u8 = 0;
    let mut next_id = || -> u8 {
        let v = available_id;
        available_id += 1;
        v
    };

    let empty = Tile { name: st("empty"), id: next_id(), nodes: Vec::new() };
    let starting_steps = Tile { name: st("starting_steps"), id: next_id(), nodes: all_bottom.clone() };
    let flat = Tile { name: st("flat"), id: next_id(), nodes: all_bottom.clone() };
    let ramp_up_right = Tile { name: st("ramp_up_right"), id: next_id(), nodes: vec![bottom_left, top_right]};
    let ramp_down_right = Tile { name: st("ramp_down_right"), id: next_id(), nodes: vec![top_left, bottom_right]};
    let climb = Tile { name: st("climb"), id: next_id(), nodes: vec![top_left, bottom_right, top_middle]};
    let tents = Tile { name: st("tents"), id: next_id(), nodes: all_bottom.clone()};
    let trees = Tile { name: st("trees"), id: next_id(), nodes: all_bottom.clone()};
    let stone_head = Tile { name: st("stone_head"), id: next_id(), nodes: all_bottom.clone()};

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