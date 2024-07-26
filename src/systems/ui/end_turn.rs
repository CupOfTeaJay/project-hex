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

use crate::components::ui::hud::EndTurnButton;
use crate::states::game_state::GameState;

pub fn end_turn(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    ui_query: Query<Entity, With<EndTurnButton>>,
) {
    let end_turn_button = ui_query.get_single().unwrap();
    commands
        .entity(end_turn_button)
        .despawn_descendants()
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Wait...",
                TextStyle { ..default() },
            ));
        });
    next_game_state.set(GameState::OpponentTurn);
}
