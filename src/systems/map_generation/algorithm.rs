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
use rand::prelude::*;
use std::vec::Vec;

use crate::components::common::hex_pos::HexPos;
use crate::resources::map_parameters::MapParameters;
use crate::utils::coord_conversions::hex_pos_to_vec3;

const DOMAIN_SIZE: usize = 9;
const UNIFORM_PROB: f32 = 1.0 / (DOMAIN_SIZE as f32);

/// Tile "scaffolding" to be used for generating maps. Should be removed from
/// the world upon completion of the algorithm.
pub struct Scaffold {
    pub pos: HexPos,
    transform: Transform,
    pub wave_func: WaveFunction,
}

// TODO: init scaffold with non-default quaternion instead.
impl Scaffold {
    /// Creates tile scaffolding.
    pub fn new(pos: HexPos) -> Self {
        // Convert from cube coordinates to cartesian coordinates.
        let (x, y, z) = hex_pos_to_vec3(pos.q, pos.r, pos.s);

        // Return the scaffold.
        Scaffold {
            pos: pos,
            transform: Transform::from_xyz(x, y, z),
            wave_func: WaveFunction::new(),
        }
    }
}

pub struct WaveFunction {
    pub domain: [(String, f32); DOMAIN_SIZE],
}

impl WaveFunction {
    pub fn new() -> Self {
        WaveFunction {
            domain: [
                ("tiles/coastalTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/desertTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/grasslandTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/iceTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/jungleTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/oceanTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/snowTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/steppeTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/tundraTile.glb#Scene0".to_string(), UNIFORM_PROB),
            ],
        }
    }

    pub fn collapse(&self) -> String {
        let mut rng: ThreadRng = thread_rng();
        self
            .domain
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
            .clone()
    }
}

pub fn run_algorithm(map_par: Res<MapParameters>) -> Vec<Vec<Scaffold>> {
    let mut scaffolding = generate_scaffolding(map_par.width, map_par.height);
    adjust_for_latitude(map_par, &mut scaffolding);
    scaffolding
}

fn generate_scaffolding(width: i32, height: i32) -> Vec<Vec<Scaffold>> {
    // Return vector.
    let mut scaffolding: Vec<Vec<Scaffold>> = Vec::new();

    // Vars to iteratively update.
    let mut curr_pos: HexPos = HexPos::new(0.0, 0.0, 0.0);
    let mut q_min: i32 = 0;
    let mut q_max: i32 = width;

    // Spawn the scaffolding.
    let mut i: usize = 0;
    for r in 0..height {
        curr_pos.r = r as f32;
        if r % 2 == 0 && r != 0 {
            q_min -= 1;
            q_max -= 1;
        }
        scaffolding.push(Vec::new());
        for q in q_min..q_max {
            curr_pos.q = q as f32;
            curr_pos.s = (-q - r) as f32;
            scaffolding[i].push(Scaffold::new(curr_pos));
        }
        i += 1;
    }

    scaffolding
}

fn adjust_for_latitude(map_par: Res<MapParameters>, scaffolding: &mut Vec<Vec<Scaffold>>) {
    // Constants.
    let num_regions: f32 = 5.0;
    let map_height: f32 = map_par.height as f32;
    let region_width: f32 = map_height / num_regions;

    // Region labels.
    let north_tund: f32 = 1.0 * region_width; // Northern hemisphere tundra.
    let north_dive: f32 = 2.0 * region_width; // Northern hemisphere diverse.
    let equatorial: f32 = 3.0 * region_width; // Equatorial.
    let south_dive: f32 = 4.0 * region_width; // Southern hemisphere diverse.
    let south_tund: f32 = 5.0 * region_width; // Southern hemisphere tundra.

    for vec in scaffolding.iter_mut() {
        for scaffold in vec.iter_mut() {
            let latitude: f32 = scaffold.pos.r;
            let dom_size: f32 = scaffold.wave_func.domain.len() as f32;

            // Adjust weights for the northern icecaps region.
            if latitude < map_par.icecap_limit {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/iceTile.glb#Scene0" {
                        pair.1 = 1.0;
                    } else {
                        pair.1 = 0.0;
                    }
                }
            }
            // Adjust weights for the northern snowsheets region.
            else if latitude < map_par.icecap_limit + map_par.snow_limit {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/snowTile.glb#Scene0" {
                        pair.1 = 1.0;
                    } else {
                        pair.1 = 0.0;
                    }
                }
            }
            // Adjust weights for the northern tundra region.
            else if latitude < north_tund {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/snowTile.glb#Scene0" {
                        pair.1 += map_par.tundra_snow_bias;
                    } else if pair.0 == "tiles/tundraTile.glb#Scene0" {
                        pair.1 += map_par.tundra_tundra_bias;
                    } else {
                        pair.1 -= map_par.tundra_bias_sum / dom_size
                    }
                }
            }
            // Adjust weights for the northern diverse region.
            else if latitude < north_dive {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/grasslandTile.glb#Scene0" {
                        pair.1 += map_par.diverse_grassland_bias;
                    } else if pair.0 == "tiles/steppeTile.glb#Scene0" {
                        pair.1 += map_par.diverse_steppe_bias;
                    } else {
                        pair.1 -= map_par.diverse_bias_sum / dom_size
                    }
                }
            }
            // Adjust weights for the equatorial region.
            else if latitude < equatorial {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/desertTile.glb#Scene0" {
                        pair.1 += map_par.equator_desert_bias;
                    } else if pair.0 == "tiles/jungleTile.glb#Scene0" {
                        pair.1 += map_par.equator_jungle_bias;
                    } else {
                        pair.1 -= map_par.equator_bias_sum / dom_size
                    }
                }
            }
            // Adjust weights for the southern diverse region.
            else if latitude < south_dive {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/grasslandTile.glb#Scene0" {
                        pair.1 += map_par.diverse_grassland_bias;
                    } else if pair.0 == "tiles/steppeTile.glb#Scene0" {
                        pair.1 += map_par.diverse_steppe_bias;
                    } else {
                        pair.1 -= map_par.diverse_bias_sum / dom_size
                    }
                }
            }
            // Adjust weights for the southern tundra region.
            else if latitude < south_tund - (map_par.icecap_limit + map_par.snow_limit) {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/snowTile.glb#Scene0" {
                        pair.1 += map_par.tundra_snow_bias;
                    } else if pair.0 == "tiles/tundraTile.glb#Scene0" {
                        pair.1 += map_par.tundra_tundra_bias;
                    } else {
                        pair.1 -= map_par.tundra_bias_sum / dom_size
                    }
                }
            }
            // Adjust weights for the southern snowsheets region.
            else if latitude < south_tund - map_par.icecap_limit {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/snowTile.glb#Scene0" {
                        pair.1 = 1.0;
                    } else {
                        pair.1 = 0.0;
                    }
                }
            }
            // Adjust weights for the southern icecaps region.
            else {
                for pair in scaffold.wave_func.domain.iter_mut() {
                    if pair.0 == "tiles/iceTile.glb#Scene0" {
                        pair.1 = 1.0;
                    } else {
                        pair.1 = 0.0;
                    }
                }
            }
        }
    }
}
