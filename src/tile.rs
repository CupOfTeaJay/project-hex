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
