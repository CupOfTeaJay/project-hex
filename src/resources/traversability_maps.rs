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

use std::collections::HashMap;

use bevy::prelude::*;

use crate::common::components::movement::HexPos;

#[derive(Resource)]
pub struct TraversabilityMaps {
    pub pos_land_map: HashMap<HexPos, bool>,
    pub pos_sea_map: HashMap<HexPos, bool>,
}

impl TraversabilityMaps {
    pub fn new() -> Self {
        TraversabilityMaps {
            pos_land_map: HashMap::new(),
            pos_sea_map: HashMap::new(),
        }
    }
}
