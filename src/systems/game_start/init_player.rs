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

// Find a random land tile on the map. Spawn a unit there.

use bevy::prelude::*;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};

use crate::{
    components::{common::hex_pos::HexPos, map_generation::terrain::Terrain},
    utils::coord_conversions::cube_to_cartesian,
};

pub fn init_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(&HexPos, &Terrain)>,
) {
    let mut trans_terr_map: IndexMap<&HexPos, &Terrain> = IndexMap::new();

    for (hex_pos, terrain) in query.iter() {
        trans_terr_map.insert(hex_pos, terrain);
    }

    let mut random_hex_pos: &HexPos = &HexPos::new(0, 0, 0);
    let mut pos_found: bool = false;
    let mut random_index: usize;
    while !pos_found {
        random_index = thread_rng().gen_range(0..trans_terr_map.keys().len());
        match trans_terr_map.get_index(random_index).unwrap().1 {
            &Terrain::Coastal => (),
            &Terrain::Debug => (),
            &Terrain::Desert => pos_found = true,
            &Terrain::Grassland => pos_found = true,
            &Terrain::Ice => (),
            &Terrain::Mountain => (),
            &Terrain::Ocean => (),
            &Terrain::Snow => (),
            &Terrain::Steppe => pos_found = true,
            &Terrain::Tundra => (),
        }
        if pos_found {
            random_hex_pos = trans_terr_map.get_index(random_index).unwrap().0;
        }
    }

    let (x, y, z) = cube_to_cartesian(
        random_hex_pos.q as f32,
        random_hex_pos.r as f32,
        random_hex_pos.s as f32,
    );
    let my_transform = Transform::from_xyz(x, y, z);
    let unit_model = SceneBundle {
        scene: asset_server.load("units/unit.glb#Scene0"),
        transform: my_transform,
        ..Default::default()
    };

    commands.spawn(unit_model);
}
