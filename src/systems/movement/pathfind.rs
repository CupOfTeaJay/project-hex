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

use crate::components::common::hex_pos::HexPos;

use crate::events::movement_event::MovementEvent;

use crate::resources::pos_neighbors_map::PosNeighborsMap;

use crate::utils::coord_conversions::calc_distance;

/// An A* Node. Can be linked to a proceeding Node by way of the 'next'
/// pointer.
struct Node<'a> {
    f_cost: u32,
    g_cost: u32,
    h_cost: u32,
    pos: &'a HexPos,
    next: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    /// Creates a new A* Node.
    fn new(pos: &'a HexPos, start: &HexPos, end: &HexPos) -> Self {
        // Calculate the costs of this node.6
        let g_cost: u32 = calc_distance(pos, start);
        let h_cost: u32 = calc_distance(pos, end);

        // Allocate and return the node.
        Node {
            f_cost: g_cost + h_cost,
            g_cost: g_cost,
            h_cost: h_cost,
            pos,
            next: None,
        }
    }
}

/// The A* pathfinding algorithm.
pub fn pathfind(
    mut movement_event: EventReader<MovementEvent>,
    pos_neighbors_map: Res<PosNeighborsMap>,
) {
    // For every movement event received...
    for event in movement_event.read() {
        // Initialize the first A* node. We're going to be moving backwards here,
        // with the "origin" and "destination" fields of MovementEvent
        // corresponding to "end" and "start", respectively.
        let start: &HexPos = &event.destination;
        let end: &HexPos = &event.origin;
        let mut curr_node: Node = Node::new(start, start, end);
    }
}
