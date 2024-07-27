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

#[derive(Component, Resource)]
pub struct MapParameters {
    pub width: u32,
    pub height: u32,
    pub seed: u32,
    pub elevation_parameters: ElevationParameters,
    pub latitude_parameters: LatitudeParameters,
    pub terrain_spawn_parameters: TerrainSpawnParameters,
    pub convolution_parameters: ConvolutionParameters,
}

impl MapParameters {
    pub fn new() -> Self {
        MapParameters {
            width: 0,
            height: 0,
            seed: 0,
            elevation_parameters: ElevationParameters::new(),
            latitude_parameters: LatitudeParameters::new(),
            terrain_spawn_parameters: TerrainSpawnParameters::new(),
            convolution_parameters: ConvolutionParameters::new(),
        }
    }
}

pub struct ConvolutionParameters {
    pub terrain_convolutions: u32,
}

impl ConvolutionParameters {
    pub fn new() -> Self {
        ConvolutionParameters {
            terrain_convolutions: 0,
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
    pub fn new() -> Self {
        // Return new elevation parameters.
        ElevationParameters {
            noise_requests: Vec::new(),
            ocean_threshold: 0.0,
            coastal_threshold: 0.0,
            land_threshold: 0.0,
        }
    }
}

/// TODO: document.
pub struct LatitudeParameters {
    pub sigma: f32,
    pub temperature: f32,
}

impl LatitudeParameters {
    pub fn new() -> Self {
        LatitudeParameters {
            sigma: 0.0,
            temperature: 0.0,
        }
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

pub struct TerrainSpawnParameters {
    pub desert_bias: f32,
    pub grassland_bias: f32,
    pub ice_bias: f32,
    pub snow_bias: f32,
    pub steppe_bias: f32,
    pub tundra_bias: f32,
}

impl TerrainSpawnParameters {
    pub fn new() -> Self {
        TerrainSpawnParameters {
            desert_bias: 0.0,
            grassland_bias: 0.0,
            ice_bias: 0.0,
            snow_bias: 0.0,
            steppe_bias: 0.0,
            tundra_bias: 0.0,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum NoiseType {
    Simplex,
    Worley,
}
