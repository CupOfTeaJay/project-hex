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

use crate::components::common::hex_pos::HexPos;
use crate::resources::map_parameters::MapParameters;

/// Generates two hash tables - one maps position to scaffolding, the other position to neighboring
/// positions.
pub fn init_barebones(
    map_par: &Res<MapParameters>,
) -> IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> {
    // We need two maps here. One that maps a position to neighboring positions...
    let mut pos_neighbor_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = IndexMap::new();

    // Vars to iteratively update.
    let mut curr_pos: HexPos = HexPos::new(0.0, 0.0, 0.0);
    let mut q_min: i32 = 0;
    let mut q_max: i32 = map_par.dimensions.width;

    // For every possible position, as defined by the map width and height.
    for r in 0..map_par.dimensions.height {
        curr_pos.r = r as f32;
        if r % 2 == 0 && r != 0 {
            q_min -= 1;
            q_max -= 1;
        }
        for q in q_min..q_max {
            curr_pos.q = q as f32;
            curr_pos.s = (-q - r) as f32;

            // Unsigned integer representation of curr_pos.
            let int_rep: (i32, i32, i32) =
                (curr_pos.q as i32, curr_pos.r as i32, curr_pos.s as i32);

            // Insert data into pos_neighbor_map.
            pos_neighbor_map.insert(int_rep, determine_neighbors(int_rep, map_par));
        }
    }

    // Return the hash map.
    pos_neighbor_map
}

/// Given some position, determines its neighboring positions.
fn determine_neighbors(
    curr_pos: (i32, i32, i32),
    map_par: &Res<MapParameters>,
) -> Vec<(i32, i32, i32)> {
    // Vars for readability.
    let width = map_par.dimensions.width;
    let q = curr_pos.0;
    let r = curr_pos.1;
    let s = curr_pos.2;

    // Vector to return.
    let ret_vec: Vec<(i32, i32, i32)>;

    // Neighbors for tiles on the LEFT edge of the map.
    if r == -2 * q {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q + 1, r, s - 1),                        // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q + (width - 1), r + 1, -q - r - width), // Southwestern neighbor.
            (q + (width - 1), r, -q - r - width + 1), // Western neighbor.
            (q + width, r - 1, -q - r - width + 1),   // Northwestern neighbor.
        ]
    } else if r == -2 * q + 1 {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q + 1, r, s - 1),                        // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q + (width - 1), r, -q - r - width + 1), // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];
    }
    // Neighbors for tiles on the RIGHT edge of the map.
    else if r == 2 * (width - q - 1) {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q - (width - 1), r, -q - r + width - 1), // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q - 1, r, s + 1),                        // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];
    } else if r == 2 * (width - q) - 1 {
        ret_vec = vec![
            (q - (width - 1), r - 1, -q - r + width), // Northeastern neighbor.
            (q - (width - 1), r, -q - r + width - 1), // Eastern neighbor.
            (q - width, r + 1, -q - r + width - 1),   // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q - 1, r, s + 1),                        // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];

    // Neighbors for tiles that are NOT on the edges, and do NOT need to wrap.
    } else {
        ret_vec = vec![
            (q + 1, r - 1, s), // Northeastern neighbor.
            (q + 1, r, s - 1), // Eastern neighbor.
            (q, r + 1, s - 1), // Southeastern neighbor.
            (q - 1, r + 1, s), // Southwestern neighbor.
            (q - 1, r, s + 1), // Western neighbor.
            (q, r - 1, s + 1), // Northwestern neighbor.
        ];
    }

    // Return neighbors.
    ret_vec
}
