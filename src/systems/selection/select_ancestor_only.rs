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

use crate::components::selection::label::Label;
use crate::resources::selection_focus::SelectionFocus;
use crate::utils::get_ancestor::get_ancestor;

pub fn select_ancestor_only(
    pointer_click_event: Listener<Pointer<Select>>,
    mut selectables: Query<&mut PickSelection>,
    parents: Query<&Parent>,
    labels: Query<&Label>,
    mut selection_focus: ResMut<SelectionFocus>,
) {
    // Unselect the entity that is the subject of this event.
    selectables
        .get_mut(pointer_click_event.target)
        .unwrap()
        .is_selected = false;

    // Determine and select this entity's ancestor.
    let ancestor: Entity = get_ancestor(&pointer_click_event.target, &parents);
    selectables.get_mut(ancestor).unwrap().is_selected = true;

    // TODO:.
    selection_focus.set_focus(&ancestor, labels.get(ancestor).unwrap());
}
