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

use crate::plugins::ui::rnd::components::markers::RndTechnologyRootMarker;

pub fn destruct_rnd_technology(
    mut commands: Commands,
    rnd_technology_root: Query<Entity, With<RndTechnologyRootMarker>>,
) {
    commands
        .entity(rnd_technology_root.get_single().unwrap())
        .despawn_recursive();
}
