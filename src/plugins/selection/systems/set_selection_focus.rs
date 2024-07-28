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

use crate::common::components::labels::Label;
use crate::common::resources::selection_focus::SelectionFocus;
use crate::common::systems::utils::get_ancestor;

// TODO: Can all of these be boiled down to a single generic? We're just
//       pivoting off of the label enum.

/// Sets the 'SelectionFocus' resource to the constituent pilgrim the player
/// just clicked on.
pub fn set_pilgrim_focus(
    listener: Listener<Pointer<Select>>,
    parents: Query<&Parent>,
    mut selectables: Query<&mut PickSelection>,
    mut selection_focus: ResMut<SelectionFocus>,
) {
    // Unselect the entity that is the subject of this event.
    selectables.get_mut(listener.target).unwrap().is_selected = false;

    // Determine this entity's ancestor (greatest parent), select it instead,
    // and set it as the focus.
    let ancestor: Entity = get_ancestor(&listener.target, &parents);
    selectables.get_mut(ancestor).unwrap().is_selected = true;
    selection_focus.set_focus(&Label::Pilgrim, &ancestor);
}

/// Sets the 'SelectionFocus' resource to the constituent city the player just
/// clicked on.
pub fn set_city_focus(
    listener: Listener<Pointer<Select>>,
    parents: Query<&Parent>,
    mut selectables: Query<&mut PickSelection>,
    mut selection_focus: ResMut<SelectionFocus>,
) {
    // Unselect the entity that is the subject of this event.
    selectables.get_mut(listener.target).unwrap().is_selected = false;

    // Determine this entity's ancestor (greatest parent), select it instead,
    // and set it as the focus.
    let ancestor: Entity = get_ancestor(&listener.target, &parents);
    selectables.get_mut(ancestor).unwrap().is_selected = true;
    selection_focus.set_focus(&Label::City, &ancestor);
}
