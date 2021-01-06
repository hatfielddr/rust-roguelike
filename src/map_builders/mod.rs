use specs::prelude::*;

use common::*;
use simple_map::SimpleMapBuilder;

use super::{Map, Position, Rect, spawner, TileType};

mod simple_map;
mod common;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    // Note that until we have a second map type, this isn't even slighlty random
    Box::new(SimpleMapBuilder::new(new_depth))
}

