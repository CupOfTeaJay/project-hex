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
use crate::systems::map_generation::generate_map_data::generate_map_data;
use crate::systems::map_generation::generate_map_data::DOMAIN_SIZE;

use super::generate_map_data::Scaffold;

//TODO: might not even need deleted_positions with "if let".

fn adjust_neighbors(
    choice: &String,
    pos: &(i32, i32, i32),
    pos_neighbor_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    pos_scaffold_map: &mut IndexMap<(i32, i32, i32), Scaffold>,
    tile_to_incompatible_map: &HashMap<String, Vec<String>>,
    deleted_positions: &Vec<(i32, i32, i32)>,
) {
    // Get neighbors to this position and what they can no longer collapse to (incompatabilities).
    let neighbors = pos_neighbor_map.get(pos).unwrap();
    let incompats = tile_to_incompatible_map.get(choice).unwrap();

    // Adjust each of the neighbors, one by one - but only if the position still exists.
    for neighbor_pos in neighbors.iter() {
        if let Some(neighbor) = pos_scaffold_map.get_mut(neighbor_pos) {
            // Loop through every incompatability.
            for incompatible in incompats.iter() {
                // Find the incompatability, and divvy its weight.
                let mut divvy: f32 = 0.0;
                for (possibility, weight) in neighbor.wave_func.domain.iter_mut() {
                    if possibility == incompatible {
                        divvy += *weight / DOMAIN_SIZE as f32;
                        *weight = 0.0;
                    }
                }
                // Iterate over the domain once more, sharing the divvy with remaining possibiliies.
                for (possibility, weight) in neighbor.wave_func.domain.iter_mut() {
                    if possibility != incompatible {
                        *weight += divvy;
                    }
                }
            }
        }
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

    // We need to keep track of scaffolds at positions that no longer need to collapse.
    let mut deleted_positions: Vec<(i32, i32, i32)> = Vec::new();

    // This algorithm shall run until all scaffolding have "collapsed" to some tile.
    let mut remaining_scaffolds: usize = pos_scaffold_map.keys().len();
    while remaining_scaffolds > 0 {
        // First: pick a random scaffold from our collection.
        let rand_index = thread_rng().gen_range(0..remaining_scaffolds);
        let (pos, scaffold) = pos_scaffold_map.get_index_mut(rand_index).unwrap();
        let pos_clone = pos.clone();

        // Second: collapse the selected scaffold's wave function.
        let choice: String = scaffold.wave_func.collapse();

        // Third: initialize the chosen model.
        let model: SceneBundle = SceneBundle {
            scene: asset_server.load(choice.clone()),
            transform: scaffold.transform,
            ..Default::default()
        };

        // Third: spawn the chosen tile at it's respective location.
        commands.spawn((
            Name::new(format!("Tile ({},{})", scaffold.pos.q, scaffold.pos.r)),
            TileBundle::new(scaffold.pos, model),
            // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
        ));

        // Fourth: remove the scaffold from those still waiting to collapse.
        deleted_positions.push(pos_clone);
        pos_scaffold_map.swap_remove_index(rand_index);
        remaining_scaffolds -= 1;

        // Fifth: Adjust neighboring scaffolds.
        adjust_neighbors(
            &choice,
            &pos_clone,
            &pos_neighbor_map,
            &mut pos_scaffold_map,
            &tile_to_incompatible_map,
            &deleted_positions,
        );
    }
}

fn init_tile_to_incompatible_map() -> HashMap<String, Vec<String>> {
    // Map to update.
    let mut incompatible = HashMap::new();

    incompatible.insert("tiles/coastalTile.glb#Scene0".to_string(), vec![]);

    // Tiles incompatible with desert terrain.
    incompatible.insert(
        "tiles/desertTile.glb#Scene0".to_string(),
        vec![
            "tiles/iceTile.glb#Scene0".to_string(),
            "tiles/jungleTile.glb#Scene0".to_string(),
            "tiles/snowTile.glb#Scene0".to_string(),
            "tiles/tundraTile.glb#Scene0".to_string(),
        ],
    );

    incompatible.insert("tiles/grasslandTile.glb#Scene0".to_string(), vec![]);

    incompatible.insert(
        "tiles/iceTile.glb#Scene0".to_string(),
        vec!["tiles/desertTile.glb#Scene0".to_string()],
    );

    incompatible.insert(
        "tiles/jungleTile.glb#Scene0".to_string(),
        vec!["tiles/desertTile.glb#Scene0".to_string()],
    );

    incompatible.insert("tiles/oceanTile.glb#Scene0".to_string(), vec![]);

    incompatible.insert(
        "tiles/snowTile.glb#Scene0".to_string(),
        vec!["tiles/desertTile.glb#Scene0".to_string()],
    );

    incompatible.insert("tiles/steppeTile.glb#Scene0".to_string(), vec![]);

    incompatible.insert(
        "tiles/tundraTile.glb#Scene0".to_string(),
        vec!["tiles/desertTile.glb#Scene0".to_string()],
    );

    // Return the map.
    incompatible
}
