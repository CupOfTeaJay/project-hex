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

#[rustfmt::skip]
use crate::states::{
    app_state::AppState,
    assets_state::AssetsState,
    boot_state::BootState,
    game_state::GameState,
};

use crate::systems::map_generation::set_map_parameters::set_map_parameters;
use crate::systems::map_generation::spawn_map::spawn_map;

/// Plugin that encapsulates algorithms related to map generation. Currently, the MapPlugin:
///     - Spawns a map from a given seed.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            (set_map_parameters, spawn_map)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(in_state(GameState::MapGen)),
        );
    }
}
