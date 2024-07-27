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
use bevy_mod_picking::selection::PickSelection;

#[rustfmt::skip]
use crate::common::components::{
    movement::HexPos,
    movement::IsMovable,
    movement::IsTraversable,
};

#[rustfmt::skip]
use crate::events::movement_event::MovementEvent;

/// Checks to see if special criteria are met every frame. If so, write a movement event. This
/// function effectively initiates the movement of all movable entities.
pub fn post_movement_event(
    movable_entities: Query<(Entity, &HexPos, &IsMovable, &PickSelection), Without<IsTraversable>>,
    mut movement_event: EventWriter<MovementEvent>,
    mut traversable_entities: Query<
        (&HexPos, &IsTraversable, &mut PickSelection),
        Without<IsMovable>,
    >,
) {
    // Iterate over all non-traversable entities that:
    //     - Could possibly be moved.
    //     - Could possibly be selected.
    for (entity, origin_position, is_movable, origin_pick_selection) in movable_entities.iter() {
        // If there is in fact an entity that is selected AND movable...
        if origin_pick_selection.is_selected && is_movable.truthful {
            // ...then iterate over all non-movable entities that:
            //     - Could possibly be traversed.
            //     - Count possibly be selected.
            for (destination_position, is_traversable, mut destination_pick_selection) in
                traversable_entities.iter_mut()
            {
                // If the destination is not the origin AND there is in fact an
                // entity that is selected AND that same entity is
                // traversable...
                if origin_position != destination_position
                    && destination_pick_selection.is_selected
                    && is_traversable.truthful
                {
                    // ...then all criteria have been met. Send a movement
                    // event and commence pathfinding.
                    movement_event.send(MovementEvent::new(
                        entity,
                        *origin_position,
                        *destination_position,
                    ));

                    // Deselect the destination position to prevent looping
                    // indefinitely.
                    destination_pick_selection.is_selected = false;
                }
            }
        }
    }
}
