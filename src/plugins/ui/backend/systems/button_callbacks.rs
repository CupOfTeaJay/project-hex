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
use crate::common::systems::utils::hexpos_to_vec3;
use crate::events::pickable_spawn_event::PickableSpawnEvent;
use crate::resources::asset_handles::AssetHandles;
use crate::resources::selection_focus::SelectionFocus;
use crate::states::game_state::GameState;

/// Callback function for the "End turn" button.
pub fn end_turn(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::OpponentTurn);
}

/// Callback function for the "Settle" button.
/// TODO: Refactor "PickableSpawnEvent"
pub fn settle(
    assets: Res<AssetHandles>,
    mut commands: Commands,
    hexpos_query: Query<&HexPos>,
    mut pickable_spawn_event: EventWriter<PickableSpawnEvent>,
    mut selection_focus: ResMut<SelectionFocus>,
) {
    if let Some(selection) = selection_focus.focus {
        // Despawn the selection focus (which SHOULD be some selected pilgrim).
        commands.entity(selection).despawn_recursive();

        // Spawn a new city in the pilgrim's place.
        if let Ok(position) = hexpos_query.get(selection) {
            pickable_spawn_event.send(PickableSpawnEvent::new(
                commands
                    .spawn(SceneBundle {
                        scene: assets.scenes.city_center.clone().unwrap(),
                        transform: Transform::from_translation(hexpos_to_vec3(position)),
                        ..default()
                    })
                    .id(),
            ));
        } else {
            panic!("Error: 'settle' callback used with an invalid 'SelectionFocus'.");
        }

        // Clear the selection focus.
        selection_focus.clear_focus();
    } else {
        panic!("Error: 'settle' callback used without an empty 'SelectionFocus'.");
    }
}
