/*
    Project Hex
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

use crate::common::components::movement::HexPos;
use crate::resources::map_parameters::MapParameters;

/// Generates a hash table that maps any cube coordinate to its neighboring cube coordinates.
pub fn init_pos_neighbors_map(map_par: &Res<MapParameters>) -> IndexMap<HexPos, Vec<HexPos>> {
    // Init map to return.
    let mut pos_neighbors_map: IndexMap<HexPos, Vec<HexPos>> = IndexMap::new();

    // Loop constraints.
    let mut q_min: i32 = 0;
    let mut q_max: i32 = map_par.width as i32;
    let r_min: i32 = 0;
    let r_max: i32 = map_par.height as i32;

    // Determine every possible cube coordinate for our map, parameterized by our map width and
    // height.
    let mut s: i32;
    for r in r_min..r_max {
        if r % 2 == 0 && r != 0 {
            q_min -= 1;
            q_max -= 1;
        }
        for q in q_min..q_max {
            s = -q - r;

            // Insert data into the return map after determining this position's neighbors.
            pos_neighbors_map.insert(
                HexPos::new(&q, &r, &s),
                determine_neighbors((q, r, s), map_par),
            );
        }
    }

    // Return the position-to-neighbors hash map.
    pos_neighbors_map
}

/// Given some cube coordinate, determines its neighboring cube coordinates. Coordinates that lie
/// upon the edges of the x-axis should wrap by selecting their appropriate neighbors.
#[rustfmt::skip]
fn determine_neighbors(curr_pos: (i32, i32, i32), map_par: &Res<MapParameters>) -> Vec<HexPos> {
    // Init vars for readability.
    let width = map_par.width as i32;
    let q = curr_pos.0;
    let r = curr_pos.1;
    let s = curr_pos.2;

    // Vector to return.
    let neighbors: Vec<HexPos>;

    // Neighbors for tiles on the LEFT edge of the map.
    if r == -2 * q {
        neighbors = vec![
            HexPos::new(&(q + 1), &(r - 1), &s),                          // Northeastern neighbor.
            HexPos::new(&(q + 1), &r, &(s - 1)),                          // Eastern neighbor.
            HexPos::new(&q, &(r + 1), &(s - 1)),                          // Southeastern neighbor.
            HexPos::new(&(q + (width - 1)), &(r + 1), &(-q - r - width)), // Southwestern neighbor.
            HexPos::new(&(q + (width - 1)), &r, &(-q - r - width + 1)),   // Western neighbor.
            HexPos::new(&(q + width), &(r - 1), &(-q - r - width + 1)),   // Northwestern neighbor.
        ]
    } else if r == -2 * q + 1 {
        neighbors = vec![
            HexPos::new(&(q + 1), &(r - 1), &s),                          // Northeastern neighbor.
            HexPos::new(&(q + 1), &r, &(s - 1)),                          // Eastern neighbor.
            HexPos::new(&q, &(r + 1), &(s - 1)),                          // Southeastern neighbor.
            HexPos::new(&(q - 1), &(r + 1), &s),                          // Southwestern neighbor.
            HexPos::new(&(q + (width - 1)), &r, &(-q - r - width + 1)),   // Western neighbor.
            HexPos::new(&q, &(r - 1), &(s + 1)),                          // Northwestern neighbor.
        ];
    }
    // Neighbors for tiles on the RIGHT edge of the map.
    else if r == 2 * (width - q - 1) {
        neighbors = vec![
            HexPos::new(&(q + 1), &(r - 1), &s),                          // Northeastern neighbor.
            HexPos::new(&(q - (width - 1)), &r, &(-q - r + width - 1)),   // Eastern neighbor.
            HexPos::new(&q, &(r + 1), &(s - 1)),                          // Southeastern neighbor.
            HexPos::new(&(q - 1), &(r + 1), &s),                          // Southwestern neighbor.
            HexPos::new(&(q - 1), &r, &(s + 1)),                          // Western neighbor.
            HexPos::new(&q, &(r - 1), &(s + 1)),                          // Northwestern neighbor.
        ];
    } else if r == 2 * (width - q) - 1 {
        neighbors = vec![
            HexPos::new(&(q - (width - 1)), &(r - 1), &(-q - r + width)), // Northeastern neighbor.
            HexPos::new(&(q - (width - 1)), &r, &(-q - r + width - 1)),   // Eastern neighbor.
            HexPos::new(&(q - width), &(r + 1), &(-q - r + width - 1)),   // Southeastern neighbor.
            HexPos::new(&(q - 1), &(r + 1), &s),                          // Southwestern neighbor.
            HexPos::new(&(q - 1), &r, &(s + 1)),                          // Western neighbor.
            HexPos::new(&q, &(r - 1), &(s + 1)),                          // Northwestern neighbor.
        ];

    // Neighbors for tiles that are NOT on the edges, and do NOT need to wrap.
    } else {
        neighbors = vec![
            HexPos::new(&(q + 1), &(r - 1), &s),                          // Northeastern neighbor.
            HexPos::new(&(q + 1), &r, &(s - 1)),                          // Eastern neighbor.
            HexPos::new(&q, &(r + 1), &(s - 1)),                          // Southeastern neighbor.
            HexPos::new(&(q - 1), &(r + 1), &s),                          // Southwestern neighbor.
            HexPos::new(&(q - 1), &r, &(s + 1)),                          // Western neighbor.
            HexPos::new(&q, &(r - 1), &(s + 1)),                          // Northwestern neighbor.
        ];
    }

    // Return the neighbors vector.
    neighbors
}
