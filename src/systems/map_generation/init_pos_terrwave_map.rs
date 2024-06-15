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

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::terrain::Terrain;
use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::common::Elevation;
use crate::systems::map_generation::common::WaveFunction;

/// Initializes terrain wave functions for every cube coordinate on the map.
pub fn init_pos_terrwave_map(
    map_par: &Res<MapParameters>,
    pos_elevation_map: &IndexMap<HexPos, Elevation>,
) -> IndexMap<HexPos, WaveFunction> {
    // Initialize a wave function hash table.
    let mut pos_wave_map: IndexMap<HexPos, WaveFunction> = IndexMap::new();
    for pos in pos_elevation_map.keys() {
        pos_wave_map.insert(*pos, WaveFunction::new());
    }

    // Adjust the newly created hash table according to the input position elevation map.
    let (mut pos, mut elevation): (&HexPos, &Elevation);
    for index in 0..pos_wave_map.len() {
        (pos, elevation) = pos_elevation_map.get_index(index).unwrap();
        if pos.r == 0 || pos.r == (map_par.height as i32) - 1 {
            // The northermost and southernmost latitudes should only be ice tiles.
            pos_wave_map[pos].purge(&Terrain::Coastal);
            pos_wave_map[pos].purge(&Terrain::Debug);
            pos_wave_map[pos].purge(&Terrain::Desert);
            pos_wave_map[pos].purge(&Terrain::Grassland);
            pos_wave_map[pos].purge(&Terrain::Mountain);
            pos_wave_map[pos].purge(&Terrain::Ocean);
            pos_wave_map[pos].purge(&Terrain::Snow);
            pos_wave_map[pos].purge(&Terrain::Steppe);
            pos_wave_map[pos].purge(&Terrain::Tundra);
        } else if elevation == &Elevation::Ocean {
            // Purge everything except for the OCEAN terrain.
            pos_wave_map[pos].purge(&Terrain::Coastal);
            pos_wave_map[pos].purge(&Terrain::Debug);
            pos_wave_map[pos].purge(&Terrain::Desert);
            pos_wave_map[pos].purge(&Terrain::Grassland);
            pos_wave_map[pos].purge(&Terrain::Ice);
            pos_wave_map[pos].purge(&Terrain::Mountain);
            pos_wave_map[pos].purge(&Terrain::Snow);
            pos_wave_map[pos].purge(&Terrain::Steppe);
            pos_wave_map[pos].purge(&Terrain::Tundra);
        } else if elevation == &Elevation::Coastal {
            // Purge everything except for the COASTAL terrain.
            pos_wave_map[pos].purge(&Terrain::Debug);
            pos_wave_map[pos].purge(&Terrain::Desert);
            pos_wave_map[pos].purge(&Terrain::Grassland);
            pos_wave_map[pos].purge(&Terrain::Ice);
            pos_wave_map[pos].purge(&Terrain::Mountain);
            pos_wave_map[pos].purge(&Terrain::Ocean);
            pos_wave_map[pos].purge(&Terrain::Snow);
            pos_wave_map[pos].purge(&Terrain::Steppe);
            pos_wave_map[pos].purge(&Terrain::Tundra);
        } else if elevation == &Elevation::Land {
            // Purge the OCEAN, ICE, MOUNTAIN, and COASTAL terrains.
            pos_wave_map[pos].purge(&Terrain::Coastal);
            pos_wave_map[pos].purge(&Terrain::Debug);
            pos_wave_map[pos].purge(&Terrain::Ocean);
            pos_wave_map[pos].purge(&Terrain::Ice);
            pos_wave_map[pos].purge(&Terrain::Mountain);
        } else {
            // Purge everything except for the MOUNTAIN terrain.
            pos_wave_map[pos].purge(&Terrain::Coastal);
            pos_wave_map[pos].purge(&Terrain::Debug);
            pos_wave_map[pos].purge(&Terrain::Desert);
            pos_wave_map[pos].purge(&Terrain::Grassland);
            pos_wave_map[pos].purge(&Terrain::Ice);
            pos_wave_map[pos].purge(&Terrain::Ocean);
            pos_wave_map[pos].purge(&Terrain::Snow);
            pos_wave_map[pos].purge(&Terrain::Steppe);
            pos_wave_map[pos].purge(&Terrain::Tundra);
        }
    }

    // Return our position to wave function map.
    pos_wave_map
}
