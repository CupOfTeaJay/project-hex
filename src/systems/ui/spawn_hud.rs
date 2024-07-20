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

pub fn spawn_hud(mut commands: Commands) {
    // Root node. Encapsulates the entire screen.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: BorderColor(Color::srgb(1.00, 0.00, 0.00)),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // Left "pane" (vertical split of root).
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        justify_self: JustifySelf::Start,
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                    ..default()
                },
                Pickable::IGNORE,
            ));
            // Right "pane" (vertical split of root).
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        justify_self: JustifySelf::End,
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                    ..default()
                },
                Pickable::IGNORE,
            ));
        });
}
