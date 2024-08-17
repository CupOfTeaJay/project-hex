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

use crate::common::components::labels::Label;
use crate::common::events::pickable_spawn_event::PickableSpawnEvent;
use crate::common::events::placement_event::PlacementEvent;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::resources::placement_focus::PlacementFocus;
use crate::common::states::placement_state::PlacementState;
use crate::plugins::city::bundles::zones::MartialZone;
use crate::plugins::city::components::heirarchy::Zones;

pub fn place(
    assets: Res<AssetHandles>,
    mut commands: Commands,
    mut next_placement_state: ResMut<NextState<PlacementState>>,
    mut pickable_spawn_event: EventWriter<PickableSpawnEvent>,
    mut placement_event: EventReader<PlacementEvent>,
    mut placement_focus: ResMut<PlacementFocus>,
    mut zones: Query<&mut Zones>,
) {
    for event in placement_event.read() {
        match placement_focus.label {
            Label::MartialZone => {
                // Spawn a new martial zone.
                let martial_zone: Entity = commands
                    .spawn(MartialZone::new(
                        &assets,
                        &placement_focus.subject.unwrap(),
                        &event.position,
                    ))
                    .id();

                // Append the newly-spawned martial zone to the parent city's
                // 'Zone' component.
                zones
                    .get_mut(placement_focus.subject.unwrap())
                    .unwrap()
                    .ids
                    .push(martial_zone);

                // Send a pickable spawn event for the newly spawned martial zone.
                pickable_spawn_event.send(PickableSpawnEvent::new(martial_zone));
            }
            _ => {}
        }

        placement_focus.clear_focus();
        next_placement_state.set(PlacementState::Inactive);
    }
}
