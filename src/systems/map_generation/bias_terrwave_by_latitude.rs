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
use indexmap::IndexMap;
use std::f32::consts::E;
use std::f32::consts::PI;

use crate::common::components::movement::HexPos;
use crate::components::map_generation::terrain::Terrain;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::common::WaveFunction;

/// Biases terrain wave functions according to their cube coordinate's "latitude".
pub fn bias_terrwave_by_latitude(
    map_par: &Res<MapParameters>,
    pos_terrwave_map: &mut IndexMap<HexPos, WaveFunction>,
) {
    // Determine what latitude the equator should be.
    let mu: u32 = map_par.height / 2;

    // Adjust wave functions at all cube coordinates according to their latitude.
    let mut gauss: f32;
    for (pos, wave_func) in pos_terrwave_map.iter_mut() {
        // Sample gaussian at this latitude.
        gauss = calc_gaussian(
            &(mu as f32),
            &map_par.latitude_parameters.sigma,
            &(pos.r as f32),
        );

        // Adjust wave function domain at this latitude.
        for (possibility, weight) in wave_func.domain.iter_mut() {
            match possibility {
                &Terrain::Coastal => (), // Coasts are fixed.
                &Terrain::Debug => (),   // Debug terrain.
                &Terrain::Desert => *weight *= gauss * map_par.latitude_parameters.temperature,
                &Terrain::Grassland => *weight *= gauss * map_par.latitude_parameters.temperature,
                &Terrain::Ice => *weight /= gauss * map_par.latitude_parameters.temperature,
                &Terrain::Mountain => (), // Mountains are fixed.
                &Terrain::Ocean => (),    // Oceans are fixed.
                &Terrain::Snow => *weight /= gauss * map_par.latitude_parameters.temperature,
                &Terrain::Steppe => *weight *= gauss * map_par.latitude_parameters.temperature,
                &Terrain::Tundra => *weight /= gauss * map_par.latitude_parameters.temperature,
            }
        }
    }
}

/// Calculates the value of a gaussian at some point x, using the parameters mu (mean) and sigma
/// (standard deviation).
fn calc_gaussian(mu: &f32, sigma: &f32, x: &f32) -> f32 {
    // Setup for readability.
    let exp: f32 = -0.5 * ((x - mu) / sigma).powi(2);
    let num: f32 = E.powf(exp);
    let den: f32 = sigma * (2.0 * PI).sqrt();

    // Calculate gaussian at some input x.
    num / den
}
