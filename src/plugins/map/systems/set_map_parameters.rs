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

use crate::common::resources::map_parameters::{MapParameters, NoiseRequest, NoiseType};

pub fn set_map_parameters(mut map_par: ResMut<MapParameters>) {
    // TODO
    map_par.width = 106;
    map_par.height = 66;
    map_par.seed = 6969420;

    // TODO
    map_par.elevation_parameters.noise_requests = vec![
        // NoiseRequest::new(NoiseType::Simplex, 10, 1.0, 1.0, 1.0),
        NoiseRequest::new(NoiseType::Worley, 20, 1.0, 1.0, 0.5),
    ];
    map_par.elevation_parameters.ocean_threshold = 0.35;
    map_par.elevation_parameters.coastal_threshold = 0.50;
    map_par.elevation_parameters.land_threshold = 0.85;

    // TODO
    map_par.latitude_parameters.sigma = 15.0;
    map_par.latitude_parameters.temperature = 50.0;

    // TODO
    map_par.terrain_spawn_parameters.desert_bias = 2.0;
    map_par.terrain_spawn_parameters.grassland_bias = 2.0;
    map_par.terrain_spawn_parameters.ice_bias = 2.0;
    map_par.terrain_spawn_parameters.snow_bias = 2.0;
    map_par.terrain_spawn_parameters.steppe_bias = 2.0;
    map_par.terrain_spawn_parameters.tundra_bias = 2.0;

    // TODO
    map_par.convolution_parameters.terrain_convolutions = 12;
}
