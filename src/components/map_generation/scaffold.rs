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
use std::f32::consts::FRAC_PI_2;

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::wave_func::WaveFunc;
use crate::utils::coord_conversions::hex_pos_to_vec3;

/// Tile "scaffolding" to be used for generating maps. Should be removed from
/// the world upon completion of the algorithm.
#[derive(Bundle)]
pub struct Scaffold {
    pos: HexPos,
    transform: Transform,
    wave_func: WaveFunc,
}

// TODO: init scaffold with non-default quaternion instead.
impl Scaffold {
    /// Creates tile scaffolding.
    pub fn new(pos: HexPos) -> Self {
        // Convert from cube coordinates to cartesian coordinates.
        let (x, y, z) = hex_pos_to_vec3(pos.q, pos.r, pos.s);

        // Tile assets are flat side up, so first init a transform...
        let mut transform = Transform {
            translation: Vec3::new(x, y, z),
            ..Default::default()
        };

        // ...then rotate it by 90 degrees.
        transform.rotate_y(FRAC_PI_2);

        // Return the scaffold.
        Scaffold {
            pos: pos,
            transform: transform,
            wave_func: WaveFunc::new(),
        }
    }
}

// TODO: test Scaffold::new()
