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

use crate::resources::map_parameters::{
    DimensionParameters, LatitudeParameters, LimitParameters, MapParameters, SpawnParameters,
};
use crate::systems::map_generation::make_tiles_pickable::make_tiles_pickable;
use crate::systems::map_generation_v2::generate_map::generate_map;

pub struct MapPluginV2;

// TODO: somehow move make_tiles_pickable out of the Update schedule.
impl Plugin for MapPluginV2 {
    fn build(&self, app: &mut App) {
        // Initialize map sub-parameters.
        let map_latitude_parameters = LatitudeParameters::new(
            0.0, 0.0, 0.2, 0.2, 0.2, 0.0, 0.4, 0.0, 0.5, 0.0, 0.0, 0.0, 0.2, 0.2, 0.0, 0.2,
        );
        let map_limit_parameters = LimitParameters::new(4.0, 3.0);
        let map_spawn_parameters = SpawnParameters::new(
            0.5, -0.1, -0.1, -0.1, 0.5, -0.1, -0.1, -0.1, 0.0, 0.8, 0.0, 0.0, 0.0, 0.0, 0.8, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, -0.2, 0.0, 0.50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.8,
        );
        let map_dimensions = DimensionParameters::new(106, 66);

        // Initialize map parameters.
        let map_parameters = MapParameters::new(
            map_dimensions,
            map_latitude_parameters,
            map_spawn_parameters,
            map_limit_parameters,
        );

        // Insert resources into the app.
        app.insert_resource(map_parameters);
        // Add startup scheduled systems to the app.
        app.add_systems(Startup, generate_map);

        // Add update scheduled systems to the app.
        app.add_systems(Update, make_tiles_pickable);
    }
}
