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
                background_color: BackgroundColor(Color::linear_rgba(224.0, 181.0, 63.0, 0.5)),
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
                background_color: BackgroundColor(Color::linear_rgba(207.0, 95.0, 41.0, 0.5)),
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
                background_color: BackgroundColor(Color::linear_rgba(63.0, 160.0, 224.0, 0.5)),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(|| println!("'Technology' button clicked.")),
            marker: TechnologyButtonMarker,
        }
    }
}
