use bevy::prelude::*;

use crate::components::common::hex_pos::HexPos;

#[derive(Component)]
pub struct MovementQueue {
    pub queue: Vec<HexPos>,
}

impl MovementQueue {
    pub fn new() -> Self {
        MovementQueue { queue: Vec::new() }
    }
}
