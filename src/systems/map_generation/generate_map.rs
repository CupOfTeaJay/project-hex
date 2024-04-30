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

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::resources::map_parameters::MapParameters;
use crate::resources::tile_socket_maps::TileSocketMaps;
use crate::systems::map_generation::generate_map_data::generate_map_data;

// TODO: break down into smaller functions.
/// Generates a map according to the following algorithm:
pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_par: Res<MapParameters>,
    sockets: Res<TileSocketMaps>,
) {
    // Generate map data represented as hash tables. We need this to update scaffolding on the fly.
    let (pos_neighbor_map, mut pos_scaffold_map) = generate_map_data(&map_par);

    // This algorithm shall run until all scaffolding have "collapsed" to some tile.
    let mut remaining_scaffolds: usize = pos_scaffold_map.keys().len();
    while remaining_scaffolds > 0 {
        // First: pick a random scaffold from our collection.
        let rand_index = thread_rng().gen_range(0..remaining_scaffolds);
        let (pos, scaffold) = pos_scaffold_map.get_index(rand_index).unwrap();

        // Second: collapse the selected scaffold's wave function.
        let choice: String = scaffold.wave_func.collapse();

        // Third: initialize the chosen model.
        let model: SceneBundle = SceneBundle {
            scene: asset_server.load(choice),
            transform: scaffold.transform,
            ..Default::default()
        };

        // Third: spawn the chosen tile at it's respective location.
        commands.spawn((
            Name::new(format!("Tile ({},{})", scaffold.pos.q, scaffold.pos.r)),
            TileBundle::new(scaffold.pos, model),
            // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
        ));

        // Fourth: Adjust neighboring scaffolds.

        // Fifth: remove the scaffold from those still waiting to collapse.
        pos_scaffold_map.swap_remove_index(rand_index);
        remaining_scaffolds -= 1;
    }
}