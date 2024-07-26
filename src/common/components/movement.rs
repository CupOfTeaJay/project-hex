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

/// Cube coordinates relative to the origin of a global hexagonal grid. Thereby
/// referred to as 'HexPos' or 'Hex Position'.
#[derive(Clone, Component, Copy, Hash, Eq, PartialEq)]
pub struct HexPos {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexPos {
    /// Create a new hex position with the input cube coordinates.
    pub fn new(q: &i32, r: &i32, s: &i32) -> Self {
        HexPos {
            q: *q,
            r: *r,
            s: *s,
        }
    }
}

/// Vector used to store unvisited hex positions a movable entity is planning
/// to traverse.
#[derive(Component)]
pub struct MovementBuffer {
    pub buffer: Vec<HexPos>,
}

impl MovementBuffer {
    pub fn new(buffer: Vec<HexPos>) -> Self {
        MovementBuffer { buffer }
    }
}

/// Flag that gives one the ability to indicate whether or not a movable entity
/// is capable of moving at any given time.
#[derive(Component)]
pub struct IsMovable {
    pub truthful: bool,
}

impl IsMovable {
    pub fn new() -> Self {
        IsMovable { truthful: true }
    }
    pub fn toggle(&mut self) {
        self.truthful = !self.truthful;
    }
}

/// Flag that gives one the ability to indicate whether or not a traversable
/// entity is capable of being traversed at any given time.
#[derive(Component)]
pub struct IsTraversable {
    pub truthful: bool,
}

impl IsTraversable {
    pub fn new() -> Self {
        IsTraversable { truthful: true }
    }
    pub fn toggle(&mut self) {
        self.truthful = !self.truthful;
    }
}
