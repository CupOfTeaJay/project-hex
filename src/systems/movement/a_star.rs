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

use crate::components::common::hex_pos::HexPos;

type Node = (
    u32, // G-cost.
    u32, // H-cost.
    u32, // F-cost.
);

pub fn a_star(origin: HexPos, destination: HexPos) -> Vec<HexPos> {
    // Initialize return vector.
    let mut path: Vec<HexPos> = Vec::new();

    // Return the shortest path.
    path
}
