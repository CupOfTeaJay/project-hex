/*
 * wave_function.rs
 */

use bevy::prelude::*;
use std::collections::HashMap;

use crate::tile::TileTerrainType;

#[derive(Component)]
pub struct WaveFunction {
    map: HashMap<TileTerrainType, f32>
}

impl WaveFunction {
    pub fn new() -> Self {
        let mut map: HashMap<TileTerrainType, f32> = HashMap::new();
        map.insert(TileTerrainType::Desert, 0.25);
        map.insert(TileTerrainType::Grassland, 0.25);
        map.insert(TileTerrainType::Ocean, 0.25);
        map.insert(TileTerrainType::Snow, 0.25);
        WaveFunction {
            map
        }
    }
}
