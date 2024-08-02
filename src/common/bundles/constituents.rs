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

use crate::common::components::combat::Health;
use crate::common::components::movement::HexPos;
use crate::common::components::movement::IsMovable;
use crate::common::components::movement::MovementBuffer;
use crate::plugins::city::components::markers::CityMarker;

#[derive(Bundle)]
pub struct City {
    marker: CityMarker,
    model: SceneBundle,
    pick_selection: PickSelection,
    pointer_deselect_callback: On<Pointer<Deselect>>,
    pointer_over_callback: On<Pointer<Over>>,
    pointer_select_callback: On<Pointer<Select>>,
    position: HexPos,
}

impl City {
    pub fn new(
        model: &SceneBundle,
        pointer_deselect_callback: On<Pointer<Deselect>>,
        pointer_over_callback: On<Pointer<Over>>,
        pointer_select_callback: On<Pointer<Select>>,
        position: &HexPos,
    ) -> Self {
        City {
            marker: CityMarker,
            model: model.clone(),
            pick_selection: PickSelection { is_selected: false },
            pointer_deselect_callback,
            pointer_over_callback,
            pointer_select_callback,
            position: *position,
        }
    }
}

#[derive(Bundle)]
pub struct Unit {
    health: Health,
    is_movable: IsMovable,
    model: SceneBundle,
    movement_buffer: MovementBuffer,
    pick_selection: PickSelection,
    pointer_deselect_callback: On<Pointer<Deselect>>,
    pointer_over_callback: On<Pointer<Over>>,
    pointer_select_callback: On<Pointer<Select>>,
    position: HexPos,
}

impl Unit {
    pub fn new(
        model: &SceneBundle,
        pointer_deselect_callback: On<Pointer<Deselect>>,
        pointer_over_callback: On<Pointer<Over>>,
        pointer_select_callback: On<Pointer<Select>>,
        position: &HexPos,
    ) -> Self {
        Unit {
            health: Health::new(),
            is_movable: IsMovable::new(),
            model: model.clone(),
            movement_buffer: MovementBuffer::new(Vec::new()),
            pick_selection: PickSelection { is_selected: false },
            pointer_deselect_callback,
            pointer_over_callback,
            pointer_select_callback,
            position: *position,
        }
    }
}
