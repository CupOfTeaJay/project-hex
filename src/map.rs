/*
 * map.rs
 */

use bevy::prelude::*;

use crate::hex_grid::{HexFile, HexRank, HexGrid};
use crate::tile::TileBundle;

const HEX_FACTOR: f32 = 0.75;
const TILE_Y_POS: f32 = 0.0;

pub struct MapPlugin;

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}

fn determine_z_pos_start(
    total_ranks: u8,
    iteration: u8
) -> f32 {
    let height_unit: f32 = HEX_FACTOR.sqrt();
    let mut z_pos: f32 = 0.0;
    if iteration % 2 == 0 {
        z_pos = -height_unit*((total_ranks - 1) as f32);
    }
    else {
        z_pos = -height_unit*((total_ranks - 2) as f32);
    }
    return z_pos;
}

fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    hex_grid: Res<HexGrid>
) {
    let ranks: u8 = hex_grid.ranks;
    let files: u8 = hex_grid.files;
    let mut x_pos: f32 = -HEX_FACTOR*(files as f32) + HEX_FACTOR;
    for i in 0..files {
        let mut z_pos: f32 = determine_z_pos_start(files, i);
        for j in 0..ranks {
            commands.spawn(
                TileBundle {
                    file: HexFile::new(i),
                    rank: HexRank::new(j),
                    model: SceneBundle {
                        scene: asset_server.load("tiles/greenTile.glb#Scene0"),
                        transform: Transform::from_xyz(x_pos, TILE_Y_POS, z_pos),
                        ..Default::default()
                    }
                }
            );
            z_pos += HEX_FACTOR*2.0;
        }
        x_pos += HEX_FACTOR*2.0;
    }
}

