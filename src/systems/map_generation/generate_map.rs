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
// use bevy_mod_picking::prelude::*;
use rand::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::components::map_generation::tile_bundle::TileBundle;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::algorithm::run_algorithm;
use crate::utils::coord_conversions::hex_pos_to_vec3;

// TODO: move event listener run() into function.
// TODO: make scene top level entity. push tilebundle onto it.
pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_par: Res<MapParameters>,
) {
    // Generate map scaffolding.
    let mut scaffolding = run_algorithm(map_par);

    // Iterate through all of the scaffolding.
    for vec in scaffolding.iter_mut() {
        for scaffold in vec.iter_mut() {
            // Convert from cube coordinates to cartesian coordinates:
            let (x, y, z) = hex_pos_to_vec3(scaffold.pos.q, scaffold.pos.r, scaffold.pos.s);

            // Collapse the scaffold's wave function.
            let mut rng: ThreadRng = thread_rng();
            let choice: String = scaffold
                .wave_func
                .domain
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap()
                .0
                .clone();

            // Initialize the model.
            let mut model: SceneBundle = SceneBundle {
                scene: asset_server.load(choice),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            };

            // Rotate the model by ninety degrees. Tiles are flat-side up by default.
            model.transform.rotate_y(FRAC_PI_2);

            // Spawn the tile.
            commands.spawn((
                // Tile name.
                Name::new(format!("Tile ({},{})", scaffold.pos.q, scaffold.pos.r)),
                // Tilebundle.
                TileBundle::new(scaffold.pos, model),
                // Event listener for on click.
                // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
            ));

            // Adjust weights for incompatible surrounding tiles.
        }
    }
}
