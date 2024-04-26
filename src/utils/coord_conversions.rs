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

/// Converts a cartesian coordinates to cube coordinates.
fn vec3_to_hex_pos(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    // Perform coordinate conversion.
    let q: f32 = (1.0 / 3.0_f32.sqrt()) * x;
    let r: f32 = (2.0 / 3.0) * z;
    let s: f32 = -(1.0 / 3.0_f32.sqrt()) * x;

    // Return new cube coordinates.
    (q, r, s)
}

/// Converts cube coordinates to cartesian coordinates.
pub fn hex_pos_to_vec3(q: f32, r: f32, s: f32) -> (f32, f32, f32) {
    // Perform coordinate conversion.
    let x: f32 = (3.0_f32.sqrt() / 2.0) * (q - s);
    let y: f32 = 0.0;
    let z: f32 = 1.5 * r;

    // Return new cartesian coordinates.
    (x, y, z)
}
