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

const DOMAIN_SIZE: usize = 9;
const UNIFORM_PROB: f32 = 1.0 / (DOMAIN_SIZE as f32);

pub struct WaveFunction {
    pub domain: [(String, f32); DOMAIN_SIZE],
}

impl WaveFunction {
    pub fn new() -> Self {
        WaveFunction {
            domain: init_domain(),
        }
    }
}

fn init_domain() -> [(String, f32); DOMAIN_SIZE] {
    [
        ("tiles/coastalTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/desertTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/grasslandTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/iceTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/jungleTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/oceanTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/snowTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/steppeTile.glb#Scene0".to_string(), UNIFORM_PROB),
        ("tiles/tundraTile.glb#Scene0".to_string(), UNIFORM_PROB),
    ]
}

// TODO: test WaveFunction::new()
