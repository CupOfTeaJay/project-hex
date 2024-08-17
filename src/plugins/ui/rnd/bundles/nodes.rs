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

use crate::plugins::ui::rnd::components::markers::RndLandingRootMarker;

#[derive(Bundle)]
pub struct RndLandingRoot {
    marker: RndLandingRootMarker,
    node: NodeBundle,
    pickability: Pickable,
}

impl RndLandingRoot {
    pub fn new() -> Self {
        RndLandingRoot {
            marker: RndLandingRootMarker,
            node: NodeBundle {
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
            pickability: Pickable::IGNORE,
        }
    }
}
