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
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::common::Terrain;
use crate::systems::map_generation::common::WaveFunction;

/// Initializes terrain selections for every cube coordinate on the map, as determined by a
/// wave-function collapse algorithm.
pub fn init_pos_terr_map(
    map_par: &Res<MapParameters>,
    pos_neighbors_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    pos_terrwave_map: &mut IndexMap<(i32, i32, i32), WaveFunction>,
) -> IndexMap<(i32, i32, i32), Terrain> {
    // Initialize a new position to terrain hash table.
    let mut pos_terr_map: IndexMap<(i32, i32, i32), Terrain> = IndexMap::new();

    // Initialize the terrain incompatability hash table.
    let terr_incompat_map: IndexMap<Terrain, Vec<Terrain>> = init_terr_incompat_map();

    // Declare / init loop vars.
    let total_wave_functions: usize = pos_terrwave_map.len();
    let mut remaining_wave_functions: usize = total_wave_functions;
    let (mut pos, mut wave_func): (&(i32, i32, i32), &WaveFunction);
    let mut pos_clone: (i32, i32, i32);
    let mut min_entropy_index: usize;
    let mut choice: Terrain;

    // Perform wave-function collapse on the input pos_terrwave_map.
    while remaining_wave_functions > 0 {
        // If we have not collapsed any wave functions yet, select a random one on the map.
        if remaining_wave_functions == total_wave_functions {
            (pos, wave_func) = pos_terrwave_map
                .get_index(
                    StdRng::seed_from_u64(map_par.seed as u64).gen_range(0..total_wave_functions),
                )
                .unwrap()
        }
        // Otherwise get a wave function with the lowest detected entropy.
        else {
            min_entropy_index = determine_min_entropy_index(pos_terrwave_map);
            (pos, wave_func) = pos_terrwave_map.get_index(min_entropy_index).unwrap();
        }

        // Collapse this wave function. Offset the map seed each iteration by remaining_wave_functions.
        choice = wave_func
            .collapse(map_par.seed + remaining_wave_functions as u32)
            .clone();
        pos_clone = pos.clone();

        // Insert the choice into our return map.
        pos_terr_map.insert(pos_clone, choice);

        // Adjust neighboring wave functions based on this selection.
        adjust_neighbors(
            &choice,
            &map_par,
            &pos_clone,
            pos_neighbors_map,
            pos_terrwave_map,
            &terr_incompat_map,
        );

        // Remove this wave function from pos_terrwave_map and update remaining wave functions.
        pos_terrwave_map.swap_remove(&pos_clone);
        remaining_wave_functions -= 1;
    }

    // Return our position to terrain map.
    pos_terr_map
}

/// Adjusts neighboring wave functions based on the collapse of a select wave function.
fn adjust_neighbors(
    choice: &Terrain,
    map_par: &Res<MapParameters>,
    pos: &(i32, i32, i32),
    pos_neighbors_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    pos_terrwave_map: &mut IndexMap<(i32, i32, i32), WaveFunction>,
    terr_incompat_map: &IndexMap<Terrain, Vec<Terrain>>,
) {
    // Get neighbors of this position and what they can no longer collapse to (incompatabilities).
    let neighbors = pos_neighbors_map.get(pos).unwrap();
    let incompats = terr_incompat_map.get(choice).unwrap();

    // Adjust each of the neighbors, one by one - but only if the position still exists.
    for neighbor in neighbors.iter() {
        if let Some(wave_func) = pos_terrwave_map.get_mut(neighbor) {
            // Remove incompatabilities.
            for incompat in incompats.iter() {
                wave_func.purge(incompat);
            }

            // Bias remaining possibilities based on collapse.
            if let Some(weight) = wave_func.domain.get_mut(choice) {
                match choice {
                    &Terrain::Coastal => (),
                    &Terrain::Debug => (),
                    &Terrain::Desert => *weight *= map_par.terrain_spawn_parameters.desert_bias,
                    &Terrain::Grassland => {
                        *weight *= map_par.terrain_spawn_parameters.grassland_bias
                    }
                    &Terrain::Ice => *weight *= map_par.terrain_spawn_parameters.ice_bias,
                    &Terrain::Mountain => (),
                    &Terrain::Ocean => (),
                    &Terrain::Snow => *weight *= map_par.terrain_spawn_parameters.snow_bias,
                    &Terrain::Steppe => *weight *= map_par.terrain_spawn_parameters.steppe_bias,
                    &Terrain::Tundra => *weight *= map_par.terrain_spawn_parameters.tundra_bias,
                }
            }
        }
    }
}

