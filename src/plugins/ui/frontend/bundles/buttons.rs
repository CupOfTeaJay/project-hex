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

use crate::plugins::ui::backend::systems::button_callbacks::end_turn;
use crate::plugins::ui::frontend::components::markers::HudEndTurnButton;

#[derive(Bundle)]
pub struct EndTurnButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: HudEndTurnButton,
}

impl EndTurnButton {
    pub fn new() -> Self {
        EndTurnButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(end_turn),
            marker: HudEndTurnButton,
        }
    }
}
