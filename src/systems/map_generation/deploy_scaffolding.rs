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
use crate::resources::map_resources::MapDimensions;

pub fn deploy_scaffolding(dims: Res<MapDimensions>, mut commands: Commands) {
    let mut curr_pos: HexPos = HexPos::new(0.0, 0.0, 0.0);
    let mut q_min: i8 = 0;
    let mut q_max: i8 = dims.width;
    for r in 0..dims.height {
        curr_pos.r = r as f32;
        if r % 2 == 0 && r != 0 {
            q_min -= 1;
            q_max -= 1;
        }
        for q in q_min..q_max {
            curr_pos.q = q as f32;
            curr_pos.s = (-q - r) as f32;
            commands.spawn((
                Name::new(format!("Tile {},{}", q, r)),
                Scaffold::new(curr_pos),
            ));
        }
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test_deploy_scaffolding() {
        // Layout a grid of tile scaffolding.
    }
}
