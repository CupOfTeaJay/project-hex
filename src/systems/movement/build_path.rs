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

use crate::events::build_path_event::BuildPathEvent;
use crate::systems::movement::common::Node;

pub fn build_path(mut build_path_event: EventReader<BuildPathEvent>) {
    for event in build_path_event.read() {
        recurse_nodes(&event.root);
    }
}

fn recurse_nodes(node: &Node) {
    let q = node.pos.q;
    let r = node.pos.r;
    let s = node.pos.s;
    println!("({q}, {r}, {s})");
    if let Some(next_node) = &node.next {
        recurse_nodes(next_node);
    }
}
