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

use crate::resources::asset_handles::AssetHandles;
use crate::resources::asset_handles::SceneHandles;
use crate::states::app_state::AppState;
use crate::states::boot_state::BootState;
use crate::states::game_state::GameState;

/// Loads all assets into memory, then inserts their handles into a resource for global access.
pub fn load_game_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_boot_state: ResMut<NextState<BootState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let scenes = SceneHandles::new(
        // Load terrain handles.
        asset_server.load("tiles/coastalTile.glb#Scene0"),
        asset_server.load("tiles/debugTile.glb#Scene0"),
        asset_server.load("tiles/desertTile.glb#Scene0"),
        asset_server.load("tiles/grasslandTile.glb#Scene0"),
        asset_server.load("tiles/iceTile.glb#Scene0"),
        asset_server.load("tiles/mountainTile.glb#Scene0"),
        asset_server.load("tiles/oceanTile.glb#Scene0"),
        asset_server.load("tiles/snowTile.glb#Scene0"),
        asset_server.load("tiles/steppeTile.glb#Scene0"),
        asset_server.load("tiles/tundraTile.glb#Scene0"),
        // Load unit handles.
        asset_server.load("units/unit.glb#Scene0"),
    );

    // Insert asset handles into the app's resources.
    commands.insert_resource(AssetHandles::new(scenes));

    // Boot state transition.
    next_boot_state.set(BootState::NotInBoot);

    // App state transition.
    // TODO: transition to main menu when implemented.
    next_app_state.set(AppState::InGame);
    next_game_state.set(GameState::MapGen);
}
