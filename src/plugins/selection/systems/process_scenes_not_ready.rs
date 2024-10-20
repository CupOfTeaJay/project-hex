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

use crate::{
    common::resources::pickable_buffers::{PickableBufferHelpers, PickableBuffers},
    common::states::pickable_buffers_state::PickableBuffersState,
};

use super::{
    make_meshes_pickable::make_meshes_pickable,
    pickable_buffers_populated::pickable_buffers_populated,
};

pub fn process_scenes_not_ready(
    mut commands: Commands,
    children: Query<&Children>,
    entities: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
    mut helper_vecs: ResMut<PickableBufferHelpers>,
    mut next_pickable_buffers_state: ResMut<NextState<PickableBuffersState>>,
    mut pickable_buffers: ResMut<PickableBuffers>,
    scene_instances: Query<&SceneInstance>,
    scene_manager: Res<SceneSpawner>,
) {
    for entity in pickable_buffers.scenes_not_ready.iter() {
        if let Ok(scene) = scene_instances.get(*entity) {
            if scene_manager.instance_is_ready(**scene) {
                helper_vecs.scenes_ready.push(*entity);
                make_meshes_pickable(&mut commands, &entity, &children, &entities);
            }
        }
    }
    for entity in helper_vecs.scenes_ready.iter() {
        pickable_buffers.scenes_not_ready.remove(entity);
    }
    if !pickable_buffers_populated(&pickable_buffers) {
        // Dump helper vectors.
        helper_vecs.scenes_instanced.clear();
        helper_vecs.scenes_ready.clear();
        // State transition.
        next_pickable_buffers_state.set(PickableBuffersState::Empty);
    }
}
