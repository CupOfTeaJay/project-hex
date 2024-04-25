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

const DOMAIN_SIZE: usize = 8;
const UNIFORM_PROB: f32 = 1.0 / (DOMAIN_SIZE as f32);

#[derive(Component)]
pub struct WaveFunc {
    pub domain: [(String, f32); DOMAIN_SIZE],
}

impl WaveFunc {
    pub fn new() -> Self {
        WaveFunc {
            domain: [
                ("tiles/coastalTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/desertTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/grasslandTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/iceTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/jungleTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/oceanTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/snowTile.glb#Scene0".to_string(), UNIFORM_PROB),
                ("tiles/steppeTile.glb#Scene0".to_string(), UNIFORM_PROB),
            ],
        }
    }
}

// TODO: test WaveFunc::new()
