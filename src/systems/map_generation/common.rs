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

use indexmap::IndexMap;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::components::map_generation::terrain::Terrain;

/// Classifiers for what positions during map generation will collapse to a
/// coastal, ocean, or some land tile.
#[derive(Clone, Eq, Hash, PartialEq, Copy)]
pub enum Elevation {
    Coastal,
    Land,
    Mountain,
    Ocean,
}

pub struct WaveFunction {
    pub domain: IndexMap<Terrain, f32>,
    pub entropy: usize,
}

impl WaveFunction {
    /// Creates a new Wave Function.
    pub fn new() -> Self {
        // Insert all possible terrains into the map.
        let mut domain = IndexMap::new();
        domain.insert(Terrain::Coastal, 0.0);
        domain.insert(Terrain::Desert, 0.0);
        domain.insert(Terrain::Debug, 0.0);
        domain.insert(Terrain::Grassland, 0.0);
        domain.insert(Terrain::Ice, 0.0);
        domain.insert(Terrain::Mountain, 0.0);
        domain.insert(Terrain::Ocean, 0.0);
        domain.insert(Terrain::Snow, 0.0);
        domain.insert(Terrain::Steppe, 0.0);
        domain.insert(Terrain::Tundra, 0.0);

        // Quickly iterate over what was inserted to apply a uniform probability distribution.
        let entropy: usize = domain.len();
        let uniform: f32 = 100.0 / (domain.len() as f32);
        for probability in domain.values_mut() {
            *probability = uniform;
        }

        // Return a newly created wave function.
        WaveFunction { domain, entropy }
    }

    /// Selects a possible terrain from the wave function's domain.
    pub fn collapse(&self, seed: u32) -> &Terrain {
        // Sometimes the heightmap will assign an ocean elevation adjacent to a land elevation. In
        // this case, just put a coastal tile in its place.
        if self.domain.keys().len() == 0 {
            return &Terrain::Coastal;
        }

        // Pre-process the domain by calculating a prefix sum of its weights.
        let mut possibilities: Vec<&Terrain> = Vec::new();
        let mut weights_pref_sums: Vec<f32> = Vec::new();
        let mut curr_sum: f32 = 0.0;
        for (possibility, weight) in self.domain.iter() {
            possibilities.push(possibility);
            curr_sum += weight;
            weights_pref_sums.push(curr_sum.clone());
        }

        // Select a random, weighted possibility from the wave function's domain.
        let mut choice_index: usize = 0;
        let rand_num: f32 = StdRng::seed_from_u64(seed as u64).gen_range(0.0..curr_sum);
        for i in 0..weights_pref_sums.len() {
            if rand_num < weights_pref_sums[i] {
                choice_index = i;
                break;
            }
        }

        possibilities[choice_index]
    }

    /// Removes a select terrain from the wave function's domain, divvy-ing its weight with all
    /// remaining possibilities. Returns the weight of the purged terrain if successful.
    pub fn purge(&mut self, possibility: &Terrain) -> Option<f32> {
        if let Some(weight) = self.domain.swap_remove(possibility) {
            self.divvy_weight(&weight); // If terrain successfully removed, divvy its weight.
            self.entropy -= 1; // Remember to reduce the wave function's entropy!
            Some(weight) // Return the weight that was "popped".
        } else {
            None
        }
    }

    /// Given some weight, divvy it into equal shares and adjust remaining terrain weights
    /// accordingly.
    fn divvy_weight(&mut self, weight_to_divvy: &f32) {
        let domain_size: f32 = self.domain.keys().len() as f32;
        let divvy: f32 = weight_to_divvy / domain_size;
        for weight in self.domain.values_mut() {
            *weight += divvy;
        }
    }
}
