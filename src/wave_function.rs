/*
 * wave_function.rs
 */

use bevy::prelude::*;

use tile::TileTerrainType;

#[derive(Component)]
pub struct WaveFunction {
    val: HashMap<TileTerrainType, f32>
}

pub struct WaveFunctionPlugin;

impl Plugin for HexGridPlugin {
        fn build(&self, app: &mut App) {
        app.insert_resource(
            HexGrid::new(MAP_WIDTH, MAP_HEIGHT)
        );
    }
}