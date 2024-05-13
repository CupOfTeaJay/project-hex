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
    pub elevation_parameters: ElevationParameters,
    pub latitude_parameters: LatitudeParameters,
}

impl MapParameters {
    pub fn new(
        width: u32,
        height: u32,
        seed: u32,
        elevation_parameters: ElevationParameters,
        latitude_parameters: LatitudeParameters,
    ) -> Self {
        MapParameters {
            width,
            height,
            seed,
            elevation_parameters,
            latitude_parameters,
        }
    }
}

pub struct ElevationParameters {
    pub noise_requests: Vec<NoiseRequest>,
    pub ocean_threshold: f64,
    pub coastal_threshold: f64,
    pub land_threshold: f64,
}

impl ElevationParameters {
    pub fn new(
        noise_requests: Vec<NoiseRequest>,
        ocean_threshold: f64,
        coastal_threshold: f64,
        land_threshold: f64,
    ) -> Self {
        // Constraints.
        if ocean_threshold <= 0.0 {
            panic!("\nElevationParameters Error: ocean threshold must be greater than 0.0!\n")
        } else if ocean_threshold > coastal_threshold {
            panic!("\nElevationParameters Error: ocean threshold exceeds coastal threshold!\n")
        } else if coastal_threshold > land_threshold {
            panic!("\nElevationParameters Error: coastal threshold exceeds land threshold!\n")
        } else if land_threshold >= 1.0 {
            panic!("\nElevationParameters Error: land threshold must be greater than 1.0!\n")
        }

        // Return new elevation parameters.
        ElevationParameters {
            noise_requests,
            ocean_threshold,
            coastal_threshold,
            land_threshold,
        }
    }
}

/// TODO: document.
pub struct LatitudeParameters {
    pub sigma: f32,
    pub temperature: f32,
}

impl LatitudeParameters {
    pub fn new(sigma: f32, temperature: f32) -> Self {
        LatitudeParameters { sigma, temperature }
    }
}

pub struct NoiseRequest {
    pub params: (NoiseType, u32, f64, f64, f64),
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
            params: (noise_type, octaves, scale, persistence, lacunarity),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum NoiseType {
    Simplex,
    Worley,
}
