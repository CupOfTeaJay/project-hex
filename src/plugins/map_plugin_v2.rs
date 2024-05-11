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
use rand::{thread_rng, Rng};

use crate::resources::map_parameters::{MapParameters, NoiseRequest, NoiseType};
use crate::systems::map_generation_v2::generate_map::generate_map;

pub struct MapPluginV2;

impl Plugin for MapPluginV2 {
    fn build(&self, app: &mut App) {
        let noise_parameters = vec![NoiseRequest::new(NoiseType::Simplex, 4, 3.0, 1.0, 1.893)];
        let map_parameters = MapParameters::new(106, 66, thread_rng().gen(), noise_parameters);

        // Insert resources into the app.
        app.insert_resource(map_parameters);
        // Add startup scheduled systems to the app.
        app.add_systems(Startup, generate_map);
    }
}
