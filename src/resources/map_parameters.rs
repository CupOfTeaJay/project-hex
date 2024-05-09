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

use bevy::{prelude::*, utils::hashbrown::Equivalent};

/// Parameters that define the map's size.
pub struct DimensionParameters {
    pub width: i32,
    pub height: i32,
}

impl DimensionParameters {
    pub fn new(width: i32, height: i32) -> Self {
        DimensionParameters { width, height }
    }
}

/// Parameters that bias terrain spawns at certain latitudes.
pub struct LatitudeParameters {
    // Biases for tiles at DIVERSE latitudes.
    pub diverse_coastal_bias: f32,
    pub diverse_desert_bias: f32,
    pub diverse_grassland_bias: f32,
    pub diverse_ocean_bias: f32,
    pub diverse_steppe_bias: f32,
    // Biases for tiles at the EQUATOR.
    pub equator_coastal_bias: f32,
    pub equator_desert_bias: f32,
    pub equator_grassland_bias: f32,
    pub equator_ocean_bias: f32,
    pub equator_steppe_bias: f32,
    // Biases for tiles at TUNDRA latitudes.
    pub tundra_coastal_bias: f32,
    pub tundra_grassland_bias: f32,
    pub tundra_ocean_bias: f32,
    pub tundra_snow_bias: f32,
    pub tundra_steppe_bias: f32,
    pub tundra_tundra_bias: f32,
}

impl LatitudeParameters {
    pub fn new(
        // Biases for tiles at DIVERSE latitudes.
        diverse_coastal_bias: f32,
        diverse_desert_bias: f32,
        diverse_grassland_bias: f32,
        diverse_ocean_bias: f32,
        diverse_steppe_bias: f32,
        // Biases for tiles at the EQUATOR.
        equator_coastal_bias: f32,
        equator_desert_bias: f32,
        equator_grassland_bias: f32,
        equator_ocean_bias: f32,
        equator_steppe_bias: f32,
        // Biases for tiles at TUNDRA latitudes.
        tundra_coastal_bias: f32,
        tundra_grassland_bias: f32,
        tundra_ocean_bias: f32,
        tundra_snow_bias: f32,
        tundra_steppe_bias: f32,
        tundra_tundra_bias: f32,
    ) -> Self {
        // Diverse latitude parameters to validate.
        let sum_diverse_biases: f32 = diverse_coastal_bias
            + diverse_desert_bias
            + diverse_grassland_bias
            + diverse_ocean_bias
            + diverse_steppe_bias;

        // Equator latitude parameters to validate.
        let sum_equator_biases: f32 = equator_coastal_bias
            + equator_desert_bias
            + equator_grassland_bias
            + equator_ocean_bias
            + equator_steppe_bias;

        // Tundra latitude parameters to validate.
        let sum_tundra_biases: f32 = tundra_coastal_bias
            + tundra_grassland_bias
            + tundra_ocean_bias
            + tundra_snow_bias
            + tundra_steppe_bias
            + tundra_tundra_bias;

        // Ensure parameters adhere to constraints.
        if sum_diverse_biases >= 1.0 {
            panic!("\nLatitudeParameters Error: sum of DIVERSE biases is greater than one!\n")
        }
        if sum_equator_biases >= 1.0 {
            panic!("\nLatitudeParameters Error: sum of EQUATOR biases is greater than one!\n")
        }
        if sum_tundra_biases >= 1.0 {
            panic!("\nLatitudeParameters Error: sum of TUNDRA biases is greater than one!\n")
        }

        // Return new LatitudeParameters.
        LatitudeParameters {
            // Biases for tiles at DIVERSE latitudes.
            diverse_coastal_bias,
            diverse_desert_bias,
            diverse_grassland_bias,
            diverse_ocean_bias,
            diverse_steppe_bias,
            // Biases for tiles at the EQUATOR.
            equator_coastal_bias,
            equator_desert_bias,
            equator_grassland_bias,
            equator_ocean_bias,
            equator_steppe_bias,
            // Biases for tiles at TUNDRA latitudes.
            tundra_coastal_bias,
            tundra_grassland_bias,
            tundra_ocean_bias,
            tundra_snow_bias,
            tundra_steppe_bias,
            tundra_tundra_bias,
        }
    }
}

