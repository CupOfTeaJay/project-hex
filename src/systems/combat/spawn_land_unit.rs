/*
    Such is Life
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

use crate::components::combat::land_unit_bundle::LandUnitBundle;
use crate::components::combat::land_unit_class::LandUnitClass;
use crate::components::common::hex_pos::HexPos;
use crate::utils::coord_conversions::hex_pos_to_vec3;

pub fn spawn_land_unit(asset_server: Res<AssetServer>, mut commands: Commands) {
    let pos = HexPos::new(0.0, 0.0, 0.0);
    let (x, y, z) = hex_pos_to_vec3(pos.q, pos.r, pos.s);

    // Initialize the model.
    let model: SceneBundle = SceneBundle {
        scene: asset_server.load("units/unit.glb#Scene0".to_string()),
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
    };

    commands.spawn((LandUnitBundle::new(pos, model, LandUnitClass::Infantry),));
}
