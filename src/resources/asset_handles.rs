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
    pub fn new() -> Self {
        AssetHandles {
            scenes: SceneHandles::new(),
        }
    }
}

pub struct SceneHandles {
    // Terrain scene handles.
    pub terrain_coastal: Option<Handle<Scene>>,
    pub terrain_debug: Option<Handle<Scene>>,
    pub terrain_desert: Option<Handle<Scene>>,
    pub terrain_grassland: Option<Handle<Scene>>,
    pub terrain_ice: Option<Handle<Scene>>,
    pub terrain_mountain: Option<Handle<Scene>>,
    pub terrain_ocean: Option<Handle<Scene>>,
    pub terrain_snow: Option<Handle<Scene>>,
    pub terrain_steppe: Option<Handle<Scene>>,
    pub terrain_tundra: Option<Handle<Scene>>,
    // Unit scene handles.
    pub unit_unit: Option<Handle<Scene>>,
    // City scene handles.
    pub city_center: Option<Handle<Scene>>,
}

impl SceneHandles {
    pub fn new() -> Self {
        SceneHandles {
            // Terrain scene handles.
            terrain_coastal: None,
            terrain_debug: None,
            terrain_desert: None,
            terrain_grassland: None,
            terrain_ice: None,
            terrain_mountain: None,
            terrain_ocean: None,
            terrain_snow: None,
            terrain_steppe: None,
            terrain_tundra: None,
            // Unit scene handles.
            unit_unit: None,
            // City scene handles.
            city_center: None,
        }
    }
}
