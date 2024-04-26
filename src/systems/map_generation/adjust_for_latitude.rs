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

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::wave_func::WaveFunc;
use crate::resources::map_parameters::MapParameters;

// TODO: make this as clean as possible.
pub fn adjust_for_latitude(map: Res<MapParameters>, mut query: Query<(&HexPos, &mut WaveFunc)>) {
    // Constants.
    let num_regions: f32 = 5.0;
    let map_height: f32 = map.height as f32;
    let region_width: f32 = map_height / num_regions;

    // Region labels.
    let north_tund: f32 = 1.0 * region_width; // Northern hemisphere tundra.
    let north_dive: f32 = 2.0 * region_width; // Northern hemisphere diverse.
    let equatorial: f32 = 3.0 * region_width; // Equatorial.
    let south_dive: f32 = 4.0 * region_width; // Southern hemisphere diverse.
    let south_tund: f32 = 5.0 * region_width; // Southern hemisphere tundra.

    for (pos, mut wave_function) in &mut query {
        let latitude: f32 = pos.r;
        let dom_size: f32 = wave_function.domain.len() as f32;

        // Adjust weights for the northern icecaps region.
        if latitude < map.icecap_limit {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/iceTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }
        // Adjust weights for the northern snowsheets region.
        else if latitude < map.icecap_limit + map.snow_limit {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }
        // Adjust weights for the northern tundra region.
        else if latitude < north_tund {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 += map.tundra_snow_bias;
                } else if pair.0 == "tiles/tundraTile.glb#Scene0" {
                    pair.1 += map.tundra_tundra_bias;
                } else {
                    pair.1 -= map.tundra_bias_sum / dom_size
                }
            }
        }
        // Adjust weights for the northern diverse region.
        else if latitude < north_dive {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/grasslandTile.glb#Scene0" {
                    pair.1 += map.diverse_grassland_bias;
                } else if pair.0 == "tiles/steppeTile.glb#Scene0" {
                    pair.1 += map.diverse_steppe_bias;
                } else {
                    pair.1 -= map.diverse_bias_sum / dom_size
                }
            }
        }
        // Adjust weights for the equatorial region.
        else if latitude < equatorial {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/desertTile.glb#Scene0" {
                    pair.1 += map.equator_desert_bias;
                } else if pair.0 == "tiles/jungleTile.glb#Scene0" {
                    pair.1 += map.equator_jungle_bias;
                } else {
                    pair.1 -= map.equator_bias_sum / dom_size
                }
            }
        }
        // Adjust weights for the southern diverse region.
        else if latitude < south_dive {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/grasslandTile.glb#Scene0" {
                    pair.1 += map.diverse_grassland_bias;
                } else if pair.0 == "tiles/steppeTile.glb#Scene0" {
                    pair.1 += map.diverse_steppe_bias;
                } else {
                    pair.1 -= map.diverse_bias_sum / dom_size
                }
            }
        }
        // Adjust weights for the southern tundra region.
        else if latitude < south_tund - (map.icecap_limit + map.snow_limit) {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 += map.tundra_snow_bias;
                } else if pair.0 == "tiles/tundraTile.glb#Scene0" {
                    pair.1 += map.tundra_tundra_bias;
                } else {
                    pair.1 -= map.tundra_bias_sum / dom_size
                }
            }
        }
        // Adjust weights for the southern snowsheets region.
        else if latitude < south_tund - map.icecap_limit {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }
        // Adjust weights for the southern icecaps region.
        else {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/iceTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }
    }
}
