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

use crate::plugins::ui::frontend::components::{
    markers::HudBottomLeftWidget, markers::HudBottomRightWidget, markers::HudLeftPane,
    markers::HudRightPane, markers::HudRoot, markers::HudTopLeftWidget, markers::HudTopRightWidget,
};

use crate::plugins::ui::frontend::bundles::buttons::EndTurnButton;
use crate::plugins::ui::frontend::bundles::texts::EndTurnText;

pub fn init_hud(mut commands: Commands) {
    // Root node. Encapsulates the entire screen.
    commands
        .spawn((
            HudRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(1.00, 0.00, 0.00)),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // Left "pane" (vertical split of root).
            parent
                .spawn((
                    HudLeftPane,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            // border: UiRect::all(Val::Px(5.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        // border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    // Top-left "widget".
                    parent.spawn((
                        HudTopLeftWidget,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(12.5),
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                            ..default()
                        },
                    ));
                    // Bottom-left "widget".
                    parent.spawn((
                        HudBottomLeftWidget,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(12.5),
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                            ..default()
                        },
                    ));
                });
            // Right "pane" (vertical split of root).
            parent
                .spawn((
                    HudRightPane,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            // border: UiRect::all(Val::Px(5.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        // border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    // Top-right "widget".
                    parent.spawn((
                        HudTopRightWidget,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(12.5),
                                border: UiRect::all(Val::Px(5.0)),
                                align_self: AlignSelf::End,
                                ..default()
                            },
                            border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                            ..default()
                        },
                    ));
                    // Bottom-right "widget".
                    parent
                        .spawn((
                            HudBottomRightWidget,
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(75.0),
                                    height: Val::Percent(12.5),
                                    border: UiRect::all(Val::Px(5.0)),
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::End,
                                    ..default()
                                },
                                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                                ..default()
                            },
                        ))
                        .with_children(|bottom_right_widget| {
                            bottom_right_widget
                                // "End turn" button.
                                .spawn(EndTurnButton::new())
                                .with_children(|end_turn_button| {
                                    end_turn_button
                                        // ""End turn" text.
                                        .spawn(EndTurnText::new());
                                });
                        });
                });
        });
}
