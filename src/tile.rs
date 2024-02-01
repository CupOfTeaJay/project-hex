/*
 * tile.rs
 */

use bevy::prelude::*;

use crate::hex_grid::HexFile;
use crate::hex_grid::HexRank;

#[derive(Bundle)]
pub struct TileBundle {
    pub file: HexFile,
    pub rank: HexRank,
    pub model: SceneBundle
}

pub struct TilePlugin;

impl Plugin for TilePlugin{
        fn build(&self, app: &mut App) {
    }
}

#[repr(u8)]
pub enum TileTerrainType {
    Desert    = 0,
    Grassland = 1,
    Ocean     = 2,
    Snow      = 3
}
