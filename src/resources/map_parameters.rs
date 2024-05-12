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
    pub desert_posbias: f32,
    pub desert_negbias: f32,
    pub grassland_negbias: f32,
    pub grassland_posbias: f32,
    pub ice_negbias: f32,
    pub ice_posbias: f32,
    pub ocean_negbias: f32,
    pub ocean_posbias: f32,
    pub snow_negbias: f32,
    pub snow_posbias: f32,
    pub steppe_negbias: f32,
    pub steppe_posbias: f32,
    pub tundra_negbias: f32,
    pub tundra_posbias: f32,
}

impl LatitudeParameters {
    pub fn new(
        sigma: f32,
        desert_posbias: f32,
        desert_negbias: f32,
        grassland_negbias: f32,
        grassland_posbias: f32,
        ice_negbias: f32,
        ice_posbias: f32,
        ocean_negbias: f32,
        ocean_posbias: f32,
        snow_negbias: f32,
        snow_posbias: f32,
        steppe_negbias: f32,
        steppe_posbias: f32,
        tundra_negbias: f32,
        tundra_posbias: f32,
    ) -> Self {
        LatitudeParameters {
            sigma,
            desert_posbias,
            desert_negbias,
            grassland_negbias,
            grassland_posbias,
            ice_negbias,
            ice_posbias,
            ocean_negbias,
            ocean_posbias,
            snow_negbias,
            snow_posbias,
            steppe_negbias,
            steppe_posbias,
            tundra_negbias,
            tundra_posbias,
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

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum NoiseType {
    Simplex,
    Worley,
}
