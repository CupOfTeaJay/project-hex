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
};

pub fn movement_event_trigger(
    mut movement_event: EventWriter<MovementEvent>,
    movable_units: Query<(Entity, &IsMovable, &PickSelection)>,
    traversable_positions: Query<(Entity, &HexPos, &IsTraversable, &PickSelection)>,
) {
    for (origin_entity, is_movable, origin_pick_selection) in movable_units.iter() {
        if is_movable.status && origin_pick_selection.is_selected {
            for (dest_entity, dest_pos, is_traversable, dest_pick_selection) in
                traversable_positions.iter()
            {
                if is_traversable.status && dest_pick_selection.is_selected {
                    println!("Event sent.");
                    movement_event.send(MovementEvent::new(origin_entity, dest_entity, *dest_pos));
                }
            }
        }
    }
}
