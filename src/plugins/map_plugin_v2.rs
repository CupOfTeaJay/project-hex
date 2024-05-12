/*
    Such is Life
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
use rand::thread_rng;
use rand::Rng;

use crate::resources::map_parameters::{
    ElevationParameters, MapParameters, NoiseRequest, NoiseType,
};
use crate::systems::map_generation_v2::generate_map_data::generate_map_data;
use crate::systems::map_generation_v2::spawn_map::spawn_map;

pub struct MapPluginV2;

impl Plugin for MapPluginV2 {
    fn build(&self, app: &mut App) {
        let elevation_parameters = ElevationParameters::new(
            vec![
                NoiseRequest::new(NoiseType::Simplex, 1, 1.0, 1.0, 1.0),
                // NoiseRequest::new(NoiseType::Worley, 1, 0.01, 0.0, 1.0),
            ],
            0.35,
            0.50,
            0.75,
        );
        let map_parameters = MapParameters::new(106, 66, 32013, elevation_parameters);

        // Insert resources into the app.
        app.insert_resource(map_parameters);
        // Add startup scheduled systems to the app.
        app.add_systems(Startup, spawn_map);
    }
}
