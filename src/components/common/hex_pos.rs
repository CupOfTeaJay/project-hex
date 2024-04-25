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

/// Cube coordinates relative to the origin of a global hexagonal grid.
#[derive(Clone, Component, Copy)]
pub struct HexPos {
    pub q: f32,
    pub r: f32,
    pub s: f32,
}

impl HexPos {
    /// Creates cube coordinates.
    pub fn new(q: f32, r: f32, s: f32) -> Self {
        HexPos { q, r, s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_pos_new() {
        // Set up.
        let q: f32 = 1.0;
        let r: f32 = 2.0;
        let s: f32 = 3.0;

        // Call unit under test.
        let pos = HexPos::new(q, r, s);

        // Validate injected data.
        assert!(pos.q == q);
        assert!(pos.r == r);
        assert!(pos.s == s);
    }
}
