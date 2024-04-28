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
use bevy_mod_picking::prelude::*;
use rand::prelude::*;

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::components::map_generation::wave_function::WaveFunction;
use crate::events::user_interaction::selection_event::SelectionEvent;
use crate::resources::tile_socket_maps::TileSocketMaps;

pub fn wave_func_collapse_copy(world: &mut World) {
    // Select a random scaffold on the map.

    // Collapse that scaffold's wave function.

    // update every neighboring scaffold's wave-function due to the collapse.

    // Despawn the selected scaffold.
}

// TODO: move event listener run() into function.
// TODO: make scene top level entity. push tilebundle onto it.
pub fn spawn_tile(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(&HexPos, &Transform, &WaveFunction)>,
    sockets: Res<TileSocketMaps>,
) {
    for (pos, transform, wave_func) in &query {
        // Select tile terrain.
        let mut rng: ThreadRng = thread_rng();
        let choice: String = wave_func
            .domain
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
            .clone();

        // Initialize the model.
        let model: SceneBundle = SceneBundle {
            scene: asset_server.load(choice),
            transform: *transform,
            ..Default::default()
        };

        // Spawn the tile.
        commands.spawn((
            // Tile name.
            Name::new(format!("Tile ({},{})", pos.q, pos.r)),
            // Tilebundle.
            TileBundle::new(*pos, model),
            // Event listener for on click.
            On::<Pointer<Click>>::send_event::<SelectionEvent>(),
        ));

        // Adjust weights for incompatible surrounding tiles.
    }
}
