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

use crate::common::states::ui_state::UiState;
use crate::plugins::ui::rnd::components::markers::EconomyButtonMarker;
use crate::plugins::ui::rnd::components::markers::MilitaryButtonMarker;
use crate::plugins::ui::rnd::components::markers::TechnologyButtonMarker;

#[derive(Bundle)]
pub struct EconomyButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: EconomyButtonMarker,
}

impl EconomyButton {
    pub fn new() -> Self {
        EconomyButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(33.33),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                background_color: BackgroundColor(Color::srgb(1.00, 0.79, 0.48)),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(|| println!("'Economy' button clicked.")),
            marker: EconomyButtonMarker,
        }
    }
}

#[derive(Bundle)]
pub struct MilitaryButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: MilitaryButtonMarker,
}

impl MilitaryButton {
    pub fn new() -> Self {
        MilitaryButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(33.33),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                background_color: BackgroundColor(Color::srgb(0.65, 0.13, 0.0)),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(|| println!("'Military' button clicked.")),
            marker: MilitaryButtonMarker,
        }
    }
}

#[derive(Bundle)]
pub struct TechnologyButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: TechnologyButtonMarker,
}

impl TechnologyButton {
    pub fn new() -> Self {
        TechnologyButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(33.33),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                background_color: BackgroundColor(Color::srgb(0.0, 0.23, 0.34)),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(|mut next_ui_state: ResMut<NextState<UiState>>| {
                next_ui_state.set(UiState::RndTechnology);
            }),
            marker: TechnologyButtonMarker,
        }
    }
}
