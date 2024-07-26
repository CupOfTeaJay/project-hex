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

#[rustfmt::skip]
use crate::events::{
    build_path_event::BuildPathEvent,
    movement_event::MovementEvent,
    pickable_spawn_event::PickableSpawnEvent,
};

/// Plugin that registers events with the main application. Currently, the EventsPlugin:
///     - Registers "TileSpawnEvent".
///     - Registers "UnitSpawnEvent".
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        // Register events with the main application.
        app.add_event::<BuildPathEvent>()
            .add_event::<MovementEvent>()
            .add_event::<PickableSpawnEvent>();
    }
}
