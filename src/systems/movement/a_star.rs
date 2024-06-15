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

use std::collections::HashMap;

use crate::components::common::hex_pos::HexPos;
use crate::systems::movement::common::AStarNode;

/// TODO:
pub fn a_star(origin: HexPos, destination: HexPos) -> Vec<HexPos> {
    // Initialize the return vector.
    let mut path: Vec<HexPos> = Vec::new();

    // Initialize a HexPos -> Node map.
    let mut pos_to_node_map: HashMap<HexPos, AStarNode> = HashMap::new();

    // Return the shortest path.
    path
}

/// Calculates the displacement between two cube coordinates.
fn calculate_displacement(a: HexPos, b: HexPos) -> u32 {
    (((a.q - b.q).abs() + (a.r - b.r).abs() + (a.s - b.s).abs()) / 2) as u32
}

/// TODO:
fn calculate_node(origin: HexPos, destination: HexPos, curr_pos: HexPos) -> AStarNode {
    AStarNode::new(
        calculate_displacement(origin, curr_pos),
        calculate_displacement(destination, curr_pos),
    )
}
