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

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::terrain::Terrain;

/// Applies a convoloution to map terrain. The number of passes is specified in the map parameters.
pub fn apply_terr_convolution(
    pos_neighbors_map: &IndexMap<HexPos, Vec<HexPos>>,
    pos_terr_map: &mut IndexMap<HexPos, Terrain>,
) {
    // We're not going to mutate pos_terr_map in place (so the convolution does not propogate).
    // Instead, make a copy to replace it with.
    let mut temp_pos_terr_map: IndexMap<HexPos, Terrain> = pos_terr_map.clone();

    // Initialize terrain to frequency map.
    let mut terr_freqs: IndexMap<&Terrain, u32> = IndexMap::new();
    terr_freqs.insert(&Terrain::Coastal, 0);
    terr_freqs.insert(&Terrain::Debug, 0);
    terr_freqs.insert(&Terrain::Desert, 0);
    terr_freqs.insert(&Terrain::Grassland, 0);
    terr_freqs.insert(&Terrain::Ice, 0);
    terr_freqs.insert(&Terrain::Mountain, 0);
    terr_freqs.insert(&Terrain::Ocean, 0);
    terr_freqs.insert(&Terrain::Snow, 0);
    terr_freqs.insert(&Terrain::Steppe, 0);
    terr_freqs.insert(&Terrain::Tundra, 0);

    // Loop vars to update.
    let mut curr_pos: &HexPos;
    let mut curr_terr: &Terrain;
    let mut pos_clone: HexPos;
    let mut max_freq: u32;
    let mut max_terr: &Terrain;

    // Loop through every cube coordinate on the map, and calculate "average" terrain at that
    // position.
    let num_tiles: usize = temp_pos_terr_map.len();
    for index in 0..num_tiles {
        // Reset terrain to frequency map.
        terr_freqs[&Terrain::Coastal] = 0;
        terr_freqs[&Terrain::Debug] = 0;
        terr_freqs[&Terrain::Desert] = 0;
        terr_freqs[&Terrain::Grassland] = 0;
        terr_freqs[&Terrain::Ice] = 0;
        terr_freqs[&Terrain::Mountain] = 0;
        terr_freqs[&Terrain::Ocean] = 0;
        terr_freqs[&Terrain::Snow] = 0;
        terr_freqs[&Terrain::Steppe] = 0;
        terr_freqs[&Terrain::Tundra] = 0;

        // Reset maximums.
        max_freq = 0;
        max_terr = &Terrain::Debug;

        // Get current position and terrain. Update frequency map.
        (curr_pos, curr_terr) = pos_terr_map.get_index(index).unwrap();
        terr_freqs[curr_terr] += 1;

        // Now determine neighboring terrains. Update frequency map accordingly.
        for neighbor_pos in pos_neighbors_map.get(curr_pos).unwrap().iter() {
            if let Some(terr) = pos_terr_map.get(neighbor_pos) {
                terr_freqs[terr] += 1;
            }
        }

        // Finally, find out what the most frequent terrain was. Earliest maximum wins.
        for (terr, freq) in terr_freqs.iter() {
            if *freq > max_freq {
                max_freq = *freq;
                max_terr = terr;
            }
        }

        // Replace the terrain at this position to that with the highest frequency.
        pos_clone = curr_pos.clone();
        temp_pos_terr_map[&pos_clone] = *max_terr;
    }

    // Update pos_terr_map.
    for (pos, terr) in temp_pos_terr_map.iter() {
        pos_terr_map[pos] = *terr;
    }
}
