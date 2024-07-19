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

use crate::components::combat::health::Health;
use crate::components::common::hex_pos::HexPos;
use crate::components::common::is_movable::IsMovable;
use crate::components::common::movement_buffer::MovementBuffer;

#[derive(Bundle)]
pub struct UnitBundle {
    pos: HexPos,
    health: Health,
    movable: IsMovable,
    movbuff: MovementBuffer,
    model: SceneBundle,
}

impl UnitBundle {
    pub fn new(pos: HexPos, model: SceneBundle) -> Self {
        UnitBundle {
            pos: pos,
            health: Health::new(100),
            movable: IsMovable::new(true),
            movbuff: MovementBuffer::new(Vec::new()),
            model: model,
        }
    }
}
