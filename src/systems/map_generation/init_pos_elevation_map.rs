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

use std::f64::consts::PI;

use bevy::prelude::*;
use indexmap::IndexMap;
use noise::{NoiseFn, Simplex, Worley};

use crate::common::components::movement::HexPos;
use crate::resources::map_parameters::MapParameters;
use crate::resources::map_parameters::NoiseType;
use crate::systems::map_generation::common::Elevation;

/// Generates a hash table that maps cube coordinates to an Ocean, Coastal, Land, or Mountain
/// elevation.
pub fn init_pos_elevation_map(
    map_par: &Res<MapParameters>,
    pos_neighbors_map: &IndexMap<HexPos, Vec<HexPos>>,
) -> IndexMap<HexPos, Elevation> {
    // Initialize a blank vector to store noise samples.
    let mut samples: Vec<f64> = Vec::new();

    // Process every noise request contained in the map's elevation parameters, updating "samples"
    // along the way.
    for request in map_par.elevation_parameters.noise_requests.iter() {
        match request.params.0 {
            NoiseType::Simplex => apply_noise_layer::<Simplex>(
                Simplex::new(map_par.seed),
                &request.params,
                &map_par,
                &mut samples,
            ),
            NoiseType::Worley => apply_noise_layer::<Worley>(
                Worley::new(map_par.seed),
                &request.params,
                &map_par,
                &mut samples,
            ),
        }
    }

    // At the moment, our noise samples can take on a multitude of values. We need to determine the
    // maximum and minimum values, then squish these and everything inbetween to a range of 0 -> 1.
    normalize_samples(&mut samples);

    // Finally, let's map the collected samples to our cube coordinates. Indexmap should preserve
    // the order of insertions, so we can index both pos_neighbors_map and sample. simultaneously.
    let pos_elevation_map = hash_samples_to_elevations(&map_par, &pos_neighbors_map, &samples);

    // Return our position elevation map.
    pos_elevation_map
}

fn apply_noise_layer<T: NoiseFn<f64, 3>>(
    noise_func: T,
    layer_par: &(NoiseType, u32, f64, f64, f64),
    map_par: &Res<MapParameters>,
    samples: &mut Vec<f64>,
) {
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

    // Sample the surface of a cylinder within the Simplex noise-field. This allows the map to
    // "wrap" around the x-axis without interpolating anything.
    let mut count: usize = 0;
    for _ in 0..map_par.height {
        for _ in 0..map_par.width {
            // Init vars for this octave.
            amplitude = 1.0;
            frequency = 1.0;
            sample = 0.0;

            // Sample a point inside the noise-field.
            for _ in 0..octaves {
                sample += noise_func.get([
                    theta.cos() / scale * frequency,
                    theta.sin() / scale * frequency,
                    z / scale * frequency,
                ]) * amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            // Update the samples vector. If the vector has less elements than the map area, we
            // know that this is the "base" layer - so simply push the sample into the vector.
            // Otherwise we need to superimpose this sample value with whatever is already contained
            // in the vector.
            if samples.len() < (map_par.width as usize * map_par.height as usize) {
                samples.push(sample)
            } else {
                samples[count] += sample;
            }

            // We've finished sampling some point. Advance to another point along the same "ring" if
            // not yet finished.
            theta += theta_increment;
            count += 1;
        }

        // We've finished sampling all points along some ring. Advance to another ring along the
        // same cylinder if not yet finished.
        z += z_increment;
        theta = 0.0;
    }
}

/// Maps a noise sample to some elevation, as defined by the map's elevation
/// parameters.
fn hash_samples_to_elevations(
    map_par: &Res<MapParameters>,
    pos_neighbors_map: &IndexMap<HexPos, Vec<HexPos>>,
    samples: &Vec<f64>,
) -> IndexMap<HexPos, Elevation> {
    // Init map to return.
    let mut pos_elevation_map: IndexMap<HexPos, Elevation> = IndexMap::new();

    // Simultaneously loop through the position neighbors map and samples vector to construct a new
    // position elevation map.
    let mut pos: HexPos;
    let mut sample: f64;
    let mut elevation: Elevation;
    for index in 0..(map_par.width as usize * map_par.height as usize) {
        // Retrieve a sample / pos pair.
        pos = *pos_neighbors_map.get_index(index).unwrap().0;
        sample = samples[index];

        // Determine what elevation the value of this sample should represent.
        if sample < map_par.elevation_parameters.ocean_threshold {
            elevation = Elevation::Ocean;
        } else if sample < map_par.elevation_parameters.coastal_threshold {
            elevation = Elevation::Coastal;
        } else if sample < map_par.elevation_parameters.land_threshold {
            elevation = Elevation::Land;
        } else {
            elevation = Elevation::Mountain;
        }

        // Insert this pair into our return map.
        pos_elevation_map.insert(pos, elevation);
    }

    // Return the newly constructed map.
    pos_elevation_map
}

/// Normalizes a vector of samples to fit within the range [0, 1].
fn normalize_samples(samples: &mut Vec<f64>) {
    // Init vars.
    let mut min: f64 = samples[0];
    let mut max: f64 = samples[0];

    // Determine the minimum and maximum value of our samples.
    for sample in samples.iter() {
        if *sample < min {
            min = *sample;
        } else if *sample > max {
            max = *sample;
        }
    }

    // Normalize our samples
    for sample in samples.iter_mut() {
        *sample = (*sample - min) / (max - min);
    }
}
