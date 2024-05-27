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

/// Cube coordinates relative to the origin of a global hexagonal grid.
#[derive(Clone, Component, Copy, Hash, Eq, PartialEq)]
pub struct HexPos {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexPos {
    /// Creates cube coordinates.
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        HexPos { q, r, s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_pos_new() {
        // Set up.
        let q: i32 = 1;
        let r: i32 = 1;
        let s: i32 = -2;

        // Call unit under test.
        let pos = HexPos::new(q, r, s);

        // Validate injected data.
        assert!(pos.q == q);
        assert!(pos.r == r);
        assert!(pos.s == s);
    }
}
