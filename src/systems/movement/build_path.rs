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

use crate::common::components::movement::HexPos;
use crate::common::components::movement::MovementBuffer;
use crate::events::build_path_event::BuildPathEvent;
use crate::systems::movement::common::Node;

pub fn build_path(
    mut build_path_event: EventReader<BuildPathEvent>,
    mut movement_buffers: Query<&mut MovementBuffer>,
) {
    for event in build_path_event.read() {
        // Initialize an empty "path" vector.
        let mut path: Vec<HexPos> = Vec::new();
        recurse_nodes(&event.root, &mut path);
        if let Ok(mut movbuff) = movement_buffers.get_mut(event.entity) {
            *movbuff = MovementBuffer::new(path);
        }
    }
}

fn recurse_nodes(node: &Node, path: &mut Vec<HexPos>) {
    path.push(node.pos);
    if let Some(next_node) = &node.next {
        recurse_nodes(next_node, path);
    }
}
