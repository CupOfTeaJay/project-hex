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

use crate::common::resources::asset_handles::AssetHandles;

/// Loads all assets into memory, then inserts their handles into a resource for global access.
pub fn load_game_assets(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    // Load scenes.
    load_scenes(&mut asset_handles, &asset_server);
}

fn load_scenes(asset_handles: &mut ResMut<AssetHandles>, asset_server: &Res<AssetServer>) {
    // Load terrain scene handles.
    asset_handles.scenes.terrain_coastal = Some(asset_server.load("tiles/coastalTile.glb#Scene0"));
    asset_handles.scenes.terrain_debug = Some(asset_server.load("tiles/debugTile.glb#Scene0"));
    asset_handles.scenes.terrain_desert = Some(asset_server.load("tiles/desertTile.glb#Scene0"));
    asset_handles.scenes.terrain_grassland =
        Some(asset_server.load("tiles/grasslandTile.glb#Scene0"));
    asset_handles.scenes.terrain_ice = Some(asset_server.load("tiles/iceTile.glb#Scene0"));
    asset_handles.scenes.terrain_mountain =
        Some(asset_server.load("tiles/mountainTile.glb#Scene0"));
    asset_handles.scenes.terrain_ocean = Some(asset_server.load("tiles/oceanTile.glb#Scene0"));
    asset_handles.scenes.terrain_snow = Some(asset_server.load("tiles/snowTile.glb#Scene0"));
    asset_handles.scenes.terrain_steppe = Some(asset_server.load("tiles/steppeTile.glb#Scene0"));
    asset_handles.scenes.terrain_tundra = Some(asset_server.load("tiles/tundraTile.glb#Scene0"));

    // Load unit scene handles.
    asset_handles.scenes.unit_unit = Some(asset_server.load("units/debug.glb#Scene0"));

    // Load city scene handles.
    asset_handles.scenes.city_executive = Some(asset_server.load("city/cityExecutive.glb#Scene0"));
    asset_handles.scenes.city_martial = Some(asset_server.load("city/cityMartial.glb#Scene0"));
}
