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

use bevy::{asset::LoadState, prelude::*};

pub fn test() {}

use crate::{
    resources::asset_handles::AssetHandles,
    states::{
        app_state::AppState, assets_state::AssetsState, boot_state::BootState,
        game_state::GameState,
    },
};

pub fn validate_assets_loaded(
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_assets_state: ResMut<NextState<AssetsState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_boot_state: ResMut<NextState<BootState>>,
) {
    // Flag used to signal that all assets are loaded.
    let mut assets_loaded: bool = true;

    // Validate the asset server has loaded all requested scene handles.
    validate_scenes_loaded(&mut assets_loaded, &asset_handles, &asset_server);

    // Assert state transition if all assets are loaded.
    if assets_loaded {
        next_app_state.set(AppState::InGame);
        next_assets_state.set(AssetsState::Loaded);
        next_boot_state.set(BootState::NotInBoot);
        next_game_state.set(GameState::MapGen);
    }
}

fn validate_scenes_loaded(
    assets_loaded: &mut bool,
    asset_handles: &Res<AssetHandles>,
    asset_server: &Res<AssetServer>,
) {
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_coastal.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_debug.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_desert.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_grassland.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_ice.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_mountain.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_ocean.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_snow.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_steppe.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.terrain_tundra.id()) == LoadState::Loaded;
    *assets_loaded &=
        asset_server.load_state(asset_handles.scenes.unit_unit.id()) == LoadState::Loaded;
}