/// Paramaters that bias adjacent terrain spawns based on another spawn.
pub struct SpawnParameters {
    // Biases for neighboring tiles based on a COASTAL tile spawn.
    pub coastal_coastal_bias: f32,
    pub coastal_desert_bias: f32,
    pub coastal_grassland_bias: f32,
    pub coastal_ice_bias: f32,
    pub coastal_ocean_bias: f32,
    pub coastal_snow_bias: f32,
    pub coastal_steppe_bias: f32,
    pub coastal_tundra_bias: f32,
    // Biases for neighboring tiles based on a DESERT tile spawn.
    pub desert_coastal_bias: f32,
    pub desert_desert_bias: f32,
    pub desert_grassland_bias: f32,
    pub desert_steppe_bias: f32,
    // Biases for neighboring tiles based on a GRASSLAND tile spawn.
    pub grassland_coastal_bias: f32,
    pub grassland_desert_bias: f32,
    pub grassland_grassland_bias: f32,
    pub grassland_steppe_bias: f32,
    pub grassland_tundra_bias: f32,
    // Biases for neighboring tiles based on an ICE tile spawn.
    pub ice_coastal_bias: f32,
    pub ice_ice_bias: f32,
    pub ice_ocean_bias: f32,
    pub ice_snow_bias: f32,
    // Biases for neighboring tiles based on an OCEAN tile spawn.
    pub ocean_coastal_bias: f32,
    pub ocean_ice_bias: f32,
    pub ocean_ocean_bias: f32,
    // Biases for neighboring tiles based on an SNOW tile spawn.
    pub snow_coastal_bias: f32,
    pub snow_ice_bias: f32,
    pub snow_snow_bias: f32,
    pub snow_tundra_bias: f32,
    // Biases for neighboring tiles based on an STEPPE tile spawn.
    pub steppe_coastal_bias: f32,
    pub steppe_desert_bias: f32,
    pub steppe_grassland_bias: f32,
    pub steppe_steppe_bias: f32,
    pub steppe_tundra_bias: f32,
    // Biases for neighboring tiles based on an TUNDRA tile spawn.
    pub tundra_coastal_bias: f32,
    pub tundra_grassland_bias: f32,
    pub tundra_snow_bias: f32,
    pub tundra_steppe_bias: f32,
    pub tundra_tundra_bias: f32,
}

