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

use crate::events::unit_spawn_event::UnitSpawnEvent;

// TODO: There's probably a much better way to do all of this.
/// Makes a unit scene pickable (selectable).
pub fn make_unit_pickable(
    mut commands: Commands,
    query_children: Query<&Children>,
    query_entities: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
    mut unit_spawn_event: EventReader<UnitSpawnEvent>,
) {
    for event in unit_spawn_event.read() {
        make_meshes_pickable_recursively(
            &mut commands,
            &event.entity,
            &query_children,
            &query_entities,
        );
    }
}

fn make_meshes_pickable_recursively(
    commands: &mut Commands,
    parent: &Entity,
    query_children: &Query<&Children>,
    query_entities: &Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    if let Ok(children) = query_children.get(*parent) {
        for child in children {
            make_meshes_pickable_recursively(commands, child, query_children, query_entities);
        }
    }

    if let Ok(entity_with_mesh) = query_entities.get(*parent) {
        commands
            .entity(entity_with_mesh)
            .insert(PickableBundle::default());
    }
}
