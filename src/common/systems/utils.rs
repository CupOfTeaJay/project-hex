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

// TODO: Check if this is right... someday.
const SCALE: f32 = 1.0;

/// Converts an input 'HexPos' to Bevy's 'Vec3'.
pub fn hexpos_to_vec3(hexpos: &HexPos) -> Vec3 {
    Vec3::new(
        // X coordinate.
        (3.0_f32.sqrt() / 2.0) * ((hexpos.q - hexpos.s) as f32) * SCALE,
        // Y coordinate.
        0.0,
        // Z coordinate.
        1.5 * (hexpos.r as f32) * SCALE,
    )
}

/// Calculates the distance between two hex positions.
pub fn calc_distance(a: &HexPos, b: &HexPos) -> u32 {
    (((a.q - b.q).abs() + (a.r - b.r).abs() + (a.s - b.s).abs()) / 2) as u32
}

