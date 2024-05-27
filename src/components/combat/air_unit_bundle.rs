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

use crate::components::combat::air_unit_class::AirUnitClass;
use crate::components::combat::unit_bundle::UnitBundle;
use crate::components::common::hex_pos::HexPos;

#[derive(Bundle)]
struct AirUnitBundle {
    class: AirUnitClass,
    unit_data: UnitBundle,
}

impl AirUnitBundle {
    fn new(pos: HexPos, class: AirUnitClass, model: SceneBundle) -> Self {
        AirUnitBundle {
            class: class,
            unit_data: UnitBundle::new(pos, model),
        }
    }
}
