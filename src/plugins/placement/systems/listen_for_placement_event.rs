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
use bevy_mod_picking::prelude::*;

use crate::common::components::movement::HexPos;
use crate::common::components::movement::IsTraversable;
use crate::common::events::placement_event::PlacementEvent;

pub fn listen_for_placement_event(
    mut placement_event: EventWriter<PlacementEvent>,
    positions: Query<(&HexPos, &PickSelection), With<IsTraversable>>,
) {
    for (position, selection) in positions.iter() {
        if selection.is_selected {
            placement_event.send(PlacementEvent::new(position));
            break;
        }
    }
}
