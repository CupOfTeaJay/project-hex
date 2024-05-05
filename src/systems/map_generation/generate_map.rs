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
use rand::{thread_rng, Rng};
use std::collections::HashMap;

use crate::components::map_generation::tile_bundle::TileBundle;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::common::Terrain;
use crate::systems::map_generation::generate_map_data::generate_map_data;
use crate::systems::map_generation::generate_map_data::Scaffold;

fn prevent_incompatibilities(
    choice: &String,
    pos: &(i32, i32, i32),
    pos_neighbor_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    pos_scaffold_map: &mut IndexMap<(i32, i32, i32), Scaffold>,
    tile_to_incompatible_map: &HashMap<String, Vec<String>>,
) {
    // Get neighbors to this position and what they can no longer collapse to (incompatabilities).
    let neighbor_positions = pos_neighbor_map.get(pos).unwrap();
    let incompats = tile_to_incompatible_map.get(choice).unwrap();

    // Adjust each of the neighbors, one by one - but only if the position still exists.
    for pos in neighbor_positions.iter() {
        if let Some(scaffold) = pos_scaffold_map.get_mut(pos) {
            // Loop through every incompatability.
            for incompatible in incompats.iter() {
                // Remove the incompatability and divvy its weight.
                let mut divvy: f32 = 0.0;
                if let Some(weight) = scaffold.purge_tile(incompatible) {
                    divvy += weight / scaffold.wave_func.domain.len() as f32;
                }

                // Iterate over the domain, sharing the divvy with remaining possibiliies.
                for weight in scaffold.wave_func.domain.values_mut() {
                    *weight += divvy;
                }
            }
        }
    }
}

fn bias_neighbors() {

}

fn determine_min_entropy_index(pos_scaffold_map: &IndexMap<(i32, i32, i32), Scaffold>) -> usize {
    // Get an arbitrary entry from the map so we can initialize min_ent and min_key with valid values.
    if let Some(arbitrary) = pos_scaffold_map.get_index(0) {
        // Vars to update.
        let mut min_key: &(i32, i32, i32) = arbitrary.0;
        let mut min_ent: usize = arbitrary.1.entropy;
        let mut curr_ent: usize;

        // Find scaffold with the lowest entropy.
        for pos in pos_scaffold_map.keys() {
            curr_ent = pos_scaffold_map[pos].entropy;
            if min_ent > curr_ent {
                min_ent = curr_ent;
                min_key = pos;
            }
        }

        // Return index of scaffold with the lowest entropy.
        if let Some(success) = pos_scaffold_map.get_index_of(min_key) {
            success
        } else {
            panic!("\nError: failed to find index of scaffold with lowest entropy\n")
        }
    } else {
        panic!("\nError: failed to execute determine_min_entropy_index\n")
    }
}

// TODO: break down into smaller functions.
/// Generates a map according to the following algorithm:
pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_par: Res<MapParameters>,
) {
    // Generate map data represented as hash tables. We need this to update scaffolding on the fly.
    let (pos_neighbor_map, mut pos_scaffold_map) = generate_map_data(&map_par);

    // Map that defines tile incompatabilities.
    let tile_to_incompatible_map = init_tile_to_incompatible_map();

    // This algorithm shall run until all scaffolding have "collapsed" to some tile.
    let init_scaffold_amt: usize = pos_scaffold_map.keys().len();
    let mut remaining_scaffolds: usize = init_scaffold_amt;
    while remaining_scaffolds > 0 {
        // If this is the first iteration, pick a random scaffold from our collection.
        // Otherwise select the scaffold with the lowest entropy.
        let index: usize;
        if remaining_scaffolds == init_scaffold_amt {
            index = thread_rng().gen_range(0..remaining_scaffolds);
        } else {
            index = determine_min_entropy_index(&pos_scaffold_map);
        }

        // Get the scaffold relative to our chosen index.
        let (pos, scaffold) = pos_scaffold_map.get_index(index).unwrap();
        let pos_clone = pos.clone();

        // Collapse this scaffold's wave function.
        let choice: String = scaffold.wave_func.collapse().clone();

        // Initialize the chosen tile's model.
        let model: SceneBundle = SceneBundle {
            scene: asset_server.load(choice.clone()),
            transform: scaffold.transform,
            ..Default::default()
        };

        // Spawn the chosen tile's model at its scaffold's position.
        commands.spawn((
            Name::new(format!("Tile ({},{})", scaffold.pos.q, scaffold.pos.r)),
            TileBundle::new(scaffold.pos, model),
            // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
        ));

        // Remove this scaffold from those still waiting to collapse.
        pos_scaffold_map.swap_remove_index(index);
        remaining_scaffolds -= 1;

        // Update the wave functions of neighboring scaffolds to prevent incompatabilities.
        prevent_incompatibilities(
            &choice,
            &pos_clone,
            &pos_neighbor_map,
            &mut pos_scaffold_map,
            &tile_to_incompatible_map,
        );
    }
}

fn init_tile_to_incompatible_map() -> HashMap<String, Vec<String>> {
    // Map to return.
    let mut incompatible = HashMap::new();

    // COASTAL.
    incompatible.insert(
        Terrain::Coastal.rep(),
        vec![
            // Any tile can be along a coast.
        ],
    );

    // DESERT.
    incompatible.insert(
        Terrain::Desert.rep(),
        vec![
            Terrain::Ice.rep(),
            Terrain::Ocean.rep(),
            Terrain::Snow.rep(),
            Terrain::Tundra.rep(),
        ],
    );

    // GRASSLAND.
    incompatible.insert(
        Terrain::Grassland.rep(),
        vec![
            Terrain::Ice.rep(),
            Terrain::Ocean.rep(),
            Terrain::Snow.rep(),
        ],
    );

    // ICE.
    incompatible.insert(
        Terrain::Ice.rep(),
        vec![
            Terrain::Desert.rep(),
            Terrain::Grassland.rep(),
            Terrain::Steppe.rep(),
            Terrain::Tundra.rep(),
        ],
    );

    // OCEAN.
    incompatible.insert(
        Terrain::Ocean.rep(),
        vec![
            Terrain::Desert.rep(),
            Terrain::Grassland.rep(),
            Terrain::Snow.rep(),
            Terrain::Steppe.rep(),
            Terrain::Tundra.rep(),
        ],
    );

    // SNOW.
    incompatible.insert(
        Terrain::Snow.rep(),
        vec![
            Terrain::Desert.rep(),
            Terrain::Grassland.rep(),
            Terrain::Ocean.rep(),
            Terrain::Steppe.rep(),
        ],
    );

    // STEPPE.
    incompatible.insert(
        Terrain::Steppe.rep(),
        vec![
            Terrain::Ice.rep(),
            Terrain::Ocean.rep(),
            Terrain::Snow.rep(),
        ],
    );

    // TUNDRA.
    incompatible.insert(
        Terrain::Tundra.rep(),
        vec![
            Terrain::Desert.rep(),
            Terrain::Ice.rep(),
            Terrain::Ocean.rep(),
        ],
    );

    // Return the map.
    incompatible
}
