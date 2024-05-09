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
use crate::systems::map_generation::common::Terrain;
use crate::utils::coord_conversions::cube_to_cartesian;

/// Tile "scaffolding" to be used for generating maps. Should be removed from
/// the world upon completion of the algorithm.
pub struct Scaffold {
    pub pos: HexPos,
    pub transform: Transform,
    pub wave_func: WaveFunction,
    pub entropy: usize,
}

// TODO: init scaffold with non-default quaternion instead.
impl Scaffold {
    /// Creates new tile scaffolding.
    pub fn new(pos: HexPos) -> Self {
        // Convert from cube coordinates to cartesian coordinates.
        let (x, y, z) = cube_to_cartesian(pos.q, pos.r, pos.s);

        // Create a new transform and rotate it.
        let mut transform = Transform::from_xyz(x, y, z);
        transform.rotate_y(FRAC_PI_2);

        // Create a new wave function.
        let wave_func = WaveFunction::new();
        let entropy: usize = wave_func.domain.keys().len();

        // Return the scaffold.
        Scaffold {
            pos: pos,
            transform: transform,
            wave_func: wave_func,
            entropy: entropy,
        }
    }

    pub fn bias_tiles(&mut self, possibilities_to_bias: &Vec<(&Terrain, &f32)>) {
        for (possibility, bias) in possibilities_to_bias.iter() {
            if let Some(weight) = self.wave_func.domain.get_mut(*possibility) {
                *weight += *bias;
            }
        }
    }

    fn weight_divvy(&mut self, weight_to_divvy: &f32) {
        let domain_size: f32 = self.wave_func.domain.keys().len() as f32;
        let divvy: f32 = weight_to_divvy / domain_size;
        for weight in self.wave_func.domain.values_mut() {
            *weight += divvy;
        }
    }

    pub fn purge_tile(&mut self, possibility: &Terrain) -> Option<f32> {
        // Remove this possibility from our wave function.
        if let Some(weight) = self.wave_func.domain.swap_remove(possibility) {
            self.weight_divvy(&weight);
            self.entropy -= 1;
            Some(weight)
        } else {
            None
        }
    }
}

pub struct WaveFunction {
    pub domain: IndexMap<Terrain, f32>,
}

impl WaveFunction {
    pub fn new() -> Self {
        // UPDATE AS NECESSARY.
        let num_possibilities: f32 = 8.0;

        // Build the map.
        let mut domain = IndexMap::new();
        let uniform_prob: f32 = 1.0 / num_possibilities;
        domain.insert(Terrain::Coastal, uniform_prob);
        domain.insert(Terrain::Desert, uniform_prob);
        domain.insert(Terrain::Grassland, uniform_prob);
        domain.insert(Terrain::Ice, uniform_prob);
        domain.insert(Terrain::Ocean, uniform_prob);
        domain.insert(Terrain::Snow, uniform_prob);
        domain.insert(Terrain::Steppe, uniform_prob);
        domain.insert(Terrain::Tundra, uniform_prob);

        // Return self.
        WaveFunction { domain }
    }

