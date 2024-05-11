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

use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};

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

use super::common::Elevation;

pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_par: Res<MapParameters>,
) {
    // STEP 1:
    //     At a very minimum, we need to generate a hash table that maps a position to all of its
    //     neighbors. The neighbor relation is needed for certain algorithms, such as Wave Function
    //     Collapse.
    let pos_neighbors_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> =
        init_barebones(&map_par);

    // STEP 2:
    //     Now we need to establish a psuedo-random heightmap to determine which positions will
    //     become ocean, coastal, or land tiles. This is done using an amalgam of noise generators.
    let pos_elevation_map: IndexMap<(i32, i32, i32), Elevation> =
        generate_noise(&map_par, &pos_neighbors_map);

    let (mut x, mut y, mut z): (f32, f32, f32);
    for (pos, elevation) in pos_elevation_map.iter() {
        // Setup.
        (x, y, z) = cube_to_cartesian(pos.0 as f32, pos.1 as f32, pos.2 as f32);
        let mut transform = Transform::from_xyz(x, y, z);
        transform.rotate_y(FRAC_PI_2);
        if *elevation == Elevation::Land {
            // Grassland
            let model: SceneBundle = SceneBundle {
                scene: asset_server.load("tiles/grasslandTile.glb#Scene0".to_string()),
                transform: transform,
                ..Default::default()
            };
            commands.spawn((
                Name::new(format!("Tile ({},{})", pos.0, pos.1)),
                TileBundle::new(HexPos::new(pos.0 as f32, pos.1 as f32, pos.2 as f32), model),
                // On::<Pointer<Click>>::send_event::<SelectionEvent>(),
            ));
        } else if *elevation == Elevation::Coastal {
            // Ocean
            let model: SceneBundle = SceneBundle {
                scene: asset_server.load("tiles/coastalTile.glb#Scene0".to_string()),
                transform: transform,
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
                transform: transform,
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
