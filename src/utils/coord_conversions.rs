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

use crate::components::hex_pos::HexPos;

/// Converts a cartesian three-vector to a hex position (cube coordinates).
fn vec3_to_hex_pos(cartesian_coords: Vec3) -> HexPos {
    // Perform coordinate conversion.
    let q: f32 = (1.0 / 3.0_f32.sqrt()) * cartesian_coords.x;
    let r: f32 = (2.0 / 3.0) * cartesian_coords.z;
    let s: f32 = -(1.0 / 3.0_f32.sqrt()) * cartesian_coords.x;

    // Return new cube coordinates.
    HexPos::new(q, r, s)
}

/// Converts a hex position (cube coordinates) to a cartesian three-vector.
pub fn hex_pos_to_vec3(cube_coords: HexPos) -> Vec3 {
    // Perform coordinate conversion.
    let x: f32 = (3.0_f32.sqrt() / 2.0) * (cube_coords.q - cube_coords.s);
    let y: f32 = 0.0;
    let z: f32 = 1.5 * cube_coords.r;

    // Return the new three-vector.
    Vec3::new(x, y, z)
}
