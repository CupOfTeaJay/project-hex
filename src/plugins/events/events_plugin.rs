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

use crate::common::events::build_path_event::BuildPathEvent;
use crate::common::events::movement_event::MovementEvent;
use crate::common::events::pickable_spawn_event::PickableSpawnEvent;
use crate::common::events::placement_event::PlacementEvent;
use crate::common::events::settle_event::SettleEvent;
use crate::common::events::train_unit_event::TrainUnitEvent;

/// Plugin that registers events with the main application. Currently, the EventsPlugin:
///     - Registers "TileSpawnEvent".
///     - Registers "UnitSpawnEvent".
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        // Register events with the main application.
        app.add_event::<BuildPathEvent>()
            .add_event::<MovementEvent>()
            .add_event::<PickableSpawnEvent>()
            .add_event::<PlacementEvent>()
            .add_event::<SettleEvent>()
            .add_event::<TrainUnitEvent>();
    }
}
