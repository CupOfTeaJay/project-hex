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
use rand::prelude::*;

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::components::map_generation::wave_func::WaveFunc;

pub fn wave_func_collapse(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(&HexPos, &Transform, &WaveFunc)>,
) {
    for (pos, transform, wave_func) in &query {
        // Select tile terrain.
        let mut rng: ThreadRng = thread_rng();
        let choice: String = wave_func
            .domain
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
            .clone();

        // Initialize the model.
        let model: SceneBundle = SceneBundle {
            scene: asset_server.load(choice),
            transform: *transform,
            ..Default::default()
        };

        // Spawn the tile.
        commands.spawn(TileBundle::new(*pos, model));
    }
}
