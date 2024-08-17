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

use crate::plugins::ui::hud::bundles::buttons::EndTurnButton;
use crate::plugins::ui::hud::bundles::texts::EndTurnText;
use crate::plugins::ui::hud::bundles::texts::OpponentTurnText;
use crate::plugins::ui::hud::components::markers::HudBottomRightWidgetMarker;
use crate::plugins::ui::hud::components::markers::HudEndTurnButtonMarker;

pub fn toggle_end_turn_button_player_turn_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<HudEndTurnButtonMarker>>,
) {
    commands
        .entity(ui_query.get_single().unwrap())
        .despawn_descendants()
        .with_children(|end_turn_button| {
            end_turn_button.spawn(OpponentTurnText::new());
        });
}

pub fn toggle_end_turn_button_opponent_turn_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<HudEndTurnButtonMarker>>,
) {
    commands
        .entity(ui_query.get_single().unwrap())
        .despawn_descendants()
        .with_children(|end_turn_button| {
            end_turn_button.spawn(EndTurnText::new());
        });
}

// TODO: decouple EndTurnButton.
pub fn toggle_bottom_right_widget_default_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<HudBottomRightWidgetMarker>>,
) {
    // Update view.
    commands
        .entity(ui_query.get_single().unwrap())
        .despawn_descendants()
        .with_children(|parent| {
            // "End turn" button.
            parent.spawn(EndTurnButton::new()).with_children(|parent| {
                parent.spawn(EndTurnText::new());
            });
        });
}
