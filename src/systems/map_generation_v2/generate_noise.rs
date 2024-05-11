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

use std::f64::consts::PI;

use bevy::prelude::*;
use indexmap::IndexMap;
use noise::{NoiseFn, Simplex};

use crate::resources::map_parameters::MapParameters;
use crate::resources::map_parameters::NoiseType;

use super::common::Elevation;

pub fn generate_noise(
    map_par: &Res<MapParameters>,
    pos_neighbors_map: &IndexMap<(i32, i32, i32), Vec<(i32, i32, i32)>>,
) -> IndexMap<(i32, i32, i32), Elevation> {
    // Initialize a blank vector to store noise samples.
    let mut samples: Vec<f64> = Vec::new();

    for layer_par in map_par.noise_parameters.iter() {
        match layer_par.request.0 {
            NoiseType::Simplex => apply_simplex_layer(&layer_par.request, &map_par, &mut samples),
        }
    }

    // Indexmap should preserve order of position insertions, so "map" samples to cube coordinates.
    let mut pos_elevation_map: IndexMap<(i32, i32, i32), Elevation> = IndexMap::new();
    let mut pos: (i32, i32, i32);
    let mut sample: f64;
    let mut elevation: Elevation;
    for i in 0..samples.len() {
        // Retrieve our sample/pos pair.
        pos = *pos_neighbors_map.get_index(i).unwrap().0;
        sample = samples[i];

        // Determine what the sample should represent.
        if sample < -0.33 {
            // TODO: parameterize.
            elevation = Elevation::Ocean;
        } else if sample < 0.33 {
            // TODO: parameterize.
            elevation = Elevation::Coastal;
        } else {
            elevation = Elevation::Land;
        }

        // Insert this pair into our return map.
        pos_elevation_map.insert(pos, elevation);
    }

    // Return.
    pos_elevation_map
}

// TODO: play with scale and z_increment.
fn apply_simplex_layer(
    layer_par: &(NoiseType, u32, f64, f64, f64),
    map_par: &Res<MapParameters>,
    samples: &mut Vec<f64>,
) {
    // Initialize a Simplex noise-field using the given map seed.
    let simplex = Simplex::new(32013);

    // Determine parameters for this layer.
    let octaves: u32 = layer_par.1;
    let scale: f64 = layer_par.2;
    let persistence: f64 = layer_par.3;
    let lacunarity: f64 = layer_par.4;

    // Init Constants.
    let theta_increment: f64 = (2.0 * PI) / (map_par.width as f64);
    let z_increment: f64 = (map_par.height as f64) / ((map_par.width as f64) * 2.0 * PI);

    // Init vars to update per map slice.
    let mut theta: f64 = 0.0;
    let mut z: f64 = 0.0;

    // Declare vars to update per octave.
    let mut amplitude: f64;
    let mut frequency: f64;
    let mut sample: f64;

    // Sample the surface of a cylinder within the Simplex noise-field.
    let mut count: usize = 0;
    for _ in 0..map_par.height {
        for _ in 0..map_par.width {
            // Init vars for this octave.
            amplitude = 1.0;
            frequency = 1.0;
            sample = 0.0;

            // Sample the octave.
            for _ in 0..octaves {
                sample += simplex.get([
                    theta.cos() / scale * frequency,
                    theta.sin() / scale * frequency,
                    z / scale * frequency,
                ]) * amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            // Update master samples with samples collected from this layer.
            if samples.len() < (map_par.width * map_par.height).try_into().unwrap() {
                samples.push(sample)
            } else {
                samples[count] += sample;
            }

            theta += theta_increment;
            count += 1;
        }
        z += z_increment;
        theta = 0.0;
    }
}