/// Initializes a map that defines the incompatabilities of terrain types.
fn init_terr_incompat_map() -> IndexMap<Terrain, Vec<Terrain>> {
    // Incompatability table to return.
    let mut incompatible: IndexMap<Terrain, Vec<Terrain>> = IndexMap::new();

    // COASTAL.
    incompatible.insert(
        Terrain::Coastal,
        vec![
            // Any tile can be along a coast.
        ],
    );

    // DEBUG.
    incompatible.insert(
        Terrain::Debug,
        vec![
            // Debug should have no incompatabilities.
        ],
    );

    // DESERT.
    incompatible.insert(
        Terrain::Desert,
        vec![Terrain::Ice, Terrain::Ocean, Terrain::Snow, Terrain::Tundra],
    );

    // GRASSLAND.
    incompatible.insert(
        Terrain::Grassland,
        vec![Terrain::Ice, Terrain::Ocean, Terrain::Snow],
    );

    // ICE.
    incompatible.insert(
        Terrain::Ice,
        vec![
            Terrain::Desert,
            Terrain::Grassland,
            Terrain::Steppe,
            Terrain::Tundra,
        ],
    );

    // MOUNTAIN.
    incompatible.insert(
        Terrain::Mountain,
        vec![
            // Any tile can be next to a mountain.
        ],
    );

    // OCEAN.
    incompatible.insert(
        Terrain::Ocean,
        vec![
            Terrain::Desert,
            Terrain::Grassland,
            Terrain::Snow,
            Terrain::Steppe,
            Terrain::Tundra,
        ],
    );

    // SNOW.
    incompatible.insert(Terrain::Snow, vec![Terrain::Desert, Terrain::Ocean]);

    // STEPPE.
    incompatible.insert(
        Terrain::Steppe,
        vec![Terrain::Ice, Terrain::Ocean, Terrain::Snow],
    );

    // TUNDRA.
    incompatible.insert(
        Terrain::Tundra,
        vec![Terrain::Desert, Terrain::Ice, Terrain::Ocean],
    );

    // Return the incompatability map.
    incompatible
}

/// Determines the wave function with the lowest entropy.
fn determine_min_entropy_index(
    pos_terrwave_map: &IndexMap<(i32, i32, i32), WaveFunction>,
) -> usize {
    // Get an arbitrary entry from our pos_terrwave_map so we can initialize min_ent and min_key
    // with valid values.
    if let Some(arbitrary) = pos_terrwave_map.get_index(0) {
        // Init vars to update.
        let mut min_key: &(i32, i32, i32) = arbitrary.0;
        let mut min_ent: usize = arbitrary.1.entropy;
        let mut curr_ent: usize;

        // Find scaffold with the lowest entropy.
        for pos in pos_terrwave_map.keys() {
            curr_ent = pos_terrwave_map[pos].entropy;
            if min_ent > curr_ent {
                min_ent = curr_ent;
                min_key = pos;
            }
        }

        // Return index of scaffold with the lowest entropy.
        if let Some(success) = pos_terrwave_map.get_index_of(min_key) {
            success
        } else {
            panic!("\ndetermine_min_entropy_index Error: failed to find index of scaffold with lowest entropy.\n")
        }
    } else {
        panic!(
            "\ndetermine_min_entropy_index Error: failed to execute determine_min_entropy_index.\n"
        )
    }
}
