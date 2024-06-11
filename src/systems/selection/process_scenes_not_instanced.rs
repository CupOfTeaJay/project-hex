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

use bevy::{prelude::*, scene::SceneInstance};

use crate::resources::pickable_buffers::{PickableBufferHelpers, PickableBuffers};

pub fn process_scenes_not_instanced(
    mut helper_vecs: ResMut<PickableBufferHelpers>,
    mut pickable_buffers: ResMut<PickableBuffers>,
    scene_instances: Query<&SceneInstance>,
) {
    for entity in pickable_buffers.scenes_not_instanced.iter() {
        if let Ok(_) = scene_instances.get(*entity) {
            helper_vecs.scenes_instanced.push(*entity);
        }
    }
    for entity in helper_vecs.scenes_instanced.iter() {
        pickable_buffers.scenes_not_instanced.remove(entity);
        pickable_buffers.scenes_not_ready.insert(*entity);
    }
}
