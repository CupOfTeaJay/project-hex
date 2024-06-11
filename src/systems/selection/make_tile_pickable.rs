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
use bevy::scene::SceneInstance;
use bevy_mod_picking::prelude::*;

use crate::events::tile_spawn_event::TileSpawnEvent;
use crate::resources::pickable_buffers::PickableBuffers;
use crate::states::pickable_buffers_state::PickableBuffersState;
use crate::systems::selection::make_meshes_pickable::make_meshes_pickable;

use crate::systems::selection::pickable_buffers_populated::pickable_buffers_populated;

// TODO: There's probably a much better way to do all of this.
/// Makes a unit scene pickable (selectable).
pub fn make_tile_pickable(
    mut commands: Commands,
    mut pickable_buffers: ResMut<PickableBuffers>,
    children: Query<&Children>,
    entities: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
    mut next_pickable_buffers_state: ResMut<NextState<PickableBuffersState>>,
    scenes: Query<&SceneInstance>,
    scene_manager: Res<SceneSpawner>,
    mut tile_spawn_event: EventReader<TileSpawnEvent>,
) {
    for event in tile_spawn_event.read() {
        if let Ok(scene_instance) = scenes.get(event.entity) {
            if scene_manager.instance_is_ready(**scene_instance) {
                make_meshes_pickable(&mut commands, &event.entity, &children, &entities);
            } else {
                pickable_buffers.scenes_not_ready.insert(event.entity);
            }
        } else {
            pickable_buffers.scenes_not_instanced.insert(event.entity);
        }
        if pickable_buffers_populated(&pickable_buffers) {
            next_pickable_buffers_state.set(PickableBuffersState::Populated);
        }
    }
}
