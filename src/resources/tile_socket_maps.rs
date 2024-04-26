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
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Resource)]
pub struct TileSocketMaps {
    pub incompatible: HashMap<String, Vec<String>>,
}

impl TileSocketMaps {
    pub fn new() -> Self {
        TileSocketMaps {
            incompatible: init_incompatible(),
        }
    }
}

#[inline]
fn init_incompatible() -> HashMap<String, Vec<String>> {
    let mut incompatible = HashMap::new();
    incompatible.insert(
        "tiles/desertTile.glb#Scene0".to_string(),
        vec![
            "tiles/iceTile.glb#Scene0".to_string(),
            "tiles/jungleTile.glb#Scene0".to_string(),
            "tiles/snowTile.glb#Scene0".to_string(),
            "tiles/tundraTile.glb#Scene0".to_string(),
        ],
    );
    incompatible
}
