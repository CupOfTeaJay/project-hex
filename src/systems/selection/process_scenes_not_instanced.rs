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

use crate::resources::pickable_deques::PickableDeques;

pub fn process_scenes_not_instanced(
    mut pickable_deques: ResMut<PickableDeques>,
    scene_instances: Query<&SceneInstance>,
) {
    if let Some(entity) = pickable_deques.scenes_not_instanced.back().cloned() {
        if let Ok(_) = scene_instances.get(entity) {
            pickable_deques.scenes_not_instanced.pop_back();
            pickable_deques.scenes_not_ready.push_front(entity);
        }
    }
}
