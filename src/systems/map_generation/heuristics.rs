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
use crate::resources::map_resources::MapDimensions;

// TODO: variable icecap intensity. Cleanup ugliness.
pub fn adjust_for_latitude(dims: Res<MapDimensions>, mut query: Query<(&HexPos, &mut WaveFunc)>) {
    // Divide the latitudes into special regions.
    let dist = dims.height as f32 / 9.0;
    let northern_icecaps = dist;
    let northern_snowsheets = 2.0 * dist;
    let northern_tundra = 3.0 * dist;
    let northern_diverse = 4.0 * dist;
    let equatorial = 5.0 * dist;
    let southern_diverse = 6.0 * dist;
    let southern_tundra = 7.0 * dist;
    let southern_snowsheets = 8.0 * dist;

    for (pos, mut wave_function) in &mut query {
        let latitude = pos.r;

        // Adjust weights for the northern icecaps region.
        if latitude < northern_icecaps {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }

        // Adjust weights for the northern snowsheets region.
        else if latitude < northern_snowsheets {
            // TODO: replace icecap tile with snow.
        }

        // Adjust weights for the northern tundra region.
        else if latitude < northern_tundra {
            // TODO: implement.
        }

        // Adjust weights for the northern diverse region.
        else if latitude < northern_diverse {
            // TODO: implement
        }

        // Adjust weights for the equatorial region.
        else if latitude < equatorial {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/desertTile.glb#Scene0" {
                    pair.1 += 0.90;
                } else {
                    pair.1 -= 0.15;
                }
            }
        }

        // Adjust weights for the southern diverse region.
        else if latitude < southern_diverse {
            // Don't do anything for now!
        }

        // Adjust weights for the southern tundra region.
        else if latitude < southern_tundra {
            // TODO: implement.
        }

        // Adjust weights for the southern snowsheets region.
        else if latitude < southern_snowsheets {
            // TODO: implement.
        }

        // Adjust weights for the southern icecaps region.
        else {
            for pair in wave_function.domain.iter_mut() {
                if pair.0 == "tiles/snowTile.glb#Scene0" {
                    pair.1 = 1.0;
                } else {
                    pair.1 = 0.0;
                }
            }
        }
    }
}
