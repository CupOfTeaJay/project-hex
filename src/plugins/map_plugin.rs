/*
    Project Hex
    Copyright (C) 2024 Clevermeld™ LLC

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use bevy::prelude::*;

#[rustfmt::skip]
use crate::resources::{
    map_parameters::ConvolutionParameters,
    map_parameters::ElevationParameters,
    map_parameters::LatitudeParameters,
    map_parameters::MapParameters,
    map_parameters::NoiseRequest,
    map_parameters::NoiseType,
    map_parameters::TerrainSpawnParameters,
};

#[rustfmt::skip]
use crate::states::{
    app_state::AppState,
    assets_state::AssetsState,
    boot_state::BootState,
    game_state::GameState,
};

use crate::systems::map_generation::spawn_map::spawn_map;

/// Plugin that encapsulates algorithms related to map generation. Currently, the MapPlugin:
///     - Spawns a map from a given seed.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let elevation_parameters = ElevationParameters::new(
            vec![
                // NoiseRequest::new(NoiseType::Simplex, 10, 1.0, 1.0, 1.0),
                NoiseRequest::new(NoiseType::Worley, 20, 1.0, 1.0, 0.5),
            ],
            0.35,
            0.50,
            0.85,
        );
        let latitude_parameters = LatitudeParameters::new(15.0, 50.0);
        let terrain_spawn_parameters = TerrainSpawnParameters::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        let convolution_parameters = ConvolutionParameters::new(11);
        let map_parameters = MapParameters::new(
            106,
            66,
            6969420,
            elevation_parameters,
            latitude_parameters,
            terrain_spawn_parameters,
            convolution_parameters,
        );

        // Register resources with the main application.
        app.insert_resource(map_parameters);
        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            spawn_map
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(in_state(GameState::MapGen)),
        );
    }
}
