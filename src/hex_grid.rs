/*
 * hex_grid.rs
 * 
 * Responsible for spawning cells of the hex grid.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct HexPosition {
    pub rank: u8,
    pub file: u8
}

impl HexPosition {
    pub fn new(rank: u8, file: u8) -> Self {
        HexPosition {
            rank,
            file
        }
    }
}

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
        fn build(&self, app: &mut App) {
    }
}
