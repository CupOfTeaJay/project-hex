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

use crate::components::hex_pos::HexPos;
use crate::components::scaffold::Scaffold;

const Q_MAX: i8 = 20;
const R_MAX: i8 = 20;
const S_MAX: i8 = 20;

// TODO: pretty ugly and probably inefficient.
pub fn deploy_scaffolding(mut commands: Commands) {
    // Hex position to be updated as the map is being generated.
    let mut curr_pos: HexPos = HexPos::new(0.0, 0.0, 0.0);
    for q in -Q_MAX..=Q_MAX {
        curr_pos.q = q as f32;
        for r in -R_MAX..=R_MAX {
            curr_pos.r = r as f32;
            for s in -S_MAX..=S_MAX {
                curr_pos.s = s as f32;
                if q + r + s == 0 {
                    commands.spawn(Scaffold::new(curr_pos));
                }
            }
        }
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test_deploy_scaffolding() {
        // Layout a grid of cell scaffolding.
    }
}
