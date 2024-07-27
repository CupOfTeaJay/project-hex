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

use crate::common::components::movement::HexPos;
use crate::common::systems::utils::hexpos_to_vec3;
use crate::components::map_generation::terrain::Terrain;
use crate::components::map_generation::tile_bundle::TileBundle;
use crate::events::pickable_spawn_event::PickableSpawnEvent;
use crate::resources::asset_handles::AssetHandles;
use crate::resources::map_parameters::MapParameters;
use crate::resources::pos_neighbors_map::PosNeighborsMap;
use crate::resources::traversability_maps::TraversabilityMaps;
use crate::states::game_state::GameState;
use crate::systems::map_generation::generate_map_data::generate_map_data;
use crate::utils::get_ancestor::get_ancestor;

pub fn spawn_map(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    map_par: Res<MapParameters>,
    mut pos_neighbors_map_res: ResMut<PosNeighborsMap>,
    mut tile_spawn_event: EventWriter<PickableSpawnEvent>,
    mut traversability_maps: ResMut<TraversabilityMaps>,
) {
    // Setup.
    let (pos_terr_map, pos_neighbors_map): (
        IndexMap<HexPos, Terrain>,
        IndexMap<HexPos, Vec<HexPos>>,
    ) = generate_map_data(&map_par);

    // Insert the HexPos -> HexPos neighbors map into app resources. It's useful for other
    // algorithms, such as A* pathfinding.
    pos_neighbors_map_res.map = pos_neighbors_map;

    let mut transform: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.rotate_y(FRAC_PI_2);

    // Spawn.
    let (mut x, mut y, mut z): (f32, f32, f32);
    for (pos, terrain) in pos_terr_map.iter() {
        transform.translation = hexpos_to_vec3(pos);
        let scene_handle: Handle<Scene>;
        match terrain {
            &Terrain::Coastal => {
                scene_handle = asset_handles.scenes.terrain_coastal.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, false);
            }
            &Terrain::Debug => {
                scene_handle = asset_handles.scenes.terrain_debug.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, false);
            }
            &Terrain::Desert => {
                scene_handle = asset_handles.scenes.terrain_desert.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, true);
            }
            &Terrain::Grassland => {
                scene_handle = asset_handles.scenes.terrain_grassland.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, true);
            }
            &Terrain::Ice => {
                scene_handle = asset_handles.scenes.terrain_ice.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, false);
            }
            &Terrain::Mountain => {
                scene_handle = asset_handles.scenes.terrain_mountain.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, false);
            }
            &Terrain::Ocean => {
                scene_handle = asset_handles.scenes.terrain_ocean.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, false);
            }
            &Terrain::Snow => {
                scene_handle = asset_handles.scenes.terrain_snow.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, true);
            }
            &Terrain::Steppe => {
                scene_handle = asset_handles.scenes.terrain_steppe.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, true);
            }
            &Terrain::Tundra => {
                scene_handle = asset_handles.scenes.terrain_tundra.clone().unwrap();
                traversability_maps.pos_land_map.insert(*pos, true);
            }
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
                            .get_mut(get_ancestor(&event.target, &parents))
                            .unwrap()
                            .is_selected = true;
                    },
                ),
            ))
            .id();
        tile_spawn_event.send(PickableSpawnEvent::new(entity));

        // GameState transition.
        next_game_state.set(GameState::PlayerInit);
    }
}
