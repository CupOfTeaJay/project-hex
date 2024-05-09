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
use indexmap::IndexMap;
use noise::{NoiseFn, Perlin, Seedable};

use crate::utils::coord_conversions::cube_to_cartesian;

const PERLIN_SCALE_FACTOR: f32 = 0.05;

pub fn generate_noise(
    pos_neighbors_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    seed: &u32,
) -> IndexMap<(i32, i32, i32), f64> {
    // To return.
    let mut noise_map: IndexMap<(i32, i32, i32), f64> = IndexMap::new();

    let perlin = Perlin::new(*seed);

    let (mut x, mut y, mut z): (f32, f32, f32);
    let mut perlin_sample: f64;
    for pos in pos_neighbors_map.keys() {
        (x, y, z) = cube_to_cartesian(pos.0 as f32, pos.1 as f32, pos.2 as f32);
        perlin_sample = perlin.get([
            (x * PERLIN_SCALE_FACTOR) as f64,
            (z * PERLIN_SCALE_FACTOR) as f64,
        ]);
        noise_map.insert(*pos, perlin_sample);
    }

    // Return.
    noise_map
}