    pub fn collapse(&self) -> &Terrain {
        // No possibilities left. Panic!
        // TODO: Identify case(s) where this happens, or just restart map gen instead of panicking.
        if self.domain.keys().len() == 0 {
            panic!("\nError: wave function domain size is zero\n")
        }

        // Pre-processing.
        let mut possibilities: Vec<&Terrain> = Vec::new();
        let mut weights_pref_sums: Vec<f32> = Vec::new();
        let mut curr_sum: f32 = 0.0;
        for (possibility, weight) in self.domain.iter() {
            possibilities.push(possibility);
            curr_sum += weight;
            weights_pref_sums.push(curr_sum.clone());
        }

        // Random sample.
        let mut choice_index: usize = 0;
        let rand_num: f32 = rand::thread_rng().gen_range(0.0..curr_sum);
        for i in 0..weights_pref_sums.len() {
            if rand_num < weights_pref_sums[i] {
                choice_index = i;
                break;
            }
        }

        possibilities[choice_index]
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
        generate_scaffolding(&map_par.dimensions.width, &map_par.dimensions.height);

    // Bias scaffolding based on latitude.
    adjust_for_latitude(map_par, &mut pos_scaffold_map);

    // Return scaffolding for ECS to process.
    (pos_neighbor_map, pos_scaffold_map)
}

/// Adjust weights for every scaffold, biasing tiles based on their latitude.
fn adjust_for_latitude(
    map_par: &Res<MapParameters>,
    pos_scaffold_map: &mut IndexMap<(i32, i32, i32), Scaffold>,
) {
    // Constants.
    let num_regions: f32 = 5.0;
    let map_height: f32 = map_par.dimensions.height as f32;
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
        if latitude < map_par.limit_params.icecap_limit {
            bias_icecaps(scaffold)
        }
        // Adjust weights for the northern snowsheets region.
        else if latitude
            < map_par.limit_params.icecap_limit + map_par.limit_params.snowsheet_limit
        {
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
        else if latitude
            < south_tund
                - (map_par.limit_params.icecap_limit + map_par.limit_params.snowsheet_limit)
        {
            bias_tundra(map_par, scaffold)
        }
        // Adjust weights for the southern snowsheets region.
        else if latitude < south_tund - map_par.limit_params.icecap_limit {
            bias_snowsheets(scaffold)
        }
        // Adjust weights for the southern icecaps region.
        else {
            bias_icecaps(scaffold)
        }
    }
}

fn bias_diverse(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    // The following tiles should not exist in this region.
    scaffold.purge_tile(&Terrain::Ice);
    scaffold.purge_tile(&Terrain::Snow);
    scaffold.purge_tile(&Terrain::Tundra);

    // Tiles to bias.
    let tiles_to_bias: Vec<(&Terrain, &f32)> = vec![
        (
            &Terrain::Coastal,
            &map_par.latitude_params.diverse_coastal_bias,
        ),
        (
            &Terrain::Desert,
            &map_par.latitude_params.diverse_desert_bias,
        ),
        (
            &Terrain::Grassland,
            &map_par.latitude_params.diverse_grassland_bias,
        ),
        (&Terrain::Ocean, &map_par.latitude_params.diverse_ocean_bias),
        (
            &Terrain::Steppe,
            &map_par.latitude_params.diverse_steppe_bias,
        ),
    ];

    // Adjust weights of remaining tile possibilities.
    scaffold.bias_tiles(&tiles_to_bias);
}

fn bias_equator(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    // The following tiles should not exist in this region.
    scaffold.purge_tile(&Terrain::Ice);
    scaffold.purge_tile(&Terrain::Snow);
    scaffold.purge_tile(&Terrain::Tundra);

    // Tiles to bias.
    let tiles_to_bias: Vec<(&Terrain, &f32)> = vec![
        (
            &Terrain::Coastal,
            &map_par.latitude_params.equator_coastal_bias,
        ),
        (
            &Terrain::Desert,
            &map_par.latitude_params.equator_desert_bias,
        ),
        (
            &Terrain::Grassland,
            &map_par.latitude_params.equator_grassland_bias,
        ),
        (&Terrain::Ocean, &map_par.latitude_params.equator_ocean_bias),
        (
            &Terrain::Steppe,
            &map_par.latitude_params.equator_steppe_bias,
        ),
    ];

    // Adjust weights of remaining tile possibilities.
    scaffold.bias_tiles(&tiles_to_bias);
}

fn bias_icecaps(scaffold: &mut Scaffold) {
    let mut to_remove: Vec<Terrain> = Vec::new();
    for possibility in scaffold.wave_func.domain.keys() {
        if possibility != &Terrain::Ice {
            to_remove.push(possibility.clone());
        }
    }
    for tile in to_remove.iter() {
        scaffold.purge_tile(tile);
    }
}

fn bias_snowsheets(scaffold: &mut Scaffold) {
    let mut to_remove: Vec<Terrain> = Vec::new();
    for possibility in scaffold.wave_func.domain.keys() {
        if possibility != &Terrain::Snow {
            to_remove.push(possibility.clone());
        }
    }
    for tile in to_remove.iter() {
        scaffold.purge_tile(tile);
    }
}

fn bias_tundra(map_par: &Res<MapParameters>, scaffold: &mut Scaffold) {
    // The following tiles should not exist in this region.
    scaffold.purge_tile(&Terrain::Desert);
    scaffold.purge_tile(&Terrain::Ice);

    // Tiles to bias.
    let tiles_to_bias: Vec<(&Terrain, &f32)> = vec![
        (
            &Terrain::Coastal,
            &map_par.latitude_params.tundra_coastal_bias,
        ),
        (
            &Terrain::Grassland,
            &map_par.latitude_params.tundra_grassland_bias,
        ),
        (&Terrain::Ocean, &map_par.latitude_params.tundra_ocean_bias),
        (&Terrain::Snow, &map_par.latitude_params.tundra_snow_bias),
        (
            &Terrain::Steppe,
            &map_par.latitude_params.tundra_steppe_bias,
        ),
        (
            &Terrain::Tundra,
            &map_par.latitude_params.tundra_tundra_bias,
        ),
    ];

    // Adjust weights of remaining tile possibilities.
    scaffold.bias_tiles(&tiles_to_bias);
}

/// Given some position, determines its neighboring positions.
fn determine_neighbors(width: &i32, pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    // Vars for readability.
    let q = pos.0;
    let r = pos.1;
    let s = pos.2;

    // Vector to return.
    let ret_vec: Vec<(i32, i32, i32)>;

    // Neighbors for tiles on the LEFT edge of the map.
    if r == -2 * q {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q + 1, r, s - 1),                        // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q + (width - 1), r + 1, -q - r - width), // Southwestern neighbor.
            (q + (width - 1), r, -q - r - width + 1), // Western neighbor.
            (q + width, r - 1, -q - r - width + 1),   // Northwestern neighbor.
        ]
    } else if r == -2 * q + 1 {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q + 1, r, s - 1),                        // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q + (width - 1), r, -q - r - width + 1), // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];
    }
    // Neighbors for tiles on the RIGHT edge of the map.
    else if r == 2 * (width - q - 1) {
        ret_vec = vec![
            (q + 1, r - 1, s),                        // Northeastern neighbor.
            (q - (width - 1), r, -q - r + width - 1), // Eastern neighbor.
            (q, r + 1, s - 1),                        // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q - 1, r, s + 1),                        // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];
    } else if r == 2 * (width - q) - 1 {
        ret_vec = vec![
            (q - (width - 1), r - 1, -q - r + width), // Northeastern neighbor.
            (q - (width - 1), r, -q - r + width - 1), // Eastern neighbor.
            (q - width, r + 1, -q - r + width - 1),   // Southeastern neighbor.
            (q - 1, r + 1, s),                        // Southwestern neighbor.
            (q - 1, r, s + 1),                        // Western neighbor.
            (q, r - 1, s + 1),                        // Northwestern neighbor.
        ];

    // Neighbors for tiles that are NOT on the edges, and do NOT need to wrap.
    } else {
        ret_vec = vec![
            (q + 1, r - 1, s), // Northeastern neighbor.
            (q + 1, r, s - 1), // Eastern neighbor.
            (q, r + 1, s - 1), // Southeastern neighbor.
            (q - 1, r + 1, s), // Southwestern neighbor.
            (q - 1, r, s + 1), // Western neighbor.
            (q, r - 1, s + 1), // Northwestern neighbor.
        ];
    }

    // Return neighbors.
    ret_vec
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
            pos_neighbor_map.insert(int_rep, determine_neighbors(width, int_rep));

            // Insert data into pos_scaffold_map.
            pos_scaffold_map.insert(int_rep, Scaffold::new(curr_pos));
        }
    }

    // Return the hash maps.
    (pos_neighbor_map, pos_scaffold_map)
}
