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

/// Loads all assets into memory, then inserts their handles into a resource for global access.
pub fn load_game_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Load scenes.
    let scenes: SceneHandles = load_scenes(&asset_server);

    // Insert asset handles into the app's resources.
    commands.insert_resource(AssetHandles::new(scenes));
}

fn load_scenes(asset_server: &Res<AssetServer>) -> SceneHandles {
    // Load terrain scene handles.
    let terrain_coastal: Handle<Scene> = asset_server.load("tiles/coastalTile.glb#Scene0");
    let terrain_debug: Handle<Scene> = asset_server.load("tiles/debugTile.glb#Scene0");
    let terrain_desert: Handle<Scene> = asset_server.load("tiles/desertTile.glb#Scene0");
    let terrain_grassland: Handle<Scene> = asset_server.load("tiles/grasslandTile.glb#Scene0");
    let terrain_ice: Handle<Scene> = asset_server.load("tiles/iceTile.glb#Scene0");
    let terrain_mountain: Handle<Scene> = asset_server.load("tiles/mountainTile.glb#Scene0");
    let terrain_ocean: Handle<Scene> = asset_server.load("tiles/oceanTile.glb#Scene0");
    let terrain_snow: Handle<Scene> = asset_server.load("tiles/snowTile.glb#Scene0");
    let terrain_steppe: Handle<Scene> = asset_server.load("tiles/steppeTile.glb#Scene0");
    let terrain_tundra: Handle<Scene> = asset_server.load("tiles/tundraTile.glb#Scene0");

    // Load unit scene handles.
    let unit_debug: Handle<Scene> = asset_server.load("units/debug.glb#Scene0");

    SceneHandles::new(
        // Terrain handles.
        terrain_coastal,
        terrain_debug,
        terrain_desert,
        terrain_grassland,
        terrain_ice,
        terrain_mountain,
        terrain_ocean,
        terrain_snow,
        terrain_steppe,
        terrain_tundra,
        // Unit handles.
        unit_debug,
    )
}
