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

#[derive(Resource)]
pub struct MapParameters {
    pub width: u32,
    pub height: u32,
    pub seed: u32,
    pub noise_parameters: Vec<NoiseRequest>,
}

impl MapParameters {
    pub fn new(width: u32, height: u32, seed: u32, noise_parameters: Vec<NoiseRequest>) -> Self {
        MapParameters {
            width,
            height,
            seed,
            noise_parameters,
        }
    }
}

pub struct NoiseRequest {
    pub request: (NoiseType, u32, f64, f64, f64),
}

impl NoiseRequest {
    pub fn new(
        noise_type: NoiseType,
        octaves: u32,
        scale: f64,
        persistence: f64,
        lacunarity: f64,
    ) -> Self {
        NoiseRequest {
            request: (noise_type, octaves, scale, persistence, lacunarity),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum NoiseType {
    Simplex,
}
