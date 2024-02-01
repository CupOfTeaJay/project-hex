/*
 * hex_grid.rs
 * 
 * Responsible for spawning cells of the hex grid.
 */

use bevy::prelude::*;

use crate::wave_function::WaveFunction;

#[derive(Bundle)]
pub struct HexBundle {
    pub transform: Transform,
    pub grid_pos: HexPosition,
    pub wave_function: WaveFunction
}

impl HexBundle {
    pub fn new(rank: u8, file: u8, pos: Vec3) -> Self {
        HexBundle {
            transform: Transform {
                translation: pos,
                ..Default::default()
            },
            grid_pos: HexPosition::new(rank, file),
            wave_function: WaveFunction::new()
        }
    }
}

#[derive(Clone, Component, Copy)]
pub struct HexPosition {
    pub file: u8,
    pub rank: u8
}

impl HexPosition {
    pub fn new(file: u8, rank: u8) -> Self {
        HexPosition {
            file,
            rank
        }
    }
}
