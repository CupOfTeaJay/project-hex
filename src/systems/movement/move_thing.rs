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
use bevy_mod_picking::selection::PickSelection;

use crate::{
    components::common::{hex_pos::HexPos, is_movable::IsMovable, is_traversable::IsTraversable},
    events::movement_event::MovementEvent,
    utils::coord_conversions::cube_to_cartesian,
};

pub fn move_thing(
    mut movement_event: EventReader<MovementEvent>,
    mut origin_data: Query<(&mut HexPos, &mut Transform), With<IsMovable>>,
    mut dest_data: Query<&mut PickSelection, With<IsTraversable>>,
) {
    for event in movement_event.read() {
        if let Ok((mut origin_hex_pos, mut origin_transform)) =
            origin_data.get_mut(event.origin_entity)
        {
            if let Ok((mut dest_pick_selection)) = dest_data.get_mut(event.dest_entity) {
                origin_hex_pos.q = event.dest_pos.q;
                origin_hex_pos.r = event.dest_pos.r;
                origin_hex_pos.s = event.dest_pos.s;

                let (x, y, z) = cube_to_cartesian(
                    event.dest_pos.q as f32,
                    event.dest_pos.r as f32,
                    event.dest_pos.s as f32,
                );

                origin_transform.translation.x = x;
                origin_transform.translation.y = y;
                origin_transform.translation.z = z;

                dest_pick_selection.is_selected = false;
            }
        }
    }
}
