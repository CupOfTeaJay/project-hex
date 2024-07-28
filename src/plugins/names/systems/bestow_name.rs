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

use crate::common::components::movement::HexPos;
use crate::common::systems::utils::hexpos_to_vec3;
use crate::plugins::names::components::markers::UnnamedCityMarker;

pub fn bestow_city_name(
    mut commands: Commands,
    unnamed_cities: Query<(Entity, &HexPos), With<UnnamedCityMarker>>,
) {
    for (city, position) in unnamed_cities.iter() {
        commands.entity(city).remove::<UnnamedCityMarker>();
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                "Poop town",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ),
            transform: Transform::from_translation(hexpos_to_vec3(position).with_y(10.0)),
            ..default()
        });
        println!("City named.");
    }
}
