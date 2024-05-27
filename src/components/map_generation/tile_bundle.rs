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

use crate::components::common::hex_pos::HexPos;
use crate::components::common::is_populated::IsPopulated;
use crate::components::map_generation::terrain::Terrain;

/// Suite of components for tile entities.
#[derive(Bundle)]
pub struct TileBundle {
    pos: HexPos,
    terrain: Terrain,
    populated: IsPopulated,
    model: SceneBundle,
}

impl TileBundle {
    /// Creates a tile bundle.
    pub fn new(pos: HexPos, terrain: Terrain, model: SceneBundle) -> Self {
        TileBundle {
            pos: pos,
            terrain: terrain,
            populated: IsPopulated::new(false),
            model: model,
        }
    }
}

// TODO: test TileBundle::new()
