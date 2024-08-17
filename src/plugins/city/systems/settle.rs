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
use crate::common::events::pickable_spawn_event::PickableSpawnEvent;
use crate::common::events::settle_event::SettleEvent;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::resources::selection_focus::SelectionFocus;
use crate::plugins::city::bundles::city_center::CityCenter;

pub fn settle(
    assets: Res<AssetHandles>,
    mut commands: Commands,
    mut pickable_spawn_event: EventWriter<PickableSpawnEvent>,
    positions: Query<&HexPos>,
    mut selection_focus: ResMut<SelectionFocus>,
    mut settle_event: EventReader<SettleEvent>,
) {
    for _ in settle_event.read() {
        if let Some(subject) = selection_focus.subject {
            if let Ok(position) = positions.get(subject) {
                // Despawn the pilgrim (should be the subject of the selection
                // focus).
                commands.entity(subject).despawn_recursive();

                // Spawn (settle) a new city in the despawned pilgrim's place,
                // and make sure it is selectable.
                pickable_spawn_event.send(PickableSpawnEvent::new(
                    commands.spawn(CityCenter::new(&assets, &position)).id(),
                ));

                // Clear the selection focus.
                selection_focus.clear_focus();
            } else {
                panic!("Error: 'settle' called with invalid 'position' field for 'SelectionFocus'");
            }
        } else {
            panic!("Error: 'settle' called with invalid 'subject' field for 'SelectionFocus'");
        }
    }
}
