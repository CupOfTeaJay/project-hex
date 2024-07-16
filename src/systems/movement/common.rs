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

use crate::components::common::hex_pos::HexPos;
use crate::utils::coord_conversions::calc_distance;

/// An A* Node. Can be linked to a proceeding Node by way of the 'next'
/// pointer.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub f_cost: u32,
    pub g_cost: u32,
    pub h_cost: u32,
    pub pos: HexPos,
    pub next: Option<Box<Node>>,
}

impl Node {
    /// Creates a new A* Node.
    pub fn new(pos: &HexPos, start: &HexPos, end: &HexPos, next: Option<Box<Node>>) -> Self {
        // Calculate the costs of this node.
        let g_cost: u32 = calc_distance(pos, start);
        let h_cost: u32 = calc_distance(pos, end);

        // Allocate and return the node.
        Node {
            f_cost: g_cost + h_cost,
            g_cost: g_cost,
            h_cost: h_cost,
            pos: *pos,
            next: next,
        }
    }
}
