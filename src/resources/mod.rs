use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct OccupiedPositions {
    positions: HashSet<(u32, u32)>,
}

impl OccupiedPositions {
    pub fn is_occupied(&self, x: u32, y: u32) -> bool {
        self.positions.contains(&(x, y))
    }

    pub fn occupy(&mut self, x: u32, y: u32) -> bool {
        self.positions.insert((x, y))
    }

    pub fn free(&mut self, x: u32, y: u32) {
        self.positions.remove(&(x, y));
    }
}