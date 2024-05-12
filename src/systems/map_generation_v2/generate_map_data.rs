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
use indexmap::IndexMap;

use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation_v2::common::Elevation;
use crate::systems::map_generation_v2::init_pos_elevation_map::init_pos_elevation_map;
use crate::systems::map_generation_v2::init_pos_neighbors_map::init_pos_neighbors_map;

/// Generates the layout of a psuedo-random, seeded map as determined by
/// MapParameters.
pub fn generate_map_data(map_par: &Res<MapParameters>) -> IndexMap<(i32, i32, i32), Elevation> {
    // STEP 1:
    //     At a very minimum, we need to generate a hash table that maps a position to all of its
    //     neighbors. This neighbor relation is needed for certain algorithms, such as Wave Function
    //     Collapse.
    let pos_neighbors_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> =
        init_pos_neighbors_map(&map_par);

    // STEP 2:
    //     Now we need to establish a psuedo-random heightmap to determine which positions will
    //     become ocean, coastal, land, and mountain tiles. This is done using an amalgam of noise generators.
    let pos_elevation_map: IndexMap<(i32, i32, i32), Elevation> =
        init_pos_elevation_map(&map_par, &pos_neighbors_map);

    // Stop here for now.
    pos_elevation_map
}
