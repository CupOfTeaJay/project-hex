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
    // Map dimensions.
    pub width: i32,
    pub height: i32,
    // Biases for tiles based on latitudes.
    pub diverse_grassland_bias: f32,
    pub diverse_steppe_bias: f32,
    pub equator_desert_bias: f32,
    pub tundra_snow_bias: f32,
    pub tundra_tundra_bias: f32,
    // Biases for neighboring tiles based on a COASTAL tile spawn.
    pub spawn_coastal_coastal_bias: f32,
    pub spawn_coastal_desert_bias: f32,
    pub spawn_coastal_grassland_bias: f32,
    pub spawn_coastal_ice_bias: f32,
    pub spawn_coastal_ocean_bias: f32,
    pub spawn_coastal_snow_bias: f32,
    pub spawn_coastal_steppe_bias: f32,
    pub spawn_coastal_tundra_bias: f32,
    // Biases for neighboring tiles based on a DESERT tile spawn.
    pub spawn_desert_coastal_bias: f32,
    pub spawn_desert_desert_bias: f32,
    pub spawn_desert_grassland_bias: f32,
    pub spawn_desert_steppe_bias: f32,
    // Biases for neighboring tiles based on a GRASSLAND tile spawn.
    pub spawn_grassland_coastal_bias: f32,
    pub spawn_grassland_desert_bias: f32,
    pub spawn_grassland_grassland_bias: f32,
    pub spawn_grassland_steppe_bias: f32,
    pub spawn_grassland_tundra_bias: f32,
    // Biases for neighboring tiles based on an ICE tile spawn.
    pub spawn_ice_coastal_bias: f32,
    pub spawn_ice_ice_bias: f32,
    pub spawn_ice_ocean_bias: f32,
    pub spawn_ice_snow_bias: f32,
    // Biases for neighboring tiles based on an OCEAN tile spawn.
    pub spawn_ocean_coastal_bias: f32,
    pub spawn_ocean_ice_bias: f32,
    pub spawn_ocean_ocean_bias: f32,
    // Biases for neighboring tiles based on an SNOW tile spawn.
    pub spawn_snow_coastal_bias: f32,
    pub spawn_snow_ice_bias: f32,
    pub spawn_snow_snow_bias: f32,
    pub spawn_snow_tundra_bias: f32,
    // Biases for neighboring tiles based on an STEPPE tile spawn.
    pub spawn_steppe_coastal_bias: f32,
    pub spawn_steppe_desert_bias: f32,
    pub spawn_steppe_grassland_bias: f32,
    pub spawn_steppe_steppe_bias: f32,
    pub spawn_steppe_tundra_bias: f32,
    // Biases for neighboring tiles based on an TUNDRA tile spawn.
    pub spawn_tundra_coastal_bias: f32,
    pub spawn_tundra_grassland_bias: f32,
    pub spawn_tundra_snow_bias: f32,
    pub spawn_tundra_steppe_bias: f32,
    pub spawn_tundra_tundra_bias: f32,
    // Encroachment limit of icecap and snow regions.
    pub icecap_limit: f32,
    pub snow_limit: f32,
}

