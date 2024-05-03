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
use rand::Rng;
use std::f32::consts::FRAC_PI_2;
use std::vec::Vec;

use crate::components::common::hex_pos::HexPos;
use crate::resources::map_parameters::MapParameters;
use crate::utils::coord_conversions::cube_to_cartesian;

/// Tile "scaffolding" to be used for generating maps. Should be removed from
/// the world upon completion of the algorithm.
pub struct Scaffold {
    pub pos: HexPos,
    pub transform: Transform,
    pub wave_func: WaveFunction,
}

// TODO: init scaffold with non-default quaternion instead.
impl Scaffold {
    /// Creates tile scaffolding.
    pub fn new(pos: HexPos) -> Self {
        // Convert from cube coordinates to cartesian coordinates.
        let (x, y, z) = cube_to_cartesian(pos.q, pos.r, pos.s);

        // Create a new transform and rotate it.
        let mut transform = Transform::from_xyz(x, y, z);
        transform.rotate_y(FRAC_PI_2);

        // Return the scaffold.
        Scaffold {
            pos: pos,
            transform: transform,
            wave_func: WaveFunction::new(),
        }
    }
}

pub struct WaveFunction {
    pub domain: IndexMap<String, f32>,
}

impl WaveFunction {
    pub fn new() -> Self {
        // Update as necessary.
        let num_possibilities: f32 = 9.0;

        // Build the map.
        let mut domain = IndexMap::new();
        let uniform_prob: f32 = 100.0 / num_possibilities;
        domain.insert("tiles/coastalTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/desertTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/grasslandTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/iceTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/jungleTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/oceanTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/snowTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/steppeTile.glb#Scene0".to_string(), uniform_prob);
        domain.insert("tiles/tundraTile.glb#Scene0".to_string(), uniform_prob);

        // Return self.
        WaveFunction { domain }
    }

    pub fn collapse(&self) -> &String {
        // Calculate prefix sums.
        let mut prefix_sums: Vec<f32> = Vec::new();
        let mut curr_sum: f32 = 0.0;
        for weight in self.domain.values() {
            curr_sum += weight;
            prefix_sums.push(curr_sum);
        }

        // Generate a random number in the range [0, 1).
        let random_number: f32 = rand::thread_rng().gen_range(0.0..curr_sum);

        // Binary search for this random number in our vector of prefix sums.
    }
}

pub fn generate_map_data(
    map_par: &Res<MapParameters>,
) -> (
    IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    IndexMap<(i32, i32, i32), Scaffold>,
) {
    // Generate scaffolding.
    let (pos_neighbor_map, mut pos_scaffold_map) =
        generate_scaffolding(&map_par.width, &map_par.height);

    // Bias scaffolding based on latitude.
    adjust_for_latitude(map_par, &mut pos_scaffold_map);

    // Return scaffolding for ECS to process.
    (pos_neighbor_map, pos_scaffold_map)
}

// TODO: since this is decoupled from ECS, split into smaller functions. Maybe resource issue?
/// Adjust weights for every scaffold, biasing tiles based on their latitude.
fn adjust_for_latitude(
    map_par: &Res<MapParameters>,
    pos_scaffold_map: &mut IndexMap<(i32, i32, i32), Scaffold>,
) {
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

    for scaffold in pos_scaffold_map.values_mut() {
        let latitude: f32 = scaffold.pos.r;

        // Adjust weights for the northern icecaps region.
        if latitude < map_par.icecap_limit {
            bias_icecaps(scaffold)
        }
        // Adjust weights for the northern snowsheets region.
        else if latitude < map_par.icecap_limit + map_par.snow_limit {
            bias_snowsheets(scaffold)
        }
        // Adjust weights for the northern tundra region.
        else if latitude < north_tund {
            bias_tundra(map_par, scaffold)
        }
        // Adjust weights for the northern diverse region.
        else if latitude < north_dive {
            bias_diverse(map_par, scaffold)
        }
        // Adjust weights for the equatorial region.
        else if latitude < equatorial {
            bias_equator(map_par, scaffold)
        }
        // Adjust weights for the southern diverse region.
        else if latitude < south_dive {
            bias_diverse(map_par, scaffold)
        }
        // Adjust weights for the southern tundra region.
        else if latitude < south_tund - (map_par.icecap_limit + map_par.snow_limit) {
            bias_tundra(map_par, scaffold)
        }
        // Adjust weights for the southern snowsheets region.
        else if latitude < south_tund - map_par.icecap_limit {
            bias_snowsheets(scaffold)
        }
        // Adjust weights for the southern icecaps region.
        else {
            bias_icecaps(scaffold)
        }
    }
}

