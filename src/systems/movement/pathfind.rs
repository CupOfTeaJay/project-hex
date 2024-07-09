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

use crate::events::movement_event::MovementEvent;

use crate::resources::pos_neighbors_map::PosNeighborsMap;
use crate::resources::traversability_maps::TraversabilityMaps;

use crate::utils::coord_conversions::calc_distance;

/// An A* Node. Can be linked to a proceeding Node by way of the 'next'
/// pointer.
#[derive(Clone, Hash, Eq, PartialEq)]
struct Node<'a> {
    f_cost: u32,
    g_cost: u32,
    h_cost: u32,
    pos: &'a HexPos,
    next: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    /// Creates a new A* Node.
    fn new(pos: &'a HexPos, start: &HexPos, end: &HexPos, next: Option<Box<Node<'a>>>) -> Self {
        // Calculate the costs of this node.6
        let g_cost: u32 = calc_distance(pos, start);
        let h_cost: u32 = calc_distance(pos, end);

        // Allocate and return the node.
        Node {
            f_cost: g_cost + h_cost,
            g_cost: g_cost,
            h_cost: h_cost,
            pos,
            next: next,
        }
    }
}

/// The A* pathfinding algorithm.
pub fn pathfind(
    mut movement_event: EventReader<MovementEvent>,
    pos_neighbors_map: Res<PosNeighborsMap>,
    traversability_maps: Res<TraversabilityMaps>,
) {
    // For every movement event received...
    for event in movement_event.read() {
        // Initialize vectors to classify and store A* nodes / positions
        // for the duration of the algorithm.
        let mut open_nodes: IndexSet<Box<Node>> = IndexSet::new();
        let mut closed_nodes: IndexSet<Box<Node>> = IndexSet::new();
        let mut closed_pos: IndexSet<HexPos> = IndexSet::new();

        // Vector to return... TODO:
        let mut path: Vec<&HexPos> = Vec::new();

        // TODO:
        let mut min: u32;

        // Initialize the first node. We're going to be moving backwards here,
        // with the "origin" and "destination" fields of MovementEvent
        // corresponding to "end" and "start", respectively.
        let start: &HexPos = &event.destination;
        let end: &HexPos = &event.origin;
        let mut curr_node: Box<Node> = Box::new(Node::new(start, start, end, None));

        // TODO:
        while curr_node.pos != end {
            // Determine which neighboring positions are traversable and
            // initialize nodes for them.
            for neighbor in pos_neighbors_map.map.get(curr_node.pos).unwrap().iter() {
                if *traversability_maps.pos_land_map.get(neighbor).unwrap() == true {
                    if !closed_pos.contains(neighbor) {
                        open_nodes.insert(Box::new(Node::new(
                            neighbor,
                            start,
                            end,
                            Some(curr_node.clone()),
                        )));
                    }
                }
            }

            // Update the open and closed sets.
            closed_nodes.insert(curr_node.clone());
            closed_pos.insert(*curr_node.pos);
            if open_nodes.contains(&curr_node) {
                open_nodes.swap_remove(&curr_node);
            }

            // Determine the "cheapest" node in open_nodes, and update curr_node
            // accordingly.
            curr_node = open_nodes[0].clone();
            min = curr_node.f_cost;
            for node in open_nodes.iter() {
                if node.f_cost < min {
                    curr_node = node.clone();
                    min = node.f_cost;
                }
            }

            let q = curr_node.pos.q;
            let r = curr_node.pos.r;
            let s = curr_node.pos.s;
            let tq = end.q;
            let tr = end.r;
            let ts = end.s;
            println!("Current position: ({q}, {r}, {s}) --- Target: ({tq}, {tr}, {ts})");
        }

        println!("Target found!");
    }
}
