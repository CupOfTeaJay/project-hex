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

use crate::common::components::movement::HexPos;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::systems::utils::hexpos_to_vec3;
use crate::plugins::city::components::heirarchy::Zones;
use crate::plugins::city::components::markers::CityCenterMarker;
use crate::plugins::selection::systems::clear_selection_focus::clear_selection_focus;
use crate::plugins::selection::systems::set_selection_focus::set_city_focus;

#[derive(Bundle)]
pub struct CityCenter {
    marker: CityCenterMarker,
    model: SceneBundle,
    pick_selection: PickSelection,
    pointer_deselect_callback: On<Pointer<Deselect>>,
    pointer_over_callback: On<Pointer<Over>>,
    pointer_select_callback: On<Pointer<Select>>,
    position: HexPos,
    zones: Zones,
}

impl CityCenter {
    pub fn new(assets: &Res<AssetHandles>, position: &HexPos) -> Self {
        CityCenter {
            marker: CityCenterMarker,
            model: SceneBundle {
                scene: assets.scenes.city_center.clone().unwrap(),
                transform: Transform::from_translation(hexpos_to_vec3(position)),
                ..default()
            },
            pick_selection: PickSelection { is_selected: false },
            pointer_deselect_callback: On::<Pointer<Deselect>>::run(clear_selection_focus),
            pointer_over_callback: On::<Pointer<Over>>::run(|| {}),
            pointer_select_callback: On::<Pointer<Select>>::run(set_city_focus),
            position: *position,
            zones: Zones::new(),
        }
    }
}
