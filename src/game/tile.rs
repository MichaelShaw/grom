extern crate cgmath;

use super::{InnerBlockLocation, INNER_BLOCK_RESOLUTION, TileId};
use cgmath::vec2;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tile {
    pub id: TileId,
    pub nodes: Vec<InnerBlockLocation>, // connected
}

pub struct Tiles {
    pub all: Vec<Tile>,
    pub empty: Tile,
    pub starting_steps: Tile,
    pub flat: Tile,
    pub ramp_up_right: Tile,
    pub ramp_down_right: Tile,
    pub climb: Tile,
    pub tents: Tile,
    pub trees: Tile,
    pub stone_head: Tile,
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

    let empty = Tile { id: next_id(), nodes: Vec::new() };
    let starting_steps = Tile { id: next_id(), nodes: all_bottom.clone() };
    let flat = Tile { id: next_id(), nodes: all_bottom.clone() };
    let ramp_up_right = Tile { id: next_id(), nodes: vec![bottom_left, top_right]};
    let ramp_down_right = Tile { id: next_id(), nodes: vec![top_left, bottom_right]};
    let climb = Tile { id: next_id(), nodes: vec![top_left, bottom_right, top_middle]};
    let tents = Tile { id: next_id(), nodes: all_bottom.clone()};
    let trees = Tile { id: next_id(), nodes: all_bottom.clone()};
    let stone_head = Tile { id: next_id(), nodes: all_bottom.clone()};
        
    let all = vec![
        empty.clone(), 
        starting_steps.clone(), 
        flat.clone(), 
        ramp_up_right.clone(), 
        ramp_down_right.clone(),
        climb.clone(),
        tents.clone(),
        trees.clone(),
        stone_head.clone(),
    ];

    Tiles {
        all: all,
        empty: empty,
        starting_steps: starting_steps,
        flat: flat,
        ramp_up_right:ramp_up_right, 
        ramp_down_right:ramp_down_right, 
        climb:climb,
        tents:tents,
        trees:trees,
        stone_head:stone_head,
    }
}