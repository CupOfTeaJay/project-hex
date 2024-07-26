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

use crate::components::ui::hud::HudBottomRightWidget;
use crate::systems::ui::default_brw_view::show_default_brw_view;
use crate::systems::ui::settle::settle;

pub fn show_pilgrim_view(mut commands: Commands, ui_query: Query<(Entity, &HudBottomRightWidget)>) {
    // Update view.
    commands
        .entity(ui_query.get_single().unwrap().0)
        .despawn_descendants()
        // "Settle" button.
        .with_children(|parent| {
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
                    On::<Pointer<Click>>::run(settle.pipe(show_default_brw_view)),
                ))
                // "Settle" button text.
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settle",
                        TextStyle { ..default() },
                    ));
                });
        });
}
