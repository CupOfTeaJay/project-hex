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
use bevy_mod_picking::PickableBundle;

use crate::events::unit_spawn_event::UnitSpawnEvent;

/// Makes a unit scene pickable (selectable).
pub fn make_unit_pickable(
    mut unit_spawn_event: EventReader<UnitSpawnEvent>,
    mut commands: Commands,
) {
    for event in unit_spawn_event.read() {
        commands
            .entity(event.entity)
            .insert(PickableBundle::default());
    }
}
