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
use crate::common::components::movement::HexPos;
use crate::common::events::pickable_spawn_event::PickableSpawnEvent;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::systems::builders::unit_builder;
use crate::plugins::cost::components::cost::Cost;
use crate::plugins::cost::components::markers::TrainingUnitCostMarker;

pub fn spawn_trained_units(
    assets: Res<AssetHandles>,
    mut commands: Commands,
    costs: Query<(Entity, &Parent, &Cost), With<TrainingUnitCostMarker>>,
    mut pickable_spawn_event: EventWriter<PickableSpawnEvent>,
    positions: Query<&HexPos>,
) {
    for cost in costs.iter() {
        if cost.2.spent == cost.2.principal {
            if let Ok(position) = positions.get(**cost.1) {
                pickable_spawn_event.send(PickableSpawnEvent::new(
                    commands
                        .spawn(unit_builder(&assets, &Label::Pilgrim, position))
                        .id(),
                ));
                commands.entity(cost.0).despawn_recursive();
            }
        }
    }
}
