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

#[derive(Resource)]
pub struct MapParameters {
    pub width: i32,
    pub height: i32,
    pub diverse_grassland_bias: f32,
    pub diverse_steppe_bias: f32,
    pub equator_desert_bias: f32,
    pub tundra_snow_bias: f32,
    pub tundra_tundra_bias: f32,
    pub icecap_limit: f32,
    pub snow_limit: f32,
}

impl MapParameters {
    pub fn new(
        width: i32,
        height: i32,
        diverse_grassland_bias: f32,
        diverse_steppe_bias: f32,
        equator_desert_bias: f32,
        tundra_snow_bias: f32,
        tundra_tundra_bias: f32,
        icecap_limit: f32,
        snow_limit: f32,
    ) -> Self {
        MapParameters {
            width,
            height,
            diverse_grassland_bias,
            diverse_steppe_bias,
            equator_desert_bias,
            tundra_snow_bias,
            tundra_tundra_bias,
            icecap_limit,
            snow_limit,
        }
    }
}
