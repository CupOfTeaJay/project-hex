/* 
    This Life of Ours
    Copyright (C) 2024 Clevermeld LLC

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
use rand::prelude::*;

#[derive(Component)]
pub struct WaveFunction {
    domain: [(String, f32); 6] // Remember to adjust this if adding or removing tiles.
}

impl WaveFunction {
    pub fn new() -> Self {
        let domain_size = 6.0; // Remember to adjust this if adding or removing tiles.
        let uniform_prob = 1.0 / domain_size;
        WaveFunction {
            domain: [
                ("tiles/coastalTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/desertTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/grasslandTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/oceanTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/snowTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/steppeTile.glb#Scene0".to_string(), uniform_prob)
            ]
        }
    }
    pub fn collapse(&self) -> String {
        let mut rng = thread_rng();
        self.domain.choose_weighted(&mut rng, |item| item.1).unwrap().0.clone()
    }
}

