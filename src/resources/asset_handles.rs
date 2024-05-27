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

#[derive(Resource)]
pub struct AssetHandles {
    pub scenes: SceneHandles,
}

impl AssetHandles {
    pub fn new(scenes: SceneHandles) -> Self {
        AssetHandles { scenes }
    }
}

pub struct SceneHandles {
    // Terrain scene handles.
    pub terrain_coastal: Handle<Scene>,
    pub terrain_debug: Handle<Scene>,
    pub terrain_desert: Handle<Scene>,
    pub terrain_grassland: Handle<Scene>,
    pub terrain_ice: Handle<Scene>,
    pub terrain_mountain: Handle<Scene>,
    pub terrain_ocean: Handle<Scene>,
    pub terrain_snow: Handle<Scene>,
    pub terrain_steppe: Handle<Scene>,
    pub terrain_tundra: Handle<Scene>,
    // Unit scene handles.
    pub unit_unit: Handle<Scene>,
}

impl SceneHandles {
    pub fn new(
        // Terrain scene handles.
        terrain_coastal: Handle<Scene>,
        terrain_debug: Handle<Scene>,
        terrain_desert: Handle<Scene>,
        terrain_grassland: Handle<Scene>,
        terrain_ice: Handle<Scene>,
        terrain_mountain: Handle<Scene>,
        terrain_ocean: Handle<Scene>,
        terrain_snow: Handle<Scene>,
        terrain_steppe: Handle<Scene>,
        terrain_tundra: Handle<Scene>,
        // Unit scene handles.
        unit_unit: Handle<Scene>,
    ) -> Self {
        SceneHandles {
            // Terrain scene handles.
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
            // Unit scene handles.
            unit_unit,
        }
    }
}
