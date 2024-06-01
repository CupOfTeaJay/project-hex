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

#[rustfmt::skip]
use crate::systems::{
    boot::load_game_assets::load_game_assets,
    boot::validate_assets_loaded::validate_assets_loaded,
};

/// Plugin that initializes the game upon launch. Currently, the BootPlugin:
///     - Loads assets.
///     - Validates requested assets have successfully loaded.
pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        // Add Startup scheduled systems to the main application.
        app.add_systems(
            Startup,
            load_game_assets
                .run_if(in_state(AppState::InBoot))
                .run_if(in_state(AssetsState::NotLoaded))
                .run_if(in_state(BootState::LoadingAssets))
                .run_if(in_state(GameState::NotInGame)),
        );
        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            validate_assets_loaded
                .run_if(in_state(AppState::InBoot))
                .run_if(in_state(AssetsState::NotLoaded))
                .run_if(in_state(BootState::LoadingAssets))
                .run_if(in_state(GameState::NotInGame)),
        );
    }
}
