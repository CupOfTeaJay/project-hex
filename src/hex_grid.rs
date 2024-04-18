/*
    Such is Life
    Copyright (C) 2024 Clevermeld LLC

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

use crate::wave_function::WaveFunction;

#[derive(Bundle)]
pub struct HexBundle {
    pub transform: Transform,        // Absolute position / transform.
    pub grid_pos: HexPosition,       // "Position" relative to an artificial hex-grid.
    pub wave_function: WaveFunction, // Possible set of tiles this bundle may "collapse" to.
}

impl HexBundle {
    pub fn new(rank: u8, file: u8, pos: Vec3) -> Self {
        HexBundle {
            transform: Transform {
                translation: pos,
                ..Default::default()
            },
            grid_pos: HexPosition::new(rank, file),
            wave_function: WaveFunction::new(),
        }
    }
}

#[derive(Clone, Component, Copy)]
pub struct HexPosition {
    pub file: u8,
    pub rank: u8,
}

impl HexPosition {
    pub fn new(file: u8, rank: u8) -> Self {
        HexPosition { file, rank }
    }
}