impl SpawnParameters {
    pub fn new(
        // Biases for neighboring tiles based on a COASTAL tile spawn.
        coastal_coastal_bias: f32,
        coastal_desert_bias: f32,
        coastal_grassland_bias: f32,
        coastal_ice_bias: f32,
        coastal_ocean_bias: f32,
        coastal_snow_bias: f32,
        coastal_steppe_bias: f32,
        coastal_tundra_bias: f32,
        // Biases for neighboring tiles based on a DESERT tile spawn.
        desert_coastal_bias: f32,
        desert_desert_bias: f32,
        desert_grassland_bias: f32,
        desert_steppe_bias: f32,
        // Biases for neighboring tiles based on a GRASSLAND tile spawn.
        grassland_coastal_bias: f32,
        grassland_desert_bias: f32,
        grassland_grassland_bias: f32,
        grassland_steppe_bias: f32,
        grassland_tundra_bias: f32,
        // Biases for neighboring tiles based on an ICE tile spawn.
        ice_coastal_bias: f32,
        ice_ice_bias: f32,
        ice_ocean_bias: f32,
        ice_snow_bias: f32,
        // Biases for neighboring tiles based on an OCEAN tile spawn.
        ocean_coastal_bias: f32,
        ocean_ice_bias: f32,
        ocean_ocean_bias: f32,
        // Biases for neighboring tiles based on an SNOW tile spawn.
        snow_coastal_bias: f32,
        snow_ice_bias: f32,
        snow_snow_bias: f32,
        snow_tundra_bias: f32,
        // Biases for neighboring tiles based on an STEPPE tile spawn.
        steppe_coastal_bias: f32,
        steppe_desert_bias: f32,
        steppe_grassland_bias: f32,
        steppe_steppe_bias: f32,
        steppe_tundra_bias: f32,
        // Biases for neighboring tiles based on an TUNDRA tile spawn.
        tundra_coastal_bias: f32,
        tundra_grassland_bias: f32,
        tundra_snow_bias: f32,
        tundra_steppe_bias: f32,
        tundra_tundra_bias: f32,
    ) -> Self {
        // Coastal spawn parameters to validate.
        let sum_coastal_biases: f32 = coastal_coastal_bias
            + coastal_desert_bias
            + coastal_grassland_bias
            + coastal_ice_bias
            + coastal_ocean_bias
            + coastal_snow_bias
            + coastal_steppe_bias
            + coastal_tundra_bias;

        // Desert spawn parameters to validate.
        let sum_desert_biases: f32 =
            desert_coastal_bias + desert_desert_bias + desert_grassland_bias + desert_steppe_bias;

        // Grassland spawn parameters to validate.
        let sum_grassland_biases: f32 = grassland_coastal_bias
            + grassland_desert_bias
            + grassland_grassland_bias
            + grassland_steppe_bias
            + grassland_tundra_bias;

        // Ice spawn parameters to validate.
        let sum_ice_biases: f32 = ice_coastal_bias + ice_ice_bias + ice_ocean_bias + ice_snow_bias;

        // Ocean spawn parameters to validate.
        let sum_ocean_biases: f32 = ocean_coastal_bias + ocean_ice_bias + ocean_ocean_bias;

        // Snow spawn parameters to validate.
        let sum_snow_biases: f32 =
            snow_coastal_bias + snow_ice_bias + snow_snow_bias + snow_tundra_bias;

        // Steppe spawn parameters to validate.
        let sum_steppe_biases: f32 = steppe_coastal_bias
            + steppe_desert_bias
            + steppe_grassland_bias
            + steppe_steppe_bias
            + steppe_tundra_bias;

        // Tundra spawn parameters to validate.
        let sum_tundra_biases: f32 = tundra_coastal_bias
            + tundra_grassland_bias
            + tundra_snow_bias
            + tundra_steppe_bias
            + tundra_tundra_bias;

        // Ensure parameters adhere to constraints.
        if sum_coastal_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of COASTAL biases is greater than one!\n")
        }
        if sum_desert_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of DESERT biases is greater than one!\n")
        }
        if sum_grassland_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of GRASSLAND biases is greater than one!\n")
        }
        if sum_ice_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of ICE biases is greater than one!\n")
        }
        if sum_ocean_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of OCEAN biases is greater than one!\n")
        }
        if sum_snow_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of SNOW biases is greater than one!\n")
        }
        if sum_steppe_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of STEPPE biases is greater than one!\n")
        }
        if sum_tundra_biases >= 1.0 {
            panic!("\nSpawnParameters Error: sum of TUNDRA biases is greater than one!\n")
        }

        SpawnParameters {
            // Biases for neighboring tiles based on a COASTAL tile spawn.
            coastal_coastal_bias,
            coastal_desert_bias,
            coastal_grassland_bias,
            coastal_ice_bias,
            coastal_ocean_bias,
            coastal_snow_bias,
            coastal_steppe_bias,
            coastal_tundra_bias,
            // Biases for neighboring tiles based on a DESERT tile spawn.
            desert_coastal_bias,
            desert_desert_bias,
            desert_grassland_bias,
            desert_steppe_bias,
            // Biases for neighboring tiles based on a GRASSLAND tile spawn.
            grassland_coastal_bias,
            grassland_desert_bias,
            grassland_grassland_bias,
            grassland_steppe_bias,
            grassland_tundra_bias,
            // Biases for neighboring tiles based on an ICE tile spawn.
            ice_coastal_bias,
            ice_ice_bias,
            ice_ocean_bias,
            ice_snow_bias,
            // Biases for neighboring tiles based on an OCEAN tile spawn.
            ocean_coastal_bias,
            ocean_ice_bias,
            ocean_ocean_bias,
            // Biases for neighboring tiles based on an SNOW tile spawn.
            snow_coastal_bias,
            snow_ice_bias,
            snow_snow_bias,
            snow_tundra_bias,
            // Biases for neighboring tiles based on an STEPPE tile spawn.
            steppe_coastal_bias,
            steppe_desert_bias,
            steppe_grassland_bias,
            steppe_steppe_bias,
            steppe_tundra_bias,
            // Biases for neighboring tiles based on an TUNDRA tile spawn.
            tundra_coastal_bias,
            tundra_grassland_bias,
            tundra_snow_bias,
            tundra_steppe_bias,
            tundra_tundra_bias,
        }
    }
}

/// Parameters that define the limits to which polar icecaps and snowsheets extend.
pub struct LimitParameters {
    pub icecap_limit: f32,
    pub snowsheet_limit: f32,
}

impl LimitParameters {
    pub fn new(icecap_limit: f32, snowsheet_limit: f32) -> Self {
        LimitParameters {
            icecap_limit,
            snowsheet_limit,
        }
    }
}

#[derive(Resource)]
pub struct MapParameters {
    pub dimensions: DimensionParameters,
    pub latitude_params: LatitudeParameters,
    pub spawn_params: SpawnParameters,
    pub limit_params: LimitParameters,
}

impl MapParameters {
    pub fn new(
        dimensions: DimensionParameters,
        latitude_params: LatitudeParameters,
        spawn_params: SpawnParameters,
        limit_params: LimitParameters,
    ) -> Self {
        MapParameters {
            dimensions,
            latitude_params,
            spawn_params,
            limit_params,
        }
    }
}
