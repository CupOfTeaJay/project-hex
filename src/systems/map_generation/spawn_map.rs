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
use bevy_mod_picking::prelude::*;
use indexmap::IndexMap;
use std::f32::consts::FRAC_PI_2;

use crate::components::common::hex_pos::HexPos;
use crate::components::map_generation::terrain::Terrain;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::events::tile_spawn_event::TileSpawnEvent;
use crate::resources::asset_handles::AssetHandles;
use crate::resources::map_parameters::MapParameters;
use crate::states::game_state::GameState;
use crate::systems::map_generation::generate_map_data::generate_map_data;
use crate::utils::coord_conversions::cube_to_cartesian;
use crate::utils::get_top_parent::get_top_parent;

pub fn spawn_map(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    map_par: Res<MapParameters>,
    mut tile_spawn_event: EventWriter<TileSpawnEvent>,
) {
    // Setup.
    let pos_terr_map: IndexMap<HexPos, Terrain> = generate_map_data(&map_par);
    let mut transform: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.rotate_y(FRAC_PI_2);

    // Spawn.
    let (mut x, mut y, mut z): (f32, f32, f32);
    for (pos, terrain) in pos_terr_map.iter() {
        (x, y, z) = cube_to_cartesian(pos.q as f32, pos.r as f32, pos.s as f32);
        transform.translation.x = x;
        transform.translation.y = y;
        transform.translation.z = z;

        let scene_handle: Handle<Scene>;
        match terrain {
            &Terrain::Coastal => scene_handle = asset_handles.scenes.terrain_coastal.clone(),
            &Terrain::Debug => scene_handle = asset_handles.scenes.terrain_debug.clone(),
            &Terrain::Desert => scene_handle = asset_handles.scenes.terrain_desert.clone(),
            &Terrain::Grassland => scene_handle = asset_handles.scenes.terrain_grassland.clone(),
            &Terrain::Ice => scene_handle = asset_handles.scenes.terrain_ice.clone(),
            &Terrain::Mountain => scene_handle = asset_handles.scenes.terrain_mountain.clone(),
            &Terrain::Ocean => scene_handle = asset_handles.scenes.terrain_ocean.clone(),
            &Terrain::Snow => scene_handle = asset_handles.scenes.terrain_snow.clone(),
            &Terrain::Steppe => scene_handle = asset_handles.scenes.terrain_steppe.clone(),
            &Terrain::Tundra => scene_handle = asset_handles.scenes.terrain_tundra.clone(),
        }

        let scene_bundle = SceneBundle {
            scene: scene_handle,
            transform: transform,
            ..Default::default()
        };

        // Spawn.
        let entity = commands
            .spawn((
                TileBundle::new(*pos, *terrain, scene_bundle),
                PickSelection { is_selected: false },
                On::<Pointer<Up>>::run(
                    |event: Listener<Pointer<Up>>,
                     mut selectables: Query<&mut PickSelection>,
                     parents: Query<&Parent>| {
                        selectables
                            .get_mut(get_top_parent(&event.target, &parents))
                            .unwrap()
                            .is_selected = true;
                    },
                ),
            ))
            .id();
        tile_spawn_event.send(TileSpawnEvent::new(entity));

        // GameState transition.
        next_game_state.set(GameState::PlayerInit);
    }
}
