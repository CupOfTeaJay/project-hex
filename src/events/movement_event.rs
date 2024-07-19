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

#[derive(Event)]
/// An event sent whenever the movement of some entity has been requested.
/// It contains two positions - an origin and destination. These positions are
/// used by the A* pathfinding algorithm to determine the shortest path between
/// them.
pub struct MovementEvent {
    pub entity: Entity,
    pub origin: HexPos,
    pub destination: HexPos,
}

impl MovementEvent {
    pub fn new(entity: Entity, origin: HexPos, destination: HexPos) -> Self {
        MovementEvent {
            entity,
            origin,
            destination,
        }
    }
}
