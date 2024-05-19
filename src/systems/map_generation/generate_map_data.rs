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

use crate::components::map_generation::terrain::Terrain;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::apply_terr_convolution::apply_terr_convolution;
use crate::systems::map_generation::bias_terrwave_by_latitude::bias_terrwave_by_latitude;
use crate::systems::map_generation::common::Elevation;
use crate::systems::map_generation::common::WaveFunction;
use crate::systems::map_generation::init_pos_elevation_map::init_pos_elevation_map;
use crate::systems::map_generation::init_pos_neighbors_map::init_pos_neighbors_map;
use crate::systems::map_generation::init_pos_terr_map::init_pos_terr_map;
use crate::systems::map_generation::init_pos_terrwave_map::init_pos_terrwave_map;

/// Generates the layout of a psuedo-random, seeded map as determined by
/// MapParameters.
pub fn generate_map_data(map_par: &Res<MapParameters>) -> IndexMap<(i32, i32, i32), Terrain> {
    // STEP 1:
    //     At a very minimum, we need to generate a hash table that maps a position to all of its
    //     neighbors. This neighbor relation is needed for certain algorithms, such as Wave Function
    //     Collapse.
    let pos_neighbors_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> =
        init_pos_neighbors_map(&map_par);

    // STEP 2:
    //     We need to establish a psuedo-random heightmap to determine which positions will
    //     become ocean, coastal, land, and mountain tiles. This is done using an amalgam of noise
    //     generators.
    let pos_elevation_map: IndexMap<(i32, i32, i32), Elevation> =
        init_pos_elevation_map(&map_par, &pos_neighbors_map);

    // STEP 3:
    //     Now that we know what types of terrain a position should spawn based on its elevation,
    //     We can construct a position to wave function hash table in preperation for terrain WFC.
    let mut pos_terrwave_map: IndexMap<(i32, i32, i32), WaveFunction> =
        init_pos_terrwave_map(&map_par, &pos_elevation_map);

    // STEP 4:
    //     Some terrain types should only appear at a certain density - or at all - according to
    //     special latitudes. Adjust wave functions for this.
    bias_terrwave_by_latitude(&map_par, &mut pos_terrwave_map);

    // STEP 5:
    //     With the position to terrain wave-function map finalized, initialize a possition to
    //     terrain map using the wave-function collapse algorithm.
    let mut pos_terr_map: IndexMap<(i32, i32, i32), Terrain> =
        init_pos_terr_map(&map_par, &pos_neighbors_map, &mut pos_terrwave_map);

    // STEP 6:
    for _ in 0..map_par.convolution_parameters.terrain_convolutions {
        apply_terr_convolution(&pos_neighbors_map, &mut pos_terr_map);
    }

    // Stop here for now.
    pos_terr_map
}
