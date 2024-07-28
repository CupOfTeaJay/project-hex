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

use crate::plugins::ui::backend::systems::button_callbacks::send_settle_event;
use crate::plugins::ui::frontend::bundles::buttons::EndTurnButton;
use crate::plugins::ui::frontend::bundles::texts::EndTurnText;
use crate::plugins::ui::frontend::bundles::texts::OpponentTurnText;
use crate::plugins::ui::frontend::components::markers::HudBottomRightWidgetMarker;
use crate::plugins::ui::frontend::components::markers::HudEndTurnButtonMarker;

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

// TODO: make bundle.
pub fn toggle_bottom_right_widget_pilgrim_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<HudBottomRightWidgetMarker>>,
) {
    // Update view.
    commands
        .entity(ui_query.get_single().unwrap())
        .despawn_descendants()
        .with_children(|parent| {
            // "Settle" button.
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(40.0),
                            border: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                        ..default()
                    },
                    On::<Pointer<Click>>::run(
                        toggle_bottom_right_widget_default_view.pipe(send_settle_event),
                    ),
                ))
                // "Settle" button text.
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settle",
                        TextStyle { ..default() },
                    ));
                });
            // "End turn" button.
            parent.spawn(EndTurnButton::new()).with_children(|parent| {
                parent.spawn(EndTurnText::new());
            });
        });
}
