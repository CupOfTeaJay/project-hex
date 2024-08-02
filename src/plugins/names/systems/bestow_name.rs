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
use bevy_mod_billboard::prelude::*;

use crate::common::components::movement::HexPos;
use crate::common::resources::city_names::CityNames;
use crate::common::systems::utils::hexpos_to_vec3;
use crate::plugins::city::components::markers::CityMarker;

pub fn bestow_city_name(
    city_names: Res<CityNames<'static>>,
    mut commands: Commands,
    unnamed_cities: Query<(Entity, &HexPos), (With<CityMarker>, Without<Name>)>,
) {
    for (city, position) in unnamed_cities.iter() {
        let city_name: String = city_names.get_random_name();
        commands.entity(city).insert(Name::new(city_name.clone()));
        commands.spawn(BillboardTextBundle {
            transform: Transform::from_translation(hexpos_to_vec3(position).with_y(1.5))
                .with_scale(Vec3::splat(0.0085)),
            text: Text::from_section(
                city_name.clone(),
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            ..default()
        });
    }
}