impl MapParameters {
    pub fn new(
        // Map dimensions.
        width: i32,
        height: i32,
        // Biases for tiles based on latitudes.
        diverse_grassland_bias: f32,
        diverse_steppe_bias: f32,
        equator_desert_bias: f32,
        tundra_snow_bias: f32,
        tundra_tundra_bias: f32,
        // Biases for neighboring tiles based on a COASTAL tile spawn.
        spawn_coastal_coastal_bias: f32,
        spawn_coastal_desert_bias: f32,
        spawn_coastal_grassland_bias: f32,
        spawn_coastal_ice_bias: f32,
        spawn_coastal_ocean_bias: f32,
        spawn_coastal_snow_bias: f32,
        spawn_coastal_steppe_bias: f32,
        spawn_coastal_tundra_bias: f32,
        // Biases for neighboring tiles based on a DESERT tile spawn.
        spawn_desert_coastal_bias: f32,
        spawn_desert_desert_bias: f32,
        spawn_desert_grassland_bias: f32,
        spawn_desert_steppe_bias: f32,
        // Biases for neighboring tiles based on a GRASSLAND tile spawn.
        spawn_grassland_coastal_bias: f32,
        spawn_grassland_desert_bias: f32,
        spawn_grassland_grassland_bias: f32,
        spawn_grassland_steppe_bias: f32,
        spawn_grassland_tundra_bias: f32,
        // Biases for neighboring tiles based on an ICE tile spawn.
        spawn_ice_coastal_bias: f32,
        spawn_ice_ice_bias: f32,
        spawn_ice_ocean_bias: f32,
        spawn_ice_snow_bias: f32,
        // Biases for neighboring tiles based on an OCEAN tile spawn.
        spawn_ocean_coastal_bias: f32,
        spawn_ocean_ice_bias: f32,
        spawn_ocean_ocean_bias: f32,
        // Biases for neighboring tiles based on an SNOW tile spawn.
        spawn_snow_coastal_bias: f32,
        spawn_snow_ice_bias: f32,
        spawn_snow_snow_bias: f32,
        spawn_snow_tundra_bias: f32,
        // Biases for neighboring tiles based on an STEPPE tile spawn.
        spawn_steppe_coastal_bias: f32,
        spawn_steppe_desert_bias: f32,
        spawn_steppe_grassland_bias: f32,
        spawn_steppe_steppe_bias: f32,
        spawn_steppe_tundra_bias: f32,
        // Biases for neighboring tiles based on an TUNDRA tile spawn.
        spawn_tundra_coastal_bias: f32,
        spawn_tundra_grassland_bias: f32,
        spawn_tundra_snow_bias: f32,
        spawn_tundra_steppe_bias: f32,
        spawn_tundra_tundra_bias: f32,
        // Encroachment limit of icecap and snow regions.
        icecap_limit: f32,
        snow_limit: f32,
    ) -> Self {
        MapParameters {
            // Map dimensions.
            width,
            height,
            // Biases for tiles based on latitudes.
            diverse_grassland_bias,
            diverse_steppe_bias,
            equator_desert_bias,
            tundra_snow_bias,
            tundra_tundra_bias,
            // Biases for neighboring tiles based on a COASTAL tile spawn.
            spawn_coastal_coastal_bias,
            spawn_coastal_desert_bias,
            spawn_coastal_grassland_bias,
            spawn_coastal_ice_bias,
            spawn_coastal_ocean_bias,
            spawn_coastal_snow_bias,
            spawn_coastal_steppe_bias,
            spawn_coastal_tundra_bias,
            // Biases for neighboring tiles based on a DESERT tile spawn.
            spawn_desert_coastal_bias,
            spawn_desert_desert_bias,
            spawn_desert_grassland_bias,
            spawn_desert_steppe_bias,
            // Biases for neighboring tiles based on a GRASSLAND tile spawn.
            spawn_grassland_coastal_bias,
            spawn_grassland_desert_bias,
            spawn_grassland_grassland_bias,
            spawn_grassland_steppe_bias,
            spawn_grassland_tundra_bias,
            // Biases for neighboring tiles based on an ICE tile spawn.
            spawn_ice_coastal_bias,
            spawn_ice_ice_bias,
            spawn_ice_ocean_bias,
            spawn_ice_snow_bias,
            // Biases for neighboring tiles based on an OCEAN tile spawn.
            spawn_ocean_coastal_bias,
            spawn_ocean_ice_bias,
            spawn_ocean_ocean_bias,
            // Biases for neighboring tiles based on an SNOW tile spawn.
            spawn_snow_coastal_bias,
            spawn_snow_ice_bias,
            spawn_snow_snow_bias,
            spawn_snow_tundra_bias,
            // Biases for neighboring tiles based on an STEPPE tile spawn.
            spawn_steppe_coastal_bias,
            spawn_steppe_desert_bias,
            spawn_steppe_grassland_bias,
            spawn_steppe_steppe_bias,
            spawn_steppe_tundra_bias,
            // Biases for neighboring tiles based on an TUNDRA tile spawn.
            spawn_tundra_coastal_bias,
            spawn_tundra_grassland_bias,
            spawn_tundra_snow_bias,
            spawn_tundra_steppe_bias,
            spawn_tundra_tundra_bias,
            // Encroachment limit of icecap and snow regions.
            icecap_limit,
            snow_limit,
        }
    }
}
