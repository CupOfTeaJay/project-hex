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
use crate::components::wave_func::WaveFunc;

/// Cell "scaffolding" to be used for generating maps. Should be removed from
/// the world upon completion of the algorithm.
#[derive(Bundle)]
struct Scaffold {
    transform: Transform,
    pos: HexPos,
    wave_func: WaveFunc,
}

impl Scaffold {
    /// Creates cell scaffolding.
    fn new(q: u8, r: u8, s: u8) -> Self {
        Scaffold {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0 ,0.0), // TODO: convert cube-coord args to transform.
                ..Default::default()
            },
            pos: HexPos::new(q, r, s),
            wave_func: WaveFunc::new(),
        }
    }
}
