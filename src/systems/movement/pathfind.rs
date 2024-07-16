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
use indexmap::IndexSet;

use crate::components::common::hex_pos::HexPos;

use crate::events::build_path_event::BuildPathEvent;
use crate::events::movement_event::MovementEvent;

use crate::resources::pos_neighbors_map::PosNeighborsMap;
use crate::resources::traversability_maps::TraversabilityMaps;

use crate::systems::movement::common::Node;

pub fn pathfind(
    mut movement_event: EventReader<MovementEvent>,
    mut build_path_event: EventWriter<BuildPathEvent>,
    pos_neighbors_map: Res<PosNeighborsMap>,
    traversability_maps: Res<TraversabilityMaps>,
) {
    // For every movement event received...
    for event in movement_event.read() {
        // Initialize reference positions. We're going to be moving backwards
        // here, with "start" and "end" referring to the movement event's
        // "destination" and "origin", respectively.
        let start: HexPos = event.destination;
        let end: HexPos = event.origin;

        // Initialize open and closed sets to store nodes for the duration of
        // the algorithm.
        let mut open_nodes: IndexSet<Node> = IndexSet::new();
        let mut closed_nodes: IndexSet<Node> = IndexSet::new();
        let mut open_pos: IndexSet<HexPos> = IndexSet::new();
        let mut closed_pos: IndexSet<HexPos> = IndexSet::new();

        // Initialize the starting node.
        let mut curr_node: Node = Node::new(&start, &start, &end, None);
        open_nodes.insert(curr_node.clone());
        open_pos.insert(start);

        // Loop until we've found a path that links "start" to "end".
        while curr_node.pos != end {
            // First things first, grab the current node's neighbors and
            // initialize them in the open nodes set if they're traversable,
            // not present in the open positons set, and not present in the
            // closed positions set.
            for neighbor_pos in pos_neighbors_map.map.get(&curr_node.pos).unwrap() {
                if *traversability_maps.pos_land_map.get(neighbor_pos).unwrap() {
                    if !open_pos.contains(neighbor_pos) && !closed_pos.contains(neighbor_pos) {
                        open_pos.insert(*neighbor_pos);
                        open_nodes.insert(Node::new(
                            neighbor_pos,
                            &start,
                            &end,
                            Some(Box::new(curr_node.clone())),
                        ));
                    }
                }
            }

            // Add the current node and its position to the closed sets,
            // we don't need to look at them anymore.
            open_nodes.swap_remove(&curr_node);
            closed_nodes.insert(curr_node.clone());
            open_pos.swap_remove(&curr_node.pos);
            closed_pos.insert(curr_node.pos);

            // Iterate the curr_node by finding the cheapest node in the open
            // nodes set.
            curr_node = open_nodes.get_index(0).unwrap().clone();
            for node in open_nodes.iter() {
                if node.f_cost < curr_node.f_cost {
                    curr_node = node.clone();
                } else if node.f_cost == curr_node.f_cost && node.g_cost < curr_node.f_cost {
                    curr_node = node.clone();
                }
            }
        }

        build_path_event.send(BuildPathEvent::new(curr_node));
    }
}