fn bias_diverse(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    let domain_size = scaffold.wave_func.domain.len();
    for (possibility, weight) in scaffold.wave_func.domain.iter_mut() {
        if possibility == "tiles/grasslandTile.glb#Scene0" {
            *weight += map_par.diverse_grassland_bias;
        } else if possibility == "tiles/steppeTile.glb#Scene0" {
            *weight += map_par.diverse_steppe_bias;
        } else {
            *weight -= map_par.diverse_bias_sum / domain_size as f32
        }
    }
}

fn bias_equator(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    for pair in scaffold.wave_func.domain.iter_mut() {
        if possibility == "tiles/desertTile.glb#Scene0" {
            weight += map_par.equator_desert_bias;
        } else if possibility == "tiles/jungleTile.glb#Scene0" {
            weight += map_par.equator_jungle_bias;
        } else {
            weight -= map_par.equator_bias_sum / DOMAIN_SIZE as f32
        }
    }
}

fn bias_icecaps(scaffold: &mut Scaffold) {
    for pair in scaffold.wave_func.domain.iter_mut() {
        if possibility == "tiles/iceTile.glb#Scene0" {
            weight = 1.0;
        } else {
            weight = 0.0;
        }
    }
}

fn bias_snowsheets(scaffold: &mut Scaffold) {
    for pair in scaffold.wave_func.domain.iter_mut() {
        if possibility == "tiles/snowTile.glb#Scene0" {
            weight = 1.0;
        } else {
            weight = 0.0;
        }
    }
}

fn bias_tundra(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    for pair in scaffold.wave_func.domain.iter_mut() {
        if possibility == "tiles/snowTile.glb#Scene0" {
            weight += map_par.tundra_snow_bias;
        } else if possibility == "tiles/tundraTile.glb#Scene0" {
            weight += map_par.tundra_tundra_bias;
        } else {
            weight -= map_par.tundra_bias_sum / DOMAIN_SIZE as f32
        }
    }
}

/// Given some position, determines its neighboring positions.
fn determine_neighbors(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (pos.0 + 1, pos.1 - 1, pos.2), // Northeastern neighbor.
        (pos.0 + 1, pos.1, pos.2 - 1), // Eastern neighbor.
        (pos.0, pos.1 + 1, pos.2 - 1), // Southeastern neighbor.
        (pos.0 - 1, pos.1 + 1, pos.2), // Southwestern neighbor.
        (pos.0 - 1, pos.1, pos.2 + 1), // Western neighbor.
        (pos.0, pos.1 - 1, pos.2 + 1), // Northwestern neighbor.
    ]
}

/// Generates two hash tables - one maps position to scaffolding, the other position to neighboring
/// positions.
fn generate_scaffolding(
    width: &i32,
    height: &i32,
) -> (
    IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
    IndexMap<(i32, i32, i32), Scaffold>,
) {
    // We need two maps here. One that maps a position to neighboring positions...
    let mut pos_neighbor_map: IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = IndexMap::new();

    // ...and another that maps a position to a scaffold.
    let mut pos_scaffold_map: IndexMap<(i32, i32, i32), Scaffold> = IndexMap::new();

    // Vars to iteratively update.
    let mut curr_pos: HexPos = HexPos::new(0.0, 0.0, 0.0);
    let mut q_min: i32 = 0;
    let mut q_max: i32 = *width;

    // For every possible position, as defined by the map width and height.
    for r in 0..*height {
        curr_pos.r = r as f32;
        if r % 2 == 0 && r != 0 {
            q_min -= 1;
            q_max -= 1;
        }
        for q in q_min..q_max {
            curr_pos.q = q as f32;
            curr_pos.s = (-q - r) as f32;

            // Unsigned integer representation of curr_pos.
            let int_rep: (i32, i32, i32) =
                (curr_pos.q as i32, curr_pos.r as i32, curr_pos.s as i32);

            // Insert data into pos_neighbor_map.
            pos_neighbor_map.insert(int_rep, determine_neighbors(int_rep));

            // Insert data into pos_scaffold_map.
            pos_scaffold_map.insert(int_rep, Scaffold::new(curr_pos));
        }
    }

    // Return the hash maps.
    (pos_neighbor_map, pos_scaffold_map)
}
