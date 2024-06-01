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

/// Plugin that registers states with the main application. Currently, the StatesPlugin:
///     - Initializes "AppState" to "AppState::InBoot".
///     - Initializes "AssetsState" to "AssetsState::NotLoaded".
///     - Initializes "BootState" to "BootState::LoadingAssets".
///     - Initializes "GameState" to "GameState::NotInGame".
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        // Register states with the main application.
        app.insert_state(AppState::InBoot)
            .insert_state(AssetsState::NotLoaded)
            .insert_state(BootState::LoadingAssets)
            .insert_state(GameState::NotInGame);
    }
}
