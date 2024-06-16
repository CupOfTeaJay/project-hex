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
use indexmap::{IndexMap, IndexSet};

use crate::components::common::hex_pos::HexPos;
use crate::resources::pos_neighbors_map::PosNeighborsMap;
use crate::resources::traversability_maps::TraversabilityMaps;

type Node = (
    u32, // F-Cost.
    u32, // G-Cost.
    u32, // H-Cost.
);

/// TODO: Add traversability for all unit types.
pub fn a_star(
    a: &HexPos,
    b: &HexPos,
    pos_neighbors_map: &Res<PosNeighborsMap>,
    traversability_maps: &Res<TraversabilityMaps>,
) -> IndexMap<HexPos, HexPos> {
    // Path ?
    let mut graph: IndexMap<HexPos, HexPos> = IndexMap::new();

    // Initialize the open / closed position sets.
    let mut open_pos: IndexSet<HexPos> = IndexSet::new();
    let mut closed_pos: IndexSet<HexPos> = IndexSet::new();

    // Initialize a HexPos ---> Node map, insert the a pos / node relation.
    let mut pos_node_map: IndexMap<HexPos, Node> = IndexMap::new();
    pos_node_map.insert(*a, init_node(a, a, b));

    // Loop helpers.
    let mut cost_to_neighbor: u32;
    let mut curr_pos: HexPos;
    let mut neighbor_node: Node;

    // Loop.
    open_pos.insert(*a);
    while !open_pos.is_empty() {
        // Find cheapest node.
        curr_pos = *open_pos.get_index(0).unwrap();
        for pos in open_pos.iter() {
            if pos_node_map.get(pos).unwrap().0 < pos_node_map.get(&curr_pos).unwrap().0
                || pos_node_map.get(pos).unwrap().2 < pos_node_map.get(&curr_pos).unwrap().2
            {
                curr_pos = *pos;
            }
        }

        // Update sets.
        open_pos.swap_remove(&curr_pos);
        closed_pos.insert(curr_pos);

        // Loop through neighbors.
        for neighbor in pos_neighbors_map.map.get(&curr_pos).unwrap().iter() {
            if neighbor == b {
                graph.insert(*neighbor, curr_pos);
                return graph;
            }

            if let Some(is_traversable) = traversability_maps
                .pos_land_traversability_map
                .get(neighbor)
            {
                // If this neighbor is traversable and not closed...
                if *is_traversable && !closed_pos.contains(neighbor) {
                    // quickly give the neighbor a node to work with.
                    pos_node_map.insert(*neighbor, (0, calc_displacement(neighbor, a), 0));

                    cost_to_neighbor = pos_node_map.get(&curr_pos).unwrap().1
                        + calc_displacement(&curr_pos, neighbor);
                    neighbor_node = *pos_node_map.get_mut(neighbor).unwrap();

                    // If this is not in the open set, OR if it is and the g_cost can be minimized.
                    if !open_pos.contains(neighbor) || cost_to_neighbor < neighbor_node.1 {
                        neighbor_node.1 = cost_to_neighbor;
                        graph.insert(*neighbor, curr_pos);

                        if !open_pos.contains(neighbor) {
                            neighbor_node.2 = calc_displacement(neighbor, b);
                            neighbor_node.0 = neighbor_node.1 + neighbor_node.2;
                            open_pos.insert(*neighbor);
                        }
                    }
                }
            }
        }
    }

    graph
}

/// Calculates the displacement between two positions.
fn calc_displacement(a: &HexPos, b: &HexPos) -> u32 {
    (((a.q - b.q).abs() + (a.r - b.r).abs() + (a.s - b.s).abs()) / 2) as u32
}

// Initializes an A* node with cost values.
fn init_node(pos: &HexPos, a: &HexPos, b: &HexPos) -> Node {
    let g_cost = calc_displacement(pos, a);
    let h_cost = calc_displacement(pos, b);
    (g_cost + h_cost, g_cost, h_cost)
}
