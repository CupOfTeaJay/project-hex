/*
 * hex_grid.rs
 * 
 * Defines the HexGrid resource and HexFile / HexRank components. These components are necessary for
 * defining an entity's location on the hex grid.
 */

use bevy::prelude::*;

const MAP_WIDTH: u8 = 9;
const MAP_HEIGHT: u8 = 7;

// At the moment, for simplicity, the grid should have an odd number of files and ranks.
// The number of ranks should be 2 less than the number of files.
#[derive(Resource, Debug)]
pub struct HexGrid {
    pub files: u8,
    pub ranks: u8
}

impl HexGrid {
    pub fn new(files: u8, ranks: u8) -> Self {
        HexGrid {
            files,
            ranks
        }
    }
}

#[derive(Component, Debug)]
pub struct HexFile {
    pub val: u8
}

impl HexFile {
    pub fn new(val: u8) -> Self {
        HexFile {
            val
        }
    }
}

#[derive(Component, Debug)]
pub struct HexRank {
    pub val: u8
}

impl HexRank {
    pub fn new(val: u8) -> Self {
        HexRank {
            val
        }
    }
}

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
        fn build(&self, app: &mut App) {
        app.insert_resource(
            HexGrid::new(MAP_WIDTH, MAP_HEIGHT)
        );
    }
}
