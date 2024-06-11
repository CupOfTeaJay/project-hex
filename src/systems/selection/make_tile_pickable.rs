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
use crate::resources::pickable_deques::PickableDeques;
use crate::systems::selection::make_meshes_pickable::make_meshes_pickable;

// TODO: There's probably a much better way to do all of this.
/// Makes a unit scene pickable (selectable).
pub fn make_tile_pickable(
    mut commands: Commands,
    mut pickable_deques: ResMut<PickableDeques>,
    children: Query<&Children>,
    entities: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
    scenes: Query<&SceneInstance>,
    scene_manager: Res<SceneSpawner>,
    mut tile_spawn_event: EventReader<TileSpawnEvent>,
) {
    for event in tile_spawn_event.read() {
        if let Ok(scene_instance) = scenes.get(event.entity) {
            if scene_manager.instance_is_ready(**scene_instance) {
                make_meshes_pickable(&mut commands, &event.entity, &children, &entities);
            } else {
                pickable_deques.scenes_not_ready.push_front(event.entity)
            }
        } else {
            pickable_deques
                .scenes_not_instanced
                .push_front(event.entity);
        }
    }
}
