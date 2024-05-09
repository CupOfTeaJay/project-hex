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
use rand::{thread_rng, Rng, SeedableRng};

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation_v2::generate_noise::generate_noise;
use crate::systems::map_generation_v2::init_barebones::init_barebones;
use crate::utils::coord_conversions::cube_to_cartesian;

const THRESHOLD: f64 = 0.1;

pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_par: Res<MapParameters>,
) {
    // Generate a random number according to the map seed.
    let rand_from_seed: u32 = thread_rng().gen();

    // At a minimum, we need a hash table that relates any given position to its neighbors.
    let pos_neighbors_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> =
        init_barebones(&map_par);

    let pos_noise_map: IndexMap<(i32, i32, i32), f64> =
        generate_noise(&pos_neighbors_map, &rand_from_seed);

    let (mut x, mut y, mut z): (f32, f32, f32);
    for (pos, noise) in pos_noise_map.iter() {
        (x, y, z) = cube_to_cartesian(pos.0 as f32, pos.1 as f32, pos.2 as f32);
        if *noise > THRESHOLD {
            // Grassland
            let model: SceneBundle = SceneBundle {
                scene: asset_server.load("tiles/grasslandTile.glb#Scene0".to_string()),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            };
            commands.spawn((
                Name::new(format!("Tile ({},{})", pos.0, pos.1)),
                TileBundle::new(HexPos::new(pos.0 as f32, pos.1 as f32, pos.2 as f32), model),
                // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
            ));
        } else {
            // Ocean
            let model: SceneBundle = SceneBundle {
                scene: asset_server.load("tiles/oceanTile.glb#Scene0".to_string()),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            };
            commands.spawn((
                Name::new(format!("Tile ({},{})", pos.0, pos.1)),
                TileBundle::new(HexPos::new(pos.0 as f32, pos.1 as f32, pos.2 as f32), model),
                // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
            ));
        }
    }
}
