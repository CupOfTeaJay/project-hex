/*
 * tile.rs
 */

use bevy::prelude::*;

use crate::hex_grid::HexPosition;

#[derive(Bundle)]
pub struct TileBundle {
    pub grid_pos: HexPosition,
    pub model: SceneBundle
}

#[repr(u8)]
#[derive(Eq, PartialEq, Hash)]
pub enum TileTerrainType {
    Desert    = 0,
    Grassland = 1,
    Ocean     = 2,
    Snow      = 3
}
